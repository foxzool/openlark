//! 单一 WebSocket 会话实现。
//!
//! 一次 `select!` 循环拥有：传输 I/O、心跳、控制帧、分包、**串行**事件派发调度与写回。
//! 用户 `EventHandler` 在专用 worker 中经 `spawn_blocking` 顺序执行，既不阻塞 Ping/收帧，
//! 又保持与旧 `handler_loop` 一致的串行调用与 ACK 顺序。

use std::collections::HashMap;
use std::panic::AssertUnwindSafe;
use std::time::Duration;

use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use lark_websocket_protobuf::pbbp2::Frame;
use log::{debug, error, trace};
use prost::Message as ProstMessage;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::{Instant, Interval};
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::protocol::Message};

use super::client::{
    ClientConfig, EventDispatcherHandler, WsClientError, WsClientResult, WsCloseReason,
};
use super::frame_handler::{
    ControlFrameEffect, ControlFrameError, FRAME_METHOD_CONTROL, FRAME_METHOD_DATA, FrameHandler,
};
use super::package::{self, FramePackageBuffer};

/// 会话连接状态（#421 / #428）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SessionState {
    /// 已连接，可处理业务帧。
    Active,
    /// 正在关闭：不再接收新业务帧，可排空已排队的 handler。
    Closing,
    /// 已关闭。
    Closed,
}

/// 会话运行选项（生产默认；测试可缩短心跳超时）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SessionOptions {
    /// 入站空闲超时。
    ///
    /// **仅** WebSocket 层 `Ping` 刷新存活计时（与历史行为一致）；
    /// Binary 业务帧不刷新。超时返回 `ConnectionClosed { reason: None }`。
    pub(crate) heartbeat_timeout: Duration,
}

impl Default for SessionOptions {
    fn default() -> Self {
        Self {
            heartbeat_timeout: Duration::from_secs(120),
        }
    }
}

/// handler 完成后回写主循环的结果。
type HandlerOutcome = WsClientResult<Option<Frame>>;

/// 单次 WebSocket 会话：连接建立后的全部协议行为。
pub(crate) struct Session {
    service_id: i32,
    sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>,
    stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    event_handler: EventDispatcherHandler,
    package_buffers: HashMap<String, FramePackageBuffer>,
    ping_frame_interval: Interval,
    heartbeat_timeout: Duration,
    state: SessionState,
    /// 关闭原因（Closing 时暂存，排空 handler 后返回）。
    pending_close: Option<Option<WsCloseReason>>,
    /// 已提交给串行 worker、尚未回写的任务数。
    inflight_handlers: usize,
}

impl Session {
    pub(crate) fn new(
        service_id: i32,
        client_config: ClientConfig,
        conn: WebSocketStream<MaybeTlsStream<TcpStream>>,
        event_handler: EventDispatcherHandler,
        options: SessionOptions,
    ) -> Self {
        let (sink, stream) = conn.split();
        let ping_secs = (client_config.ping_interval.max(1)) as u64;
        Self {
            service_id,
            sink,
            stream,
            event_handler,
            package_buffers: HashMap::new(),
            ping_frame_interval: tokio::time::interval(Duration::from_secs(ping_secs)),
            heartbeat_timeout: options.heartbeat_timeout.max(Duration::from_millis(1)),
            state: SessionState::Active,
            pending_close: None,
            inflight_handlers: 0,
        }
    }

    fn ensure_active(&self) -> WsClientResult<()> {
        if self.state != SessionState::Active {
            return Err(WsClientError::InvalidStateTransition(format!(
                "session is {:?}, expected Active",
                self.state
            )));
        }
        Ok(())
    }

    /// 进入 Closing：停止新业务，排空 inflight 后再结束。
    fn begin_close(&mut self, reason: Option<WsCloseReason>) -> WsClientResult<()> {
        match self.state {
            SessionState::Closed => {
                return Err(WsClientError::InvalidStateTransition(
                    "session already Closed".to_string(),
                ));
            }
            SessionState::Closing => {
                return Err(WsClientError::InvalidStateTransition(
                    "session already Closing".to_string(),
                ));
            }
            SessionState::Active => {}
        }
        self.state = SessionState::Closing;
        self.pending_close = Some(reason);
        Ok(())
    }

    fn finish_if_drained(&mut self) -> Option<WsClientResult<()>> {
        if self.state == SessionState::Closing && self.inflight_handlers == 0 {
            self.state = SessionState::Closed;
            let reason = self.pending_close.take().flatten();
            return Some(Err(WsClientError::ConnectionClosed { reason }));
        }
        None
    }

    /// 运行会话直至关闭或错误。
    ///
    /// 正常对端 Close / 传输失败 / 协议错误均以 `Err` 返回（含
    /// `ConnectionClosed`）。生产路径不返回 `Ok(())`。
    pub(crate) async fn run(mut self) -> WsClientResult<()> {
        // 存活计时：仅 WebSocket 层 Ping 刷新（保留历史 heartbeat 语义）。
        let mut last_activity = Instant::now();
        let mut checkout_timeout = tokio::time::interval(Duration::from_secs(1));

        // 串行 worker：FIFO 处理完整数据帧，保持 handler 调用与 ACK 顺序。
        let (job_tx, mut job_rx) = mpsc::unbounded_channel::<Frame>();
        let (outcome_tx, mut outcome_rx) = mpsc::unbounded_channel::<HandlerOutcome>();
        let worker_handler = self.event_handler.clone();
        let worker = tokio::spawn(async move {
            while let Some(frame) = job_rx.recv().await {
                let handler = worker_handler.clone();
                let outcome = tokio::task::spawn_blocking(move || {
                    match std::panic::catch_unwind(AssertUnwindSafe(|| {
                        FrameHandler::handle_data_frame(frame, &handler)
                    })) {
                        Ok(opt) => Ok(opt),
                        Err(_) => Err(WsClientError::ClientError {
                            code: 0,
                            message: "event handler panicked".to_string(),
                        }),
                    }
                })
                .await
                .unwrap_or_else(|e| {
                    Err(WsClientError::ClientError {
                        code: 0,
                        message: format!("handler task join error: {e}"),
                    })
                });
                if outcome_tx.send(outcome).is_err() {
                    break;
                }
            }
        });

        let result = async {
            loop {
                if let Some(done) = self.finish_if_drained() {
                    return done;
                }

                tokio::select! {
                    // Active 与 Closing 都继续读流：Closing 时再收 Binary 可触发可达的非法状态错误
                    item = self.stream.next(), if self.state != SessionState::Closed => {
                        match item.transpose()? {
                            Some(msg) => {
                                if msg.is_ping() {
                                    last_activity = Instant::now();
                                }
                                if self.state == SessionState::Closing {
                                    if matches!(msg, Message::Binary(_) | Message::Text(_)) {
                                        return Err(WsClientError::InvalidStateTransition(
                                            "received data frame while session is Closing"
                                                .to_string(),
                                        ));
                                    }
                                    // Closing 期间忽略多余 Ping/Pong/Close
                                    continue;
                                }
                                self.handle_message(msg, &job_tx).await?;
                            }
                            None => {
                                self.begin_close(None)?;
                            }
                        }
                    }
                    Some(outcome) = outcome_rx.recv() => {
                        self.inflight_handlers = self.inflight_handlers.saturating_sub(1);
                        match outcome {
                            Ok(Some(response_frame)) => {
                                // Closing 时丢弃 ACK，不再写回
                                if self.state == SessionState::Active {
                                    self.send_frame(response_frame).await?;
                                }
                            }
                            Ok(None) => {}
                            Err(err) => {
                                let _ = self.begin_close(None);
                                self.state = SessionState::Closed;
                                return Err(err);
                            }
                        }
                        if let Some(done) = self.finish_if_drained() {
                            return done;
                        }
                    }
                    _ = self.ping_frame_interval.tick(), if self.state == SessionState::Active => {
                        self.send_app_ping().await?;
                    }
                    _ = checkout_timeout.tick(), if self.state == SessionState::Active => {
                        if last_activity.elapsed() > self.heartbeat_timeout {
                            self.begin_close(None)?;
                        }
                    }
                }
            }
        }
        .await;

        // 停止 worker：丢弃 job 发送端后等任务结束（不取消已在执行的 handler）
        drop(job_tx);
        let _ = worker.await;
        result
    }

    async fn send_app_ping(&mut self) -> WsClientResult<()> {
        self.ensure_active()?;
        let frame = FrameHandler::build_ping_frame(self.service_id);
        let msg = Message::Binary(frame.encode_to_vec().into());
        trace!(
            "Sending ping message: {:?} {} {}",
            msg,
            msg.len(),
            self.service_id
        );
        self.sink.send(msg).await.map_err(|e| {
            error!("Failed to send ping message: {e:?}");
            WsClientError::WsError(Box::new(e))
        })?;
        Ok(())
    }

    async fn send_frame(&mut self, frame: Frame) -> WsClientResult<()> {
        trace!("send frame: {frame:?}");
        let msg = Message::Binary(frame.encode_to_vec().into());
        self.sink
            .send(msg)
            .await
            .map_err(|e| WsClientError::WsError(Box::new(e)))?;
        Ok(())
    }

    async fn handle_message(
        &mut self,
        msg: WsMessage,
        job_tx: &mpsc::UnboundedSender<Frame>,
    ) -> WsClientResult<()> {
        // Closing/Closed 下再收到业务 Binary 即为非法状态
        if self.state != SessionState::Active
            && matches!(msg, Message::Binary(_) | Message::Text(_))
        {
            return Err(WsClientError::InvalidStateTransition(format!(
                "received data while session is {:?}",
                self.state
            )));
        }

        self.ensure_active()?;
        match msg {
            Message::Ping(data) => {
                self.sink
                    .send(Message::Pong(data))
                    .await
                    .map_err(|e| WsClientError::WsError(Box::new(e)))?;
            }
            Message::Binary(data) => {
                let frame = Frame::decode(&*data)?;
                trace!("Received frame: {frame:?}");
                match frame.method {
                    FRAME_METHOD_CONTROL => self.apply_control_frame(frame)?,
                    FRAME_METHOD_DATA => self.enqueue_data_frame(frame, job_tx)?,
                    other => {
                        return Err(WsClientError::ClientError {
                            code: 0,
                            message: format!("invalid frame method: {other}"),
                        });
                    }
                }
            }
            Message::Close(close_frame) => {
                let reason = close_frame.map(|frame| WsCloseReason {
                    code: frame.code,
                    message: frame.reason.to_string(),
                });
                self.begin_close(reason)?;
            }
            _ => return Err(WsClientError::UnexpectedResponse),
        }
        Ok(())
    }

    fn apply_control_frame(&mut self, frame: Frame) -> WsClientResult<()> {
        match FrameHandler::interpret_control_frame(&frame) {
            Ok(ControlFrameEffect::UpdateClientConfig(config)) => {
                self.apply_ping_interval(config.ping_interval);
                Ok(())
            }
            Ok(ControlFrameEffect::Ignored) => Ok(()),
            Err(ControlFrameError::MalformedPong(message)) => {
                Err(WsClientError::MalformedControlFrame { message })
            }
        }
    }

    /// 仅应用 pong 中的 `PingInterval`。
    fn apply_ping_interval(&mut self, ping_interval: i32) {
        let ping_secs = (ping_interval.max(1)) as u64;
        self.ping_frame_interval = tokio::time::interval(Duration::from_secs(ping_secs));
        self.ping_frame_interval
            .reset_after(Duration::from_secs(ping_secs));
        debug!("Updated ping interval from pong response: {ping_secs}s");
    }

    /// 分包在主循环顺序完成；完整帧入队串行 worker。
    fn enqueue_data_frame(
        &mut self,
        frame: Frame,
        job_tx: &mpsc::UnboundedSender<Frame>,
    ) -> WsClientResult<()> {
        self.ensure_active()?;
        let Some(frame) = package::assemble_frame(&mut self.package_buffers, frame) else {
            return Ok(());
        };

        job_tx.send(frame).map_err(|_| {
            WsClientError::InvalidStateTransition("handler worker is gone".to_string())
        })?;
        self.inflight_handlers = self.inflight_handlers.saturating_add(1);
        Ok(())
    }
}
