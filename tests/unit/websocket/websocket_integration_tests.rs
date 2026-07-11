//! WebSocket **公开 API** 可执行覆盖（#429）。
//!
//! frame handler / 状态机不再作为独立 public interface。完整会话行为（连接、
//! 派发、响应写回、pong、超时、关闭）由 `openlark-client` 内
//! `ws_client::full_session_tests` 在 `LarkWsClient::open` 上覆盖。
//!
//! 本文件只验证调用方实际依赖的公开 seam：
//! - [`EventDispatcherHandler`] 注册 / 转发 / 错误路径
//! - [`EventHandler`] trait 用法（与示例一致）

use open_lark::ws_client::{EventDispatcherHandler, EventHandler};
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::mpsc;

struct CountingHandler {
    calls: Arc<AtomicUsize>,
}

impl EventHandler for CountingHandler {
    fn handle(&self, _payload: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

struct FailingHandler;

impl EventHandler for FailingHandler {
    fn handle(&self, _payload: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        Err("boom".into())
    }
}

#[test]
fn event_dispatcher_forwards_payload_via_public_sender() {
    let (payload_tx, mut payload_rx) = mpsc::unbounded_channel::<Vec<u8>>();
    let handler = EventDispatcherHandler::builder()
        .payload_sender(payload_tx)
        .build();

    let payload = br#"{"header":{"event_type":"im.message.receive_v1"}}"#.to_vec();
    handler
        .do_without_validation(&payload)
        .expect("dispatch should succeed");

    assert_eq!(
        payload_rx.try_recv().expect("payload should be forwarded"),
        payload
    );
}

#[test]
fn event_type_specific_raw_handler_is_invoked() {
    let calls = Arc::new(AtomicUsize::new(0));
    let handler = EventDispatcherHandler::builder()
        .register_raw(
            "im.message.receive_v1",
            CountingHandler {
                calls: Arc::clone(&calls),
            },
        )
        .expect("event-specific handler should register")
        .build();

    let payload = br#"{"header":{"event_type":"im.message.receive_v1"}}"#;
    handler
        .do_without_validation(payload)
        .expect("dispatch should succeed");

    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn raw_catch_all_handler_is_invoked() {
    let calls = Arc::new(AtomicUsize::new(0));
    let handler = EventDispatcherHandler::builder()
        .register_raw(
            EventDispatcherHandler::RAW_EVENT_KEY,
            CountingHandler {
                calls: Arc::clone(&calls),
            },
        )
        .expect("raw handler should register")
        .build();

    handler
        .do_without_validation(br#"{"header":{"event_type":"any.event"}}"#)
        .expect("dispatch should succeed");

    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn failing_raw_handler_surfaces_error() {
    let handler = EventDispatcherHandler::builder()
        .register_raw("im.message.receive_v1", FailingHandler)
        .expect("failing handler should register")
        .build();

    let err = handler
        .do_without_validation(br#"{"header":{"event_type":"im.message.receive_v1"}}"#)
        .expect_err("handler failure should surface");
    assert!(err.contains("im.message.receive_v1") || err.contains("boom"));
}

#[test]
fn duplicate_raw_handler_registration_is_rejected() {
    let handler = EventDispatcherHandler::builder()
        .register_raw("raw", CountingHandler {
            calls: Arc::new(AtomicUsize::new(0)),
        })
        .expect("first registration");

    let dup = handler.register_raw(
        "raw",
        CountingHandler {
            calls: Arc::new(AtomicUsize::new(0)),
        },
    );
    assert!(dup.is_err());
}

/// 与 `websocket_echo_bot` 相同的公开导入集合应可解析。
#[test]
fn public_session_entry_types_are_importable() {
    // 类型可命名即满足「现有事件 handler 使用方式可编译」契约。
    let _ = std::any::type_name::<open_lark::ws_client::LarkWsClient>();
    let _ = std::any::type_name::<open_lark::ws_client::EventDispatcherHandler>();
    let _ = std::any::type_name::<open_lark::ws_client::WsClientError>();
    let _ = std::any::type_name::<open_lark::ws_client::WsCloseReason>();
}
