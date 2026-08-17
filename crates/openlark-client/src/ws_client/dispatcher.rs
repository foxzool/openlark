//! WebSocket 事件分发处理器。
//!
//! 把原始事件负载分发到 channel 转发器、注册的 [`EventHandler`] 或可返回业务响应的
//! [`CallbackEventHandler`]；不做 schema 校验。
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

/// 回调型事件处理器。
///
/// 与 [`EventHandler`] 的区别：`handle` 可返回业务响应，经 ACK 帧的 `data`
/// 字段以 base64(JSON) 写回服务端（对齐官方 Go `OnP2CardActionTrigger` /
/// Python `register_p2_card_action_trigger` 的 callback 通道）。典型场景是
/// 卡片回传交互（`card.action.trigger`）：返回 `{"toast": {...}}` 或
/// `{"card": {...}}` 即可弹提示 / 更新卡片，服务端要求 3 秒内回包。
pub trait CallbackEventHandler: Send + Sync + 'static {
    /// 处理回调负载。
    ///
    /// 返回 `Some(value)` 作为业务响应写入 ACK `data`；`None` 表示无响应。
    fn handle(
        &self,
        payload: &[u8],
    ) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>>;
}

/// WebSocket 事件分发处理器。
///
/// 目前支持三类分发目标：
///
/// - `payload_sender(...)`：把原始负载转发到 channel
/// - `register_raw(...)`：注册原始事件处理器
/// - `register_callback(...)`：注册可返回业务响应的回调型处理器
#[derive(Clone)]
pub struct EventDispatcherHandler {
    payload_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,
    raw_handlers: HashMap<String, Arc<dyn EventHandler>>,
    callback_handlers: HashMap<String, Arc<dyn CallbackEventHandler>>,
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
            .field(
                "callback_handler_keys",
                &self.callback_handlers.keys().collect::<Vec<_>>(),
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
            callback_handlers: HashMap::new(),
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

    /// 注册回调型事件处理器（可返回业务响应写入 ACK `data`）。
    ///
    /// 按事件 `header.event_type` 匹配（例如 `"card.action.trigger"`）。同一
    /// 事件命中 callback 后不再走 [`Self::register_raw`] 注册的处理器（对齐
    /// 官方 callback 通道优先的行为）；需要旁路观察全部负载时用
    /// [`Self::payload_sender`]。
    pub fn register_callback<S, H>(mut self, key: S, handler: H) -> Result<Self, String>
    where
        S: Into<String>,
        H: CallbackEventHandler,
    {
        let key = key.into();
        if key.trim().is_empty() {
            return Err("processor key cannot be empty".to_string());
        }
        if self.callback_handlers.contains_key(&key) {
            return Err(format!("processor already registered, type: {key}"));
        }
        self.callback_handlers.insert(key, Arc::new(handler));
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
        self.dispatch_with_response(payload).map(|_| ())
    }

    /// 分发原始负载并返回 callback 型业务响应（若有）。
    ///
    /// 分发顺序（对齐官方 Go `dispatcher.Do` 的双 map 行为）：
    ///
    /// 1. `payload_sender` 转发（若有，callback 路径同样转发）
    /// 2. callback map 按 `header.event_type` 命中：调用并返回业务响应，
    ///    不再进入 raw 路径
    /// 3. 未命中 callback：按 `event_type` 分发 raw handler，再分发 `"raw"`
    ///    catch-all
    ///
    /// 返回 `Some(json_bytes)` 表示应写入 ACK `data` 的业务响应。
    pub fn dispatch_with_response(&self, payload: &[u8]) -> Result<Option<Vec<u8>>, String> {
        if let Some(payload_tx) = &self.payload_tx {
            payload_tx
                .send(payload.to_vec())
                .map_err(|e| format!("转发事件负载失败: {e}"))?;
        }

        if let Some(event_type) = Self::extract_event_type(payload) {
            if let Some(handler) = self.callback_handlers.get(&event_type) {
                let value = handler
                    .handle(payload)
                    .map_err(|err| format!("处理回调事件 {event_type} 失败: {err}"))?;
                return match value {
                    Some(v) => serde_json::to_vec(&v)
                        .map(Some)
                        .map_err(|e| format!("序列化回调响应失败: {e}")),
                    None => Ok(None),
                };
            }
            self.dispatch_raw_handler(&event_type, payload)?;
        }

        self.dispatch_raw_handler(Self::RAW_EVENT_KEY, payload)?;

        Ok(None)
    }
}
