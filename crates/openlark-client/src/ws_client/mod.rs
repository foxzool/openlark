// WebSocket 客户端模块
//
// 提供WebSocket连接和事件处理功能

mod client;
mod frame_handler;
mod state_machine;

#[cfg(feature = "websocket")]
// client 模块显式导出
pub use client::{
    ClientConfig, EndPointResponse, EventDispatcherHandler, EventHandler, LarkWsClient,
    WsClientError, WsClientResult, WsCloseReason, WsEvent,
};

pub use frame_handler::{
    ControlFrameEffect, ControlFrameError, FrameHandler, FrameType,
};
pub use state_machine::{ConnectionState, StateMachineEvent, WebSocketStateMachine};

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "websocket"))]
mod full_session_tests;
