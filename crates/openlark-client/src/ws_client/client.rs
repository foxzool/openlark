//! WebSocket 公开入口与 endpoint 发现。
//!
//! 会话协议实现见 [`super::session::Session`]。

use std::collections::HashMap;
use std::sync::Arc;

use log::{debug, info};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async_with_config;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use url::Url;

use super::session::{Session, SessionOptions};

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

/// 飞书 WebSocket 客户端入口。
///
/// 连接建立后由内部单一 session loop 拥有：I/O、心跳、控制帧、分包、事件调度与写回。
pub struct LarkWsClient;

impl LarkWsClient {
    /// 建立 WebSocket 长连接并运行完整会话，直到关闭或错误。
    ///
    /// # 返回
    ///
    /// 生产路径在会话终止时几乎总是 `Err`：
    /// - `Err(WsClientError::ConnectionClosed { reason })`：对端 Close（含正常关闭
    ///   code）或入站空闲超时；**正常断开也是 `Err`，调用方请匹配此变体**
    /// - 其它 `Err`：端点查询、传输、malformed 控制帧、未知 frame method、
    ///   非法会话状态等
    ///
    /// 入站空闲超时**仅**在收到 WebSocket 层 `Ping` 时刷新（与历史行为一致）。
    pub async fn open(
        config: Arc<openlark_core::config::Config>,
        event_handler: EventDispatcherHandler,
    ) -> WsClientResult<()> {
        Self::open_with(config, event_handler, SessionOptions::default()).await
    }

    /// 与 [`open`](Self::open) 相同，可注入会话选项（测试用心跳超时等）。
    pub(crate) async fn open_with(
        config: Arc<openlark_core::config::Config>,
        event_handler: EventDispatcherHandler,
        options: SessionOptions,
    ) -> WsClientResult<()> {
        let end_point = Self::get_conn_url(&config).await?;
        let conn_url = end_point.url.ok_or(WsClientError::UnexpectedResponse)?;
        let client_config = end_point
            .client_config
            .ok_or(WsClientError::UnexpectedResponse)?;
        let url = Url::parse(&conn_url)?;
        let query_pairs: HashMap<_, _> = url.query_pairs().into_iter().collect();
        let service_id = query_pairs
            .get("service_id")
            .ok_or(WsClientError::UnexpectedResponse)?
            .parse()
            .map_err(|_| WsClientError::UnexpectedResponse)?;

        let ws_config = tokio_tungstenite::tungstenite::protocol::WebSocketConfig::default()
            .max_message_size(Some(config.max_response_size() as usize))
            .max_frame_size(Some(config.max_response_size() as usize));

        let (conn, _response) = connect_async_with_config(conn_url, Some(ws_config), false).await?;
        info!("connected to {url}");

        Session::new(service_id, client_config, conn, event_handler, options)
            .run()
            .await
    }

    /// 获取连接配置
    async fn get_conn_url(
        config: &Arc<openlark_core::config::Config>,
    ) -> WsClientResult<EndPointResponse> {
        let body = json!({
            "AppID": config.app_id(),
            "AppSecret": config.app_secret()
        });

        let mut http_builder = Client::builder();
        if let Some(timeout) = config.req_timeout() {
            http_builder = http_builder.timeout(timeout);
        }
        let http_client = http_builder.build()?;

        let base_url = config.base_url().trim_end_matches('/');
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

/// WebSocket 端点查询响应（crate 内部）。
#[derive(Debug, Deserialize)]
pub(crate) struct EndPointResponse {
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "ClientConfig")]
    pub client_config: Option<ClientConfig>,
}

/// 服务端下发的 WebSocket 客户端配置（crate 内部）。
///
/// 会话仅消费 `PingInterval`。endpoint/pong JSON 中可能还带 `Reconnect*` 字段，
/// serde 默认忽略未知键；本 crate 不实现重连策略（#421）。
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ClientConfig {
    #[serde(rename = "PingInterval")]
    pub(crate) ping_interval: i32,
}

/// WebSocket 客户端结果类型别名。
pub type WsClientResult<T> = Result<T, WsClientError>;

#[derive(Debug, thiserror::Error)]
/// WebSocket 客户端错误类型。
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
        /// 关闭原因。
        reason: Option<WsCloseReason>,
    },
    #[error("WebSocket error: {0}")]
    /// WebSocket 传输错误。
    WsError(Box<tokio_tungstenite::tungstenite::Error>),
    #[error("Prost error: {0}")]
    /// Protobuf 解码错误。
    ProstError(#[from] prost::DecodeError),
    #[error("malformed control frame: {message}")]
    /// 控制帧（如 pong）payload 非法。
    MalformedControlFrame {
        /// 错误描述。
        message: String,
    },
    #[error("invalid session state transition: {kind}")]
    /// 会话状态不允许的操作（例如已关闭后继续处理业务帧）。
    InvalidStateTransition {
        /// 结构化原因（测试与调用方可 match，勿依赖展示字符串）。
        kind: InvalidStateKind,
    },
    #[error("event handler panicked")]
    /// 用户 EventHandler 在 blocking 池中 panic。
    HandlerPanicked,
    #[error("handler backlog full: {message}")]
    /// 串行 handler 队列与本地 outbox 均已满。
    BacklogFull {
        /// 错误描述。
        message: String,
    },
    #[error("invalid frame method: {method}")]
    /// 未知 protobuf frame method。
    InvalidFrameMethod {
        /// 收到的 method 值。
        method: i32,
    },
}

/// 会话非法状态转换的结构化原因。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidStateKind {
    /// 需要 Active，实际为其它状态。
    ExpectedActive {
        /// 当前状态名（如 `"Closing"` / `"Closed"`）。
        actual: &'static str,
    },
    /// Closing 期间又收到业务 Binary/Text。
    DataWhileClosing,
    /// 已 Closed 仍尝试 begin_close。
    AlreadyClosed,
    /// 串行 handler worker 通道已关闭。
    WorkerGone,
}

impl std::fmt::Display for InvalidStateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedActive { actual } => {
                write!(f, "session is {actual}, expected Active")
            }
            Self::DataWhileClosing => {
                write!(f, "received data frame while session is Closing")
            }
            Self::AlreadyClosed => write!(f, "session already Closed"),
            Self::WorkerGone => write!(f, "handler worker is gone"),
        }
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for WsClientError {
    fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
        WsClientError::WsError(Box::new(error))
    }
}

/// 连接关闭原因。
#[derive(Debug, Clone)]
pub struct WsCloseReason {
    /// WebSocket 关闭码。
    pub code: CloseCode,
    /// 关闭原因文案。
    pub message: String,
}

#[cfg(test)]
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
