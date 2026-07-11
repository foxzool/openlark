// WebSocket 客户端模块
//
// 公开入口仅暴露会话级类型（LarkWsClient / EventHandler 等）。
// 会话协议在单一 `session` loop 中实现；frame / package 为内部细节。

mod client;
mod frame_handler;
mod headers;
mod package;
mod session;

#[cfg(feature = "websocket")]
/// 会话级公开 API。
pub use client::{
    EventDispatcherHandler, EventHandler, LarkWsClient, WsClientError, WsClientResult,
    WsCloseReason,
};

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "websocket"))]
mod full_session_tests;
