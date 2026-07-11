// WebSocket 客户端模块
//
// 公开入口仅暴露会话级类型（LarkWsClient / EventHandler 等）。
// frame 解析与连接状态机为 implementation detail，不作为独立 public interface。

mod client;
mod frame_handler;
mod state_machine;

#[cfg(feature = "websocket")]
/// 会话级公开 API（#429：frame / state / 内部事件通道不再 re-export）。
pub use client::{
    EventDispatcherHandler, EventHandler, LarkWsClient, WsClientError, WsClientResult,
    WsCloseReason,
};

// ClientConfig / EndPointResponse / WsEvent / frame_handler / state_machine：
// 仅 crate 内可见，不 re-export。

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "websocket"))]
mod full_session_tests;
