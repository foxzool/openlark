//! WebSocket 事件分发处理器。
//!
//! 把原始事件负载分发到 channel 转发器或注册的 [`EventHandler`]；不做 schema 校验。
//! 会话协议见 [`super::session::Session`]。

use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::mpsc;

type EventHandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

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
