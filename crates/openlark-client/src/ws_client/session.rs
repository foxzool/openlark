//! 单一 WebSocket 会话实现。
//!
//! 一次 `select!` 循环拥有：传输 I/O、心跳、控制帧、分包组装、事件派发与响应写回。
//! 不再使用 I/O 任务 + handler 任务 + `WsEvent` 通道的双环路。

use std::collections::HashMap;
use std::time::Duration;

use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use lark_websocket_protobuf::pbbp2::Frame;
use log::{debug, error, trace};
use prost::Message as ProstMessage;
use tokio::net::TcpStream;
use tokio::time::{Instant, Interval};
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::protocol::Message,
};

use super::client::{ClientConfig, EventDispatcherHandler, WsClientError, WsClientResult, WsCloseReason};
use super::frame_handler::{
    ControlFrameEffect, ControlFrameError, FRAME_METHOD_CONTROL, FRAME_METHOD_DATA, FrameHandler,
};
use super::package::{self, FramePackageBuffer};

/// 会话运行选项（生产默认；测试可缩短心跳超时）。
#[derive(Debug, Clone)]
pub(crate) struct SessionOptions {
    /// 入站空闲超时。任意入站 WebSocket 消息（含 Binary/Ping）会刷新计时；
    /// 超时返回 `ConnectionClosed { reason: None }`。
    pub(crate) heartbeat_timeout: Duration,
}

impl Default for SessionOptions {
    fn default() -> Self {
        Self {
            heartbeat_timeout: Duration::from_secs(120),
        }
    }
}

/// 单次 WebSocket 会话：连接建立后的全部协议行为。
pub(crate) struct Session {
    service_id: i32,
    sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>,
    stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    event_handler: EventDispatcherHandler,
    package_buffers: HashMap<String, FramePackageBuffer>,
    client_config: ClientConfig,
    ping_frame_interval: Interval,
    heartbeat_timeout: Duration,
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
            client_config,
            heartbeat_timeout: options.heartbeat_timeout.max(Duration::from_millis(1)),
        }
    }

    /// 运行会话直至关闭或错误。
    ///
    /// 正常对端 Close / 传输失败 / 协议错误均以 `Err` 返回（含
    /// `ConnectionClosed`）。生产路径不返回 `Ok(())`。
    pub(crate) async fn run(mut self) -> WsClientResult<()> {
        // 存活计时：任意入站消息刷新（不仅是 WS 层 Ping）。
        // 这是有意语义：Binary 业务帧也证明链路存活。
        let mut last_activity = Instant::now();
        let mut checkout_timeout = tokio::time::interval(Duration::from_secs(1));

        loop {
            tokio::select! {
                item = self.stream.next() => {
                    match item.transpose()? {
                        Some(msg) => {
                            last_activity = Instant::now();
                            self.handle_message(msg).await?;
                        }
                        None => {
                            return Err(WsClientError::ConnectionClosed { reason: None });
                        }
                    }
                }
                _ = self.ping_frame_interval.tick() => {
                    self.send_app_ping().await?;
                }
                _ = checkout_timeout.tick() => {
                    if last_activity.elapsed() > self.heartbeat_timeout {
                        return Err(WsClientError::ConnectionClosed { reason: None });
                    }
                }
            }
        }
    }

    async fn send_app_ping(&mut self) -> WsClientResult<()> {
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

    async fn handle_message(&mut self, msg: WsMessage) -> WsClientResult<()> {
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
                    FRAME_METHOD_DATA => self.handle_data_frame(frame).await?,
                    other => {
                        return Err(WsClientError::ClientError {
                            code: 0,
                            message: format!("invalid frame method: {other}"),
                        });
                    }
                }
            }
            Message::Close(close_frame) => {
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
                self.apply_client_config(config);
                Ok(())
            }
            Ok(ControlFrameEffect::Ignored) => Ok(()),
            Err(ControlFrameError::MalformedPong(message)) => {
                Err(WsClientError::MalformedControlFrame { message })
            }
        }
    }

    fn apply_client_config(&mut self, config: ClientConfig) {
        let ping_interval = (config.ping_interval.max(1)) as u64;
        self.ping_frame_interval = tokio::time::interval(Duration::from_secs(ping_interval));
        self.ping_frame_interval
            .reset_after(Duration::from_secs(ping_interval));
        self.client_config = config;
        debug!("Updated ping interval from pong response: {ping_interval}s");
    }

    /// 分包 → 派发 → 同会话 sink 写回。
    async fn handle_data_frame(&mut self, frame: Frame) -> WsClientResult<()> {
        let Some(frame) = package::assemble_frame(&mut self.package_buffers, frame) else {
            return Ok(());
        };

        if let Some(response_frame) = FrameHandler::handle_data_frame(frame, &self.event_handler) {
            self.send_frame(response_frame).await?;
        }
        Ok(())
    }
}
