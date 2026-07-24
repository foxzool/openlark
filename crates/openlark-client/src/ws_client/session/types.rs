//! WebSocket 会话状态与错误类型。
//!
//! 描述 session 状态转移的词汇（[`InvalidStateKind`] / [`WsCloseReason`]）与
//! 客户端错误分类（[`WsClientError`]），与产生它们的 state machine 同住
//! （见 [`super::Session`]）。

use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;

/// WebSocket 客户端结果类型别名。
pub type WsClientResult<T> = Result<T, WsClientError>;

#[derive(Debug, thiserror::Error)]
/// WebSocket 客户端错误类型。
pub enum WsClientError {
    #[error("unexpected response")]
    /// 返回体缺少预期字段。
    UnexpectedResponse,
    #[error("ws endpoint request error: {0}")]
    /// 端点发现经 core Transport 返回的错误（传输失败或飞书业务码），透传 CoreError（含 request_id）。
    RequestError(#[from] openlark_core::error::CoreError),
    #[error("Url parse error: {0}")]
    /// WebSocket 地址解析失败。
    UrlParseError(#[from] url::ParseError),
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
