//! 单一 WebSocket 会话实现。
//!
//! 一次 `select!` 循环拥有：传输 I/O、心跳、控制帧、分包、**串行**事件派发调度与写回。
//! 用户 `EventHandler` 在专用 worker 中经 `spawn_blocking` 顺序执行，保持与旧
//! `handler_loop` 一致的串行调用与 ACK 顺序。
//!
//! 入队使用 **try_send + 本地 outbox + `reserve()`**：队列满时不丢弃已组装帧，
//! 也不在 select 关键路径上 `send().await` 阻塞 Ping/心跳。
//! （outbox 与 worker 队列均有界；两者皆满时才返回错误，避免无界内存。）

mod types;

use std::collections::{HashMap, VecDeque};
use std::panic::AssertUnwindSafe;
use std::time::Duration;

use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use lark_websocket_protobuf::pbbp2::Frame;
use log::{debug, error, trace, warn};
use prost::Message as ProstMessage;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc;
use tokio::time::{Instant, Interval};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;

use super::client::ClientConfig;
use super::dispatcher::EventDispatcherHandler;
use super::frame_handler::{
    ControlFrameEffect, ControlFrameError, FRAME_METHOD_CONTROL, FRAME_METHOD_DATA, FrameHandler,
};
use super::package::{self, FramePackageBuffer};

pub use types::{InvalidStateKind, WsClientError, WsClientResult, WsCloseReason};

/// 串行 handler 队列容量（worker 侧有界缓冲）。
const HANDLER_QUEUE_CAP: usize = 64;
/// 主循环本地待发 outbox 上限（与队列同量级，内存有界且不丢帧）。
const PENDING_OUTBOX_CAP: usize = HANDLER_QUEUE_CAP;
/// 会话结束后等待 worker 排空的最长时间。
const WORKER_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

/// 会话连接状态（#421 / #428 / #641）。
///
/// 真实状态空间是 Active / Closing（排空中，可选远端 reason）/ Closed。
/// reason 挂在 `Closing` 上，避免与独立 `CloseIntent` 手工对齐。
#[derive(Debug, Clone)]
enum SessionState {
    Active,
    /// 排空中。`Some` = 对端 Close 帧带来的 code/message，后续错误不得覆盖。
    Closing(Option<WsCloseReason>),
    Closed,
}

impl SessionState {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Closing(_) => "Closing",
            Self::Closed => "Closed",
        }
    }

    fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }
}

/// `try_send` 到 worker 的局部结果（Full 装箱以避免 result_large_err）。
enum TrySendToWorker {
    Full(Box<Frame>),
    Closed,
}

/// 会话运行选项（生产默认；测试可缩短心跳超时）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SessionOptions {
    /// **仅** WebSocket 层 `Ping` 刷新存活计时；超时返回 `ConnectionClosed { reason: None }`。
    pub(crate) heartbeat_timeout: Duration,
}

impl Default for SessionOptions {
    fn default() -> Self {
        Self {
            heartbeat_timeout: Duration::from_secs(120),
        }
    }
}

type HandlerOutcome = WsClientResult<Option<Frame>>;

/// 单一 WebSocket 会话（泛型于底层字节流，ADR-0006）。
///
/// 生产实例化 `S = MaybeTlsStream<TcpStream>`（`connect_async`）；测试用内存
/// 双工流 + `from_raw_socket` 适配（tungstenite 成帧语义完整保留）。对 I/O 的
/// 全部使用面仅 `stream.next()` 与 `sink.send()`（send 内含 flush）。
pub(crate) struct Session<S> {
    service_id: i32,
    sink: SplitSink<WebSocketStream<S>, WsMessage>,
    stream: SplitStream<WebSocketStream<S>>,
    event_handler: EventDispatcherHandler,
    package_buffers: HashMap<String, FramePackageBuffer>,
    ping_frame_interval: Interval,
    heartbeat_timeout: Duration,
    state: SessionState,
    inflight_handlers: usize,
    /// worker 队列满时暂存的已组装帧（不丢弃；经 reserve 再入队）。
    pending_outbox: VecDeque<Frame>,
}

impl<S> Session<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    pub(crate) fn new(
        service_id: i32,
        client_config: ClientConfig,
        conn: WebSocketStream<S>,
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
            inflight_handlers: 0,
            pending_outbox: VecDeque::new(),
        }
    }

    fn ensure_active(&self) -> WsClientResult<()> {
        if !self.state.is_active() {
            return Err(WsClientError::InvalidStateTransition {
                kind: InvalidStateKind::ExpectedActive {
                    actual: self.state.as_str(),
                },
            });
        }
        Ok(())
    }

    fn begin_close(&mut self, reason: Option<WsCloseReason>) -> WsClientResult<()> {
        match &self.state {
            SessionState::Closed => {
                return Err(WsClientError::InvalidStateTransition {
                    kind: InvalidStateKind::AlreadyClosed,
                });
            }
            // 幂等：保留首次 close 意图（远端 close reason 不被 EOF 覆盖）
            SessionState::Closing(_) => return Ok(()),
            SessionState::Active => {}
        }
        self.state = SessionState::Closing(reason);
        Ok(())
    }

    /// 后续错误不得覆盖已记录的远端 close reason（#421 US9）。
    /// 无已记录 reason 时返回 `fallback`；两种错误入口共用。
    fn fail_preserving_close_reason(
        &mut self,
        subsequent: impl std::fmt::Display,
        fallback: WsClientError,
    ) -> WsClientError {
        let recorded = match std::mem::replace(&mut self.state, SessionState::Closed) {
            SessionState::Closing(reason) => reason,
            _ => None,
        };
        match recorded {
            Some(reason) => {
                warn!("{subsequent}; returning close reason");
                WsClientError::ConnectionClosed {
                    reason: Some(reason),
                }
            }
            None => fallback,
        }
    }

    /// 若已进入 Closing 且 in-flight/outbox 均空，返回终态 `ConnectionClosed`。
    /// `None` 表示会话尚未结束，应继续 `select!`。
    fn take_connection_closed_if_idle(&mut self) -> Option<WsClientResult<()>> {
        if matches!(self.state, SessionState::Closing(_))
            && self.inflight_handlers == 0
            && self.pending_outbox.is_empty()
        {
            let reason = match std::mem::replace(&mut self.state, SessionState::Closed) {
                SessionState::Closing(reason) => reason,
                other => {
                    self.state = other;
                    return None;
                }
            };
            return Some(Err(WsClientError::ConnectionClosed { reason }));
        }
        None
    }

    fn note_job_enqueued(&mut self) {
        self.inflight_handlers = self.inflight_handlers.saturating_add(1);
    }

    /// 非阻塞将一帧交给 worker；满则退回调用方决定（outbox / 错误）。
    fn try_send_to_worker(
        &mut self,
        job_tx: &mpsc::Sender<Frame>,
        frame: Frame,
    ) -> Result<(), TrySendToWorker> {
        match job_tx.try_send(frame) {
            Ok(()) => {
                self.note_job_enqueued();
                Ok(())
            }
            Err(mpsc::error::TrySendError::Full(frame)) => {
                Err(TrySendToWorker::Full(Box::new(frame)))
            }
            Err(mpsc::error::TrySendError::Closed(_)) => Err(TrySendToWorker::Closed),
        }
    }

    fn flush_outbox_try(&mut self, job_tx: &mpsc::Sender<Frame>) -> WsClientResult<()> {
        while let Some(frame) = self.pending_outbox.pop_front() {
            match self.try_send_to_worker(job_tx, frame) {
                Ok(()) => {}
                Err(TrySendToWorker::Full(frame)) => {
                    self.pending_outbox.push_front(*frame);
                    break;
                }
                Err(TrySendToWorker::Closed) => {
                    return Err(WsClientError::InvalidStateTransition {
                        kind: InvalidStateKind::WorkerGone,
                    });
                }
            }
        }
        Ok(())
    }

    pub(crate) async fn run(mut self) -> WsClientResult<()> {
        let mut last_activity = Instant::now();
        // 心跳过期轮询周期（非业务 checkout）；钳在 [50ms, 1s] 与 timeout 之间。
        let heartbeat_check_period = self
            .heartbeat_timeout
            .min(Duration::from_secs(1))
            .max(Duration::from_millis(50));
        let mut heartbeat_check_interval = tokio::time::interval(heartbeat_check_period);

        let (job_tx, mut job_rx) = mpsc::channel::<Frame>(HANDLER_QUEUE_CAP);
        let (outcome_tx, mut outcome_rx) = mpsc::channel::<HandlerOutcome>(HANDLER_QUEUE_CAP);
        let worker_handler = self.event_handler.clone();
        let worker = tokio::spawn(async move {
            while let Some(frame) = job_rx.recv().await {
                let handler = worker_handler.clone();
                let outcome = tokio::task::spawn_blocking(move || {
                    match std::panic::catch_unwind(AssertUnwindSafe(|| {
                        FrameHandler::handle_data_frame(frame, &handler)
                    })) {
                        Ok(opt) => Ok(opt),
                        Err(_) => Err(WsClientError::HandlerPanicked),
                    }
                })
                .await
                .unwrap_or_else(|_| Err(WsClientError::HandlerPanicked));
                if outcome_tx.send(outcome).await.is_err() {
                    break;
                }
            }
        });

        let result = async {
            let mut stream_open = true;
            loop {
                if let Some(done) = self.take_connection_closed_if_idle() {
                    return done;
                }
                self.flush_outbox_try(&job_tx)?;

                // Closing 时也要冲刷 outbox，否则 idle 检查永远等非空 outbox
                let need_reserve = !self.pending_outbox.is_empty() && !self.state.is_closed();

                tokio::select! {
                    permit = job_tx.reserve(), if need_reserve => {
                        let permit = permit.map_err(|_| {
                            WsClientError::InvalidStateTransition {
                                kind: InvalidStateKind::WorkerGone,
                            }
                        })?;
                        if let Some(frame) = self.pending_outbox.pop_front() {
                            permit.send(frame);
                            self.note_job_enqueued();
                        }
                    }
                    item = self.stream.next(), if stream_open && !self.state.is_closed() => {
                        match item.transpose() {
                            Ok(Some(msg)) => {
                                if msg.is_ping() {
                                    last_activity = Instant::now();
                                }
                                if matches!(self.state, SessionState::Closing(_)) {
                                    if matches!(msg, Message::Binary(_) | Message::Text(_)) {
                                        return Err(WsClientError::InvalidStateTransition {
                                            kind: InvalidStateKind::DataWhileClosing,
                                        });
                                    }
                                    continue;
                                }
                                self.handle_message(msg, &job_tx).await?;
                            }
                            Ok(None) => {
                                stream_open = false;
                                self.begin_close(None)?;
                            }
                            Err(e) => {
                                let msg = format!("stream error after remote close: {e}");
                                return Err(self.fail_preserving_close_reason(msg, e.into()));
                            }
                        }
                    }
                    Some(outcome) = outcome_rx.recv() => {
                        self.inflight_handlers = self.inflight_handlers.saturating_sub(1);
                        match outcome {
                            Ok(Some(response_frame)) => {
                                if self.state.is_active() {
                                    self.send_frame(response_frame).await?;
                                }
                            }
                            Ok(None) => {}
                            Err(err) => {
                                let msg = format!("handler failed during close: {err}");
                                return Err(self.fail_preserving_close_reason(msg, err));
                            }
                        }
                        self.flush_outbox_try(&job_tx)?;
                        if let Some(done) = self.take_connection_closed_if_idle() {
                            return done;
                        }
                    }
                    _ = self.ping_frame_interval.tick(), if self.state.is_active() => {
                        self.send_app_ping().await?;
                    }
                    _ = heartbeat_check_interval.tick(), if self.state.is_active() => {
                        if last_activity.elapsed() > self.heartbeat_timeout {
                            self.begin_close(None)?;
                        }
                    }
                }
            }
        }
        .await;

        drop(job_tx);
        let mut worker = worker;
        tokio::select! {
            _ = &mut worker => {}
            _ = tokio::time::sleep(WORKER_SHUTDOWN_TIMEOUT) => {
                warn!(
                    "handler worker did not finish within {:?}; aborting worker task \
                     (in-flight spawn_blocking handler may still run until it returns)",
                    WORKER_SHUTDOWN_TIMEOUT
                );
                worker.abort();
                let _ = worker.await;
            }
        }
        result
    }

    async fn send_app_ping(&mut self) -> WsClientResult<()> {
        self.ensure_active()?;
        let frame = FrameHandler::build_ping_frame(self.service_id);
        let msg = Message::Binary(frame.encode_to_vec().into());
        if let Err(e) = self.sink.send(msg).await {
            error!("Failed to send ping message: {e:?}");
            return Err(e.into());
        }
        Ok(())
    }

    async fn send_frame(&mut self, frame: Frame) -> WsClientResult<()> {
        let msg = Message::Binary(frame.encode_to_vec().into());
        self.sink.send(msg).await?;
        Ok(())
    }

    async fn handle_message(
        &mut self,
        msg: WsMessage,
        job_tx: &mpsc::Sender<Frame>,
    ) -> WsClientResult<()> {
        // 仅在 Active 时由 run 循环调用；Closing 业务帧在 select 分支直接拒绝
        self.ensure_active()?;
        match msg {
            Message::Ping(data) => {
                self.sink.send(Message::Pong(data)).await?;
            }
            Message::Binary(data) => {
                let frame = Frame::decode(&*data)?;
                trace!("Received frame: {frame:?}");
                match frame.method {
                    FRAME_METHOD_CONTROL => self.apply_control_frame(frame)?,
                    FRAME_METHOD_DATA => self.enqueue_data_frame(frame, job_tx)?,
                    method => {
                        return Err(WsClientError::InvalidFrameMethod { method });
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
            Ok(ControlFrameEffect::UpdatePingInterval(secs)) => {
                self.apply_ping_interval(secs);
                Ok(())
            }
            Ok(ControlFrameEffect::Ignored) => Ok(()),
            Err(ControlFrameError::MalformedPong(message)) => {
                Err(WsClientError::MalformedControlFrame { message })
            }
        }
    }

    fn apply_ping_interval(&mut self, ping_interval: i32) {
        let ping_secs = (ping_interval.max(1)) as u64;
        self.ping_frame_interval = tokio::time::interval(Duration::from_secs(ping_secs));
        self.ping_frame_interval
            .reset_after(Duration::from_secs(ping_secs));
        debug!("Updated ping interval from pong response: {ping_secs}s");
    }

    /// try_send 入队；满则进 outbox（不丢帧）；outbox 也满才 Err。
    fn enqueue_data_frame(
        &mut self,
        frame: Frame,
        job_tx: &mpsc::Sender<Frame>,
    ) -> WsClientResult<()> {
        self.ensure_active()?;
        let Some(frame) = package::assemble_frame(&mut self.package_buffers, frame) else {
            return Ok(());
        };

        match self.try_send_to_worker(job_tx, frame) {
            Ok(()) => Ok(()),
            Err(TrySendToWorker::Full(frame)) => {
                if self.pending_outbox.len() >= PENDING_OUTBOX_CAP {
                    return Err(WsClientError::BacklogFull {
                        message: format!("queue {HANDLER_QUEUE_CAP} + outbox {PENDING_OUTBOX_CAP}"),
                    });
                }
                self.pending_outbox.push_back(*frame);
                Ok(())
            }
            Err(TrySendToWorker::Closed) => Err(WsClientError::InvalidStateTransition {
                kind: InvalidStateKind::WorkerGone,
            }),
        }
    }
}
