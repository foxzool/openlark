//! 单一 WebSocket 会话实现。
//!
//! 一次 `select!` 循环拥有：传输 I/O、心跳、控制帧、分包组装、事件派发调度与响应写回。
//! 用户 `EventHandler` 在 `spawn_blocking` 中执行，避免慢 handler 卡住 Ping/收帧。

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

/// 会话连接状态（#421 / #428：Session 拥有状态，非法操作经 Result 返回）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SessionState {
    /// 已连接，可处理帧。
    Active,
    /// 已关闭，不再接受业务帧。
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
    /// 当前 app-level ping 间隔（秒）；pong 只更新此值与 interval。
    ping_interval_secs: u64,
    ping_frame_interval: Interval,
    heartbeat_timeout: Duration,
    state: SessionState,
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
            ping_interval_secs: ping_secs,
            ping_frame_interval: tokio::time::interval(Duration::from_secs(ping_secs)),
            heartbeat_timeout: options.heartbeat_timeout.max(Duration::from_millis(1)),
            state: SessionState::Active,
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

    fn mark_closed(&mut self) -> WsClientResult<()> {
        if self.state == SessionState::Closed {
            return Err(WsClientError::InvalidStateTransition(
                "session already Closed".to_string(),
            ));
        }
        self.state = SessionState::Closed;
        Ok(())
    }

    /// 运行会话直至关闭或错误。
    ///
    /// 正常对端 Close / 传输失败 / 协议错误均以 `Err` 返回（含
    /// `ConnectionClosed`）。生产路径不返回 `Ok(())`。
    pub(crate) async fn run(mut self) -> WsClientResult<()> {
        // 存活计时：仅 WebSocket 层 Ping 刷新（保留历史 heartbeat 语义）。
        let mut last_activity = Instant::now();
        let mut checkout_timeout = tokio::time::interval(Duration::from_secs(1));
        // 数据帧 handler 在 blocking 池执行，完成后经此通道回写响应。
        let (handler_tx, mut handler_rx) = mpsc::unbounded_channel::<HandlerOutcome>();

        loop {
            tokio::select! {
                item = self.stream.next() => {
                    match item.transpose()? {
                        Some(msg) => {
                            if msg.is_ping() {
                                last_activity = Instant::now();
                            }
                            self.handle_message(msg, &handler_tx).await?;
                        }
                        None => {
                            self.mark_closed()?;
                            return Err(WsClientError::ConnectionClosed { reason: None });
                        }
                    }
                }
                // 慢 EventHandler 不阻塞 Ping / 收帧 / 关闭
                Some(outcome) = handler_rx.recv() => {
                    match outcome {
                        Ok(Some(response_frame)) => {
                            self.ensure_active()?;
                            self.send_frame(response_frame).await?;
                        }
                        Ok(None) => {}
                        Err(err) => {
                            let _ = self.mark_closed();
                            return Err(err);
                        }
                    }
                }
                _ = self.ping_frame_interval.tick() => {
                    self.send_app_ping().await?;
                }
                _ = checkout_timeout.tick() => {
                    if last_activity.elapsed() > self.heartbeat_timeout {
                        self.mark_closed()?;
                        return Err(WsClientError::ConnectionClosed { reason: None });
                    }
                }
            }
        }
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
        handler_tx: &mpsc::UnboundedSender<HandlerOutcome>,
    ) -> WsClientResult<()> {
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
                    FRAME_METHOD_DATA => self.schedule_data_frame(frame, handler_tx)?,
                    other => {
                        return Err(WsClientError::ClientError {
                            code: 0,
                            message: format!("invalid frame method: {other}"),
                        });
                    }
                }
            }
            Message::Close(close_frame) => {
                self.mark_closed()?;
                return Err(WsClientError::ConnectionClosed {
                    reason: close_frame.map(|frame| WsCloseReason {
                        code: frame.code,
                        message: frame.reason.to_string(),
                    }),
                });
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

    /// 仅应用 pong 中的 `PingInterval`（不缓存完整 ClientConfig）。
    fn apply_ping_interval(&mut self, ping_interval: i32) {
        let ping_secs = (ping_interval.max(1)) as u64;
        self.ping_interval_secs = ping_secs;
        self.ping_frame_interval = tokio::time::interval(Duration::from_secs(ping_secs));
        self.ping_frame_interval
            .reset_after(Duration::from_secs(ping_secs));
        debug!("Updated ping interval from pong response: {ping_secs}s");
    }

    /// 分包在主循环顺序完成；派发与 ACK 构造丢到 blocking 池，避免阻塞 select。
    fn schedule_data_frame(
        &mut self,
        frame: Frame,
        handler_tx: &mpsc::UnboundedSender<HandlerOutcome>,
    ) -> WsClientResult<()> {
        let Some(frame) = package::assemble_frame(&mut self.package_buffers, frame) else {
            return Ok(());
        };

        let event_handler = self.event_handler.clone();
        let tx = handler_tx.clone();
        tokio::task::spawn_blocking(move || {
            let outcome = match std::panic::catch_unwind(AssertUnwindSafe(|| {
                FrameHandler::handle_data_frame(frame, &event_handler)
            })) {
                Ok(opt) => Ok(opt),
                Err(_) => Err(WsClientError::ClientError {
                    code: 0,
                    message: "event handler panicked".to_string(),
                }),
            };
            if tx.send(outcome).is_err() {
                debug!("handler outcome dropped: session already ended");
            }
        });
        Ok(())
    }
}
