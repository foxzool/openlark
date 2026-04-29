use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use lark_websocket_protobuf::pbbp2::{Frame, Header};
use log::{debug, error, info, trace};
use prost::Message as ProstMessage;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use tokio::{net::TcpStream, sync::mpsc, time::Instant, time::Interval};
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async_with_config,
    tungstenite::protocol::{Message, frame::coding::CloseCode},
};
use url::Url;

use super::{FrameHandler, WebSocketStateMachine, state_machine::StateMachineEvent};

type EventHandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

/// WebSocket endpoint API 专用响应结构（顶层 code/msg/data）
#[derive(Debug, Deserialize)]
struct WsEndpointApiResponse<T> {
    #[serde(default)]
    code: i32,
    #[serde(default)]
    msg: String,
    data: Option<T>,
}

fn map_ws_api_error(code: i32, message: String) -> WsClientError {
    match code {
        1 | 1000040343 => WsClientError::ServerError { code, message },
        _ => WsClientError::ClientError { code, message },
    }
}

fn extract_endpoint_response(
    resp: WsEndpointApiResponse<EndPointResponse>,
) -> WsClientResult<EndPointResponse> {
    if resp.code != 0 {
        return Err(map_ws_api_error(resp.code, resp.msg));
    }

    let end_point = resp.data.ok_or(WsClientError::UnexpectedResponse)?;
    if end_point.url.as_ref().is_none_or(|url| url.is_empty()) {
        return Err(WsClientError::ServerError {
            code: 500,
            message: "No available endpoint".to_string(),
        });
    }

    Ok(end_point)
}

/// 分包消息缓存（按 message_id 聚合）
#[derive(Debug, Default)]
struct FramePackageBuffer {
    sum: usize,
    parts: Vec<Option<Vec<u8>>>,
    received: usize,
}

impl FramePackageBuffer {
    fn new(sum: usize) -> Self {
        Self {
            sum,
            parts: vec![None; sum],
            received: 0,
        }
    }

    fn insert_part(&mut self, seq: usize, payload: Vec<u8>) {
        if seq >= self.sum {
            return;
        }

        if self.parts[seq].is_none() {
            self.received = self.received.saturating_add(1);
        }
        self.parts[seq] = Some(payload);
    }

    fn is_complete(&self) -> bool {
        self.sum > 0 && self.received == self.sum && self.parts.iter().all(|p| p.is_some())
    }

    fn combine(self) -> Vec<u8> {
        let total_len: usize = self
            .parts
            .iter()
            .filter_map(|p| p.as_ref().map(|v| v.len()))
            .sum();
        let mut out = Vec::with_capacity(total_len);
        for part in self.parts.into_iter().flatten() {
            out.extend_from_slice(&part);
        }
        out
    }
}

#[derive(Debug, Deserialize)]
struct RawEventEnvelope {
    header: RawEventHeader,
}

#[derive(Debug, Deserialize)]
struct RawEventHeader {
    #[serde(default)]
    event_type: String,
}

/// 原始事件处理器。
///
/// 当调用方希望直接消费 WebSocket 原始事件负载时，可以实现该 trait，
/// 再通过 [`EventDispatcherHandler::register_raw`] 注册：
///
/// - key=`"raw"`：接收所有原始事件负载
/// - key=`"<event_type>"`：仅接收指定 `header.event_type` 的事件
pub trait EventHandler: Send + Sync + 'static {
    /// 处理原始事件负载。
    fn handle(&self, payload: &[u8]) -> EventHandlerResult;
}

/// WebSocket 事件分发处理器。
///
/// 目前支持两类分发目标：
///
/// - `payload_sender(...)`：把原始负载转发到 channel
/// - `register_raw(...)`：注册原始事件处理器
#[derive(Clone)]
pub struct EventDispatcherHandler {
    payload_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,
    raw_handlers: HashMap<String, Arc<dyn EventHandler>>,
}

impl std::fmt::Debug for EventDispatcherHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventDispatcherHandler")
            .field(
                "payload_tx",
                &self.payload_tx.as_ref().map(|_| "configured"),
            )
            .field(
                "raw_handler_keys",
                &self.raw_handlers.keys().collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl EventDispatcherHandler {
    /// 通配原始事件处理器 key。
    pub const RAW_EVENT_KEY: &'static str = "raw";

    /// 创建新的事件分发构建器。
    pub fn builder() -> Self {
        Self {
            payload_tx: None,
            raw_handlers: HashMap::new(),
        }
    }

    /// 完成构建。
    pub fn build(self) -> Self {
        self
    }

    /// 配置 channel 转发器，用于把原始负载发往外部任务。
    pub fn payload_sender(mut self, payload_tx: mpsc::UnboundedSender<Vec<u8>>) -> Self {
        self.payload_tx = Some(payload_tx);
        self
    }

    /// 注册原始事件处理器。
    ///
    /// - 传入 `"raw"` 会接收所有原始事件负载
    /// - 传入具体 `event_type`（例如 `"im.message.receive_v1"`）只会接收匹配事件
    pub fn register_raw<S, H>(mut self, key: S, handler: H) -> Result<Self, String>
    where
        S: Into<String>,
        H: EventHandler,
    {
        let key = key.into();
        if key.trim().is_empty() {
            return Err("processor key cannot be empty".to_string());
        }
        if self.raw_handlers.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        self.raw_handlers.insert(key, Arc::new(handler));
        Ok(self)
    }

    fn extract_event_type(payload: &[u8]) -> Option<String> {
        serde_json::from_slice::<RawEventEnvelope>(payload)
            .ok()
            .map(|event| event.header.event_type)
            .filter(|event_type| !event_type.trim().is_empty())
    }

    fn dispatch_raw_handler(&self, key: &str, payload: &[u8]) -> Result<(), String> {
        if let Some(handler) = self.raw_handlers.get(key) {
            handler
                .handle(payload)
                .map_err(|err| format!("处理原始事件 {key} 失败: {err}"))?;
        }
        Ok(())
    }

    /// 在不做 schema 校验的前提下分发原始负载。
    pub fn do_without_validation(&self, payload: &[u8]) -> Result<(), String> {
        if let Some(payload_tx) = &self.payload_tx {
            payload_tx
                .send(payload.to_vec())
                .map_err(|e| format!("转发事件负载失败: {e}"))?;
        }

        if let Some(event_type) = Self::extract_event_type(payload) {
            self.dispatch_raw_handler(&event_type, payload)?;
        }

        self.dispatch_raw_handler(Self::RAW_EVENT_KEY, payload)?;

        Ok(())
    }
}

const END_POINT_URL: &str = "/callback/ws/endpoint";
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(120);

/// 飞书 WebSocket 客户端。
pub struct LarkWsClient {
    frame_tx: mpsc::UnboundedSender<Frame>,
    event_rx: mpsc::UnboundedReceiver<WsEvent>,
    state_machine: WebSocketStateMachine,
    package_buffers: HashMap<String, FramePackageBuffer>,
}

impl LarkWsClient {
    #[cfg(test)]
    /// 创建一个仅用于测试的客户端实例。
    pub fn new_for_test() -> Self {
        let (frame_tx, _frame_rx) = mpsc::unbounded_channel();
        let (_event_tx, event_rx) = mpsc::unbounded_channel();

        Self {
            frame_tx,
            event_rx,
            state_machine: WebSocketStateMachine::new(),
            package_buffers: HashMap::new(),
        }
    }

    /// 建立 WebSocket 长连接并启动事件处理循环。
    pub async fn open(
        config: std::sync::Arc<crate::config::Config>,
        event_handler: EventDispatcherHandler,
    ) -> WsClientResult<()> {
        let end_point = Self::get_conn_url(&config).await?;
        let conn_url = end_point.url.ok_or(WsClientError::UnexpectedResponse)?;
        let client_config = end_point
            .client_config
            .ok_or(WsClientError::UnexpectedResponse)?;
        let url = Url::parse(&conn_url)?;
        let query_pairs: HashMap<_, _> = url.query_pairs().into_iter().collect();
        // let conn_id = query_pairs.get("device_id").unwrap();
        let service_id = query_pairs
            .get("service_id")
            .ok_or(WsClientError::UnexpectedResponse)?
            .parse()
            .map_err(|_| WsClientError::UnexpectedResponse)?;

        // 开始连接状态转换
        let mut state_machine = WebSocketStateMachine::new();
        if let Err(e) = state_machine.handle_event(StateMachineEvent::StartConnection) {
            error!("Failed to transition to connecting state: {e}");
        }

        let ws_config = tokio_tungstenite::tungstenite::protocol::WebSocketConfig {
            max_message_size: Some(config.max_response_size as usize),
            max_frame_size: Some(config.max_response_size as usize),
            ..Default::default()
        };

        let (conn, _response) = connect_async_with_config(conn_url, Some(ws_config), false).await?;
        info!("connected to {url}");

        // 连接成功状态转换
        if let Err(e) = state_machine.handle_event(StateMachineEvent::ConnectionEstablished) {
            error!("Failed to transition to connected state: {e}");
        }
        let (frame_tx, frame_rx) = mpsc::unbounded_channel();
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        tokio::spawn(client_loop(
            service_id,
            client_config,
            conn,
            frame_rx,
            event_tx,
        ));
        let mut client = LarkWsClient {
            frame_tx,
            event_rx,
            state_machine,
            package_buffers: HashMap::new(),
        };

        client.handler_loop(event_handler).await;

        Ok(())
    }

    async fn handler_loop(&mut self, event_handler: EventDispatcherHandler) {
        while let Some(ws_event) = self.event_rx.recv().await {
            if let WsEvent::Data(frame) = ws_event {
                // 更新状态机：收到数据
                if let Err(e) = self
                    .state_machine
                    .handle_event(StateMachineEvent::DataReceived)
                {
                    error!("Failed to handle DataReceived event: {e}");
                }

                // 检查是否可以处理数据
                if !self.state_machine.can_send_data() {
                    debug!(
                        "Cannot process data in current state: {:?}",
                        self.state_machine.current_state()
                    );
                    continue;
                }

                // 处理分包逻辑
                let processed_frame = self.process_frame_packages_internal(frame).await;
                let Some(frame) = processed_frame else {
                    continue;
                };

                // 使用 FrameHandler 处理帧
                // 创建一个临时的事件发送器，因为FrameHandler需要WsEvent类型
                let (temp_tx, mut temp_rx) = mpsc::unbounded_channel::<WsEvent>();
                if let Some(response_frame) =
                    FrameHandler::handle_frame(frame, &event_handler, &temp_tx).await
                    && let Err(e) = self.frame_tx.send(response_frame)
                {
                    error!("Failed to send response frame: {e:?}");
                }

                // 处理临时接收器中的任何事件 - 目前FrameHandler不会发送事件到这里，保留以备未来使用
                while let Ok(_ws_event) = temp_rx.try_recv() {
                    // 暂时忽略，因为FrameHandler主要处理控制帧
                }
            }
        }
    }

    /// 处理分包的 Frame，如果需要组合多个包则返回组合后的结果
    #[cfg(test)]
    pub async fn process_frame_packages(&mut self, frame: Frame) -> Option<Frame> {
        self.process_frame_packages_internal(frame).await
    }

    /// 内部处理分包的 Frame，如果需要组合多个包则返回组合后的结果
    async fn process_frame_packages_internal(&mut self, mut frame: Frame) -> Option<Frame> {
        let headers: &[Header] = frame.headers.as_ref();

        fn header_value<'a>(headers: &'a [Header], key: &str) -> Option<&'a str> {
            headers
                .iter()
                .find(|h| h.key == key)
                .map(|h| h.value.as_str())
        }

        fn header_usize(headers: &[Header], key: &str) -> Option<usize> {
            header_value(headers, key).and_then(|v| v.parse().ok())
        }

        // 拆包数，未拆包为1
        let sum: usize = headers
            .iter()
            .find(|h| h.key == "sum")
            .and_then(|h| h.value.parse().ok())
            .unwrap_or(1);

        // 包序号，未拆包为0
        let seq: usize = header_usize(headers, "seq").unwrap_or(0);

        // 消息ID，拆包后继承
        let msg_id: &str = header_value(headers, "message_id").unwrap_or("");

        let Some(payload) = frame.payload.take() else {
            error!("Frame payload is empty");
            return None;
        };

        let sum = if sum == 0 { 1 } else { sum };

        // 单包：直接透传
        if sum == 1 {
            frame.payload = Some(payload);
            return Some(frame);
        }

        // 分包：需要 message_id 作为聚合键；缺失则降级为单包透传，避免丢消息
        if msg_id.is_empty() {
            debug!(
                "收到分包帧但 message_id 为空，无法聚合，降级为单包处理（sum={sum}, seq={seq}）"
            );
            frame.payload = Some(payload);
            return Some(frame);
        }

        if seq >= sum {
            debug!(
                "收到分包帧但 seq 越界，降级为单包处理（sum={sum}, seq={seq}, message_id={msg_id}）"
            );
            frame.payload = Some(payload);
            return Some(frame);
        }

        // 获取/初始化缓存；sum 不一致时以最新为准重置（防止错配）
        let buffer = self
            .package_buffers
            .entry(msg_id.to_string())
            .or_insert_with(|| {
                debug!("开始聚合分包消息（sum={sum}, message_id={msg_id}）");
                FramePackageBuffer::new(sum)
            });

        if buffer.sum != sum {
            debug!(
                "分包聚合参数变化，重置缓存（old_sum={}, new_sum={}, message_id={msg_id}）",
                buffer.sum, sum
            );
            *buffer = FramePackageBuffer::new(sum);
        }

        buffer.insert_part(seq, payload);

        if !buffer.is_complete() {
            return None;
        }

        let buffer = self
            .package_buffers
            .remove(msg_id)
            .unwrap_or_else(|| FramePackageBuffer::new(sum));

        frame.payload = Some(buffer.combine());
        Some(frame)
    }

    /// 获取连接配置
    async fn get_conn_url(
        config: &std::sync::Arc<crate::config::Config>,
    ) -> WsClientResult<EndPointResponse> {
        let body = json!({
            "AppID": &config.app_id,
            "AppSecret": &config.app_secret
        });

        let http_client = Client::builder().timeout(config.timeout).build()?;

        let base_url = config.base_url.trim_end_matches('/');
        let req = http_client
            .post(format!("{base_url}{END_POINT_URL}"))
            .header("locale", "zh")
            .json(&body)
            .send()
            .await?;

        let resp = req
            .json::<WsEndpointApiResponse<EndPointResponse>>()
            .await?;
        debug!("{:?}", resp.data);

        extract_endpoint_response(resp)
    }
}

/// WebSocket 端点查询响应。
#[derive(Debug, Deserialize)]
pub struct EndPointResponse {
    #[serde(rename = "URL")]
    /// WebSocket 连接地址。
    pub url: Option<String>,
    #[serde(rename = "ClientConfig")]
    /// 服务端下发的客户端配置。
    pub client_config: Option<ClientConfig>,
}

/// 服务端下发的 WebSocket 客户端配置。
#[derive(Debug, Deserialize, Clone)]
pub struct ClientConfig {
    #[serde(rename = "ReconnectCount")]
    /// 允许的最大重连次数。
    pub reconnect_count: i32,
    #[serde(rename = "ReconnectInterval")]
    /// 重连间隔（秒）。
    pub reconnect_interval: i32,
    #[serde(rename = "ReconnectNonce")]
    /// 重连随机因子。
    pub reconnect_nonce: i32,
    #[serde(rename = "PingInterval")]
    /// Ping 心跳间隔（秒）。
    pub ping_interval: i32,
}

/// WebSocket 客户端结果类型别名。
pub type WsClientResult<T> = Result<T, WsClientError>;

#[derive(Debug, thiserror::Error)]
/// WebSocket客户端错误类型
///
/// 定义WebSocket连接和通信过程中可能出现的各种错误
pub enum WsClientError {
    #[error("unexpected response")]
    /// 返回体缺少预期字段。
    UnexpectedResponse,
    #[error("Request error: {0}")]
    /// 端点查询 HTTP 请求失败。
    RequestError(#[from] reqwest::Error),
    #[error("Url parse error: {0}")]
    /// WebSocket 地址解析失败。
    UrlParseError(#[from] url::ParseError),
    #[error("Server error: {code}, {message}")]
    /// 服务端返回业务错误。
    ServerError {
        /// 服务端错误码。
        code: i32,
        /// 服务端错误描述。
        message: String,
    },
    #[error("Client error: {code}, {message}")]
    /// 客户端侧参数或状态错误。
    ClientError {
        /// 客户端错误码。
        code: i32,
        /// 客户端错误描述。
        message: String,
    },
    #[error("connection closed")]
    /// 连接被关闭。
    ConnectionClosed {
        /// The reason the connection was closed
        reason: Option<WsCloseReason>,
    },
    #[error("WebSocket error: {0}")]
    /// WebSocket 传输错误。
    WsError(Box<tokio_tungstenite::tungstenite::Error>),
    #[error("Prost error: {0}")]
    /// Protobuf 解码错误。
    ProstError(#[from] prost::DecodeError),
}

impl From<tokio_tungstenite::tungstenite::Error> for WsClientError {
    fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
        WsClientError::WsError(Box::new(error))
    }
}

struct Context<'a> {
    service_id: i32,
    sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>,
    stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    command_rx: &'a mut mpsc::UnboundedReceiver<Frame>,
    event_sender: &'a mut mpsc::UnboundedSender<WsEvent>,
    client_config: ClientConfig,
    ping_frame_interval: Interval,
}

impl<'a> Context<'a> {
    fn new(
        service_id: i32,
        client_config: ClientConfig,
        conn: WebSocketStream<MaybeTlsStream<TcpStream>>,
        command_rx: &'a mut mpsc::UnboundedReceiver<Frame>,
        event_sender: &'a mut mpsc::UnboundedSender<WsEvent>,
    ) -> Self {
        let (sink, stream) = conn.split();
        Context {
            service_id,
            sink,
            stream,
            command_rx,
            event_sender,
            ping_frame_interval: tokio::time::interval(Duration::from_secs(
                client_config.ping_interval as u64,
            )),
            client_config,
        }
    }

    fn send_event(&mut self, event: WsEvent) {
        let _ = self.event_sender.send(event);
    }

    async fn process_loop(&mut self) -> WsClientResult<()> {
        let mut ping_time = Instant::now();
        let mut checkout_timeout = tokio::time::interval(Duration::from_secs(1));
        loop {
            tokio::select! {
                item = self.stream.next() => {
                    match item.transpose()? {
                        Some(msg) => {
                            if msg.is_ping() {
                                ping_time = Instant::now();
                            }
                            self.handle_message(msg).await?;
                        },
                        None => return Err(WsClientError::ConnectionClosed { reason: None}),
                    }
                }
                item = self.command_rx.recv() => {
                    match item {
                        Some(command) => self.handle_send_frame(command).await?,
                        None => return Ok(()),
                    }
                }
                _ = self.ping_frame_interval.tick() => {
                        let service_id: i32 = self.service_id;
                        let frame = FrameHandler::build_ping_frame(service_id);
                        let msg = Message::Binary(frame.encode_to_vec());
                        trace!(
                            "Sending ping message:  {:?} {} {}",
                            msg,
                            msg.len(),
                            service_id
                        );
                        if let Err(e) = self.sink.send(msg).await {
                            error!("Failed to send ping message: {e:?}");
                            return Err(WsClientError::WsError(Box::new(e)));
                        }

                }

                _ = checkout_timeout.tick() => {
                    if (Instant::now() - ping_time) > HEARTBEAT_TIMEOUT {
                        return Err(WsClientError::ConnectionClosed {
                            reason: None
                        });
                    }
                }
            }
        }
    }

    async fn handle_message(&mut self, msg: WsMessage) -> WsClientResult<()> {
        match msg {
            Message::Ping(data) => {
                self.sink.send(Message::Pong(data)).await?;
            }
            Message::Binary(data) => {
                let frame = Frame::decode(&*data)?;
                trace!("Received frame: {frame:?}");

                match frame.method {
                    // FrameTypeControl
                    0 => {
                        // 直接在本地处理控制帧，避免跨线程Send问题
                        self.handle_control_frame_direct(frame)?;
                    }
                    // FrameTypeData
                    1 => {
                        if let Err(e) = self.event_sender.send(WsEvent::Data(frame)) {
                            error!("Failed to send data event: {e:?}");
                        }
                    }
                    _ => {}
                }
            }
            Message::Close(Some(close_frame)) => {
                return Err(WsClientError::ConnectionClosed {
                    reason: Some(WsCloseReason {
                        code: close_frame.code,
                        message: close_frame.reason.into_owned(),
                    }),
                });
            }
            _ => return Err(WsClientError::UnexpectedResponse),
        }

        Ok(())
    }

    /// 直接处理控制帧
    fn handle_control_frame_direct(&mut self, frame: Frame) -> WsClientResult<()> {
        let headers = &frame.headers;
        let frame_type = headers
            .iter()
            .find(|h| h.key == "type")
            .map(|h| h.value.as_str())
            .unwrap_or("");

        trace!("Received control frame: {frame_type}");

        if frame_type == "pong" {
            self.handle_pong_frame_direct(frame)?;
        }

        Ok(())
    }

    /// 处理Pong帧
    fn handle_pong_frame_direct(&mut self, frame: Frame) -> WsClientResult<()> {
        let Some(payload) = frame.payload else {
            error!("Pong frame missing payload");
            return Ok(());
        };

        let config = match serde_json::from_slice::<ClientConfig>(&payload) {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("Failed to parse ClientConfig: {e:?}");
                return Ok(());
            }
        };

        // 更新ping间隔
        let ping_interval = config.ping_interval;
        self.ping_frame_interval = tokio::time::interval(Duration::from_secs(ping_interval as u64));
        self.ping_frame_interval
            .reset_after(Duration::from_secs(ping_interval as u64));
        self.client_config = config;

        debug!("Updated ping interval from pong response: {ping_interval}s");

        Ok(())
    }

    async fn handle_send_frame(&mut self, frame: Frame) -> WsClientResult<()> {
        trace!("send frame: {frame:?}");
        let msg = Message::Binary(frame.encode_to_vec());

        self.sink.send(msg).await?;
        Ok(())
    }
}

async fn client_loop(
    service_id: i32,
    client_config: ClientConfig,
    conn: WebSocketStream<MaybeTlsStream<TcpStream>>,
    mut frame_tx: mpsc::UnboundedReceiver<Frame>,
    mut event_sender: mpsc::UnboundedSender<WsEvent>,
) {
    let mut ctx = Context::new(
        service_id,
        client_config,
        conn,
        &mut frame_tx,
        &mut event_sender,
    );

    let res = ctx.process_loop().await;
    match res {
        Ok(()) => (),
        Err(err) => {
            ctx.send_event(WsEvent::Error(err));
        }
    };
}

#[derive(Debug)]
/// WebSocket事件类型
///
/// 定义WebSocket连接中可能接收到的各种事件
pub enum WsEvent {
    /// 错误事件。
    Error(WsClientError),
    /// 数据帧事件。
    Data(Frame),
}

/// Connection close reason
#[derive(Debug)]
pub struct WsCloseReason {
    /// Close code
    pub code: CloseCode,

    /// Reason string
    pub message: String,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::{
        WsClientError, WsEndpointApiResponse, extract_endpoint_response, map_ws_api_error,
    };

    #[test]
    fn test_ws_endpoint_error_response_not_treated_as_success() {
        let payload = r#"{"code":400,"msg":"Bad Request"}"#;
        let parsed = serde_json::from_str::<WsEndpointApiResponse<serde_json::Value>>(payload)
            .expect("endpoint response should deserialize");

        assert_eq!(parsed.code, 400);
        assert_eq!(parsed.msg, "Bad Request");
        assert!(parsed.data.is_none());

        let mapped = map_ws_api_error(parsed.code, parsed.msg);
        assert!(matches!(
            mapped,
            WsClientError::ClientError { code: 400, .. }
        ));
    }

    #[test]
    fn test_ws_endpoint_success_without_data_returns_unexpected_response() {
        let resp = WsEndpointApiResponse::<super::EndPointResponse> {
            code: 0,
            msg: "success".to_string(),
            data: None,
        };

        let result = extract_endpoint_response(resp);
        assert!(matches!(result, Err(WsClientError::UnexpectedResponse)));
    }

    #[test]
    fn test_ws_endpoint_server_error_mapping_is_preserved() {
        let mapped = map_ws_api_error(1000040343, "No available endpoint".to_string());
        assert!(matches!(
            mapped,
            WsClientError::ServerError {
                code: 1000040343,
                ..
            }
        ));
    }
}
