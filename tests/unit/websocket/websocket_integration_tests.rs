//! WebSocket 服务可执行覆盖测试。
//!
//! 本文件替代之前的大量 历史占位符，聚焦当前仓库里已经存在且可稳定验证的
//! WebSocket 关键路径：
//!
//! - 连接状态机的核心状态转换
//! - 数据帧处理时的 payload 转发与响应封装
//! - 自定义 raw handler 的成功 / 失败路径
//! - 控制帧（pong）的基础解析
//!
//! 这里**不**尝试伪造 live TLS / 真实长连接 / 压测场景；这些更高成本覆盖
//! 已拆分到后续 issue #104 的 follow-up 里继续收敛。

use lark_websocket_protobuf::pbbp2::{Frame, Header};
use open_lark::ws_client::{
    ConnectionState, ControlFrameEffect, EventDispatcherHandler, EventHandler, FrameHandler,
    StateMachineEvent, WebSocketStateMachine,
};
use serde_json::Value;
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::mpsc;

fn create_data_frame(payload: Vec<u8>) -> Frame {
    Frame {
        seq_id: 1,
        log_id: 1,
        service: 1,
        method: 1,
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "msg_123".to_string(),
            },
            Header {
                key: "trace_id".to_string(),
                value: "trace_456".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(payload),
        log_id_new: None,
    }
}

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
fn state_machine_connect_disconnect_flow_is_executable() {
    let mut sm = WebSocketStateMachine::new();
    assert_eq!(sm.current_state(), &ConnectionState::Initial);

    sm.handle_event(StateMachineEvent::StartConnection).unwrap();
    assert_eq!(sm.current_state(), &ConnectionState::Connecting);

    sm.handle_event(StateMachineEvent::ConnectionEstablished)
        .unwrap();
    assert_eq!(sm.current_state(), &ConnectionState::Connected);
    assert!(sm.can_send_data());

    sm.handle_event(StateMachineEvent::RequestDisconnect)
        .unwrap();
    assert_eq!(sm.current_state(), &ConnectionState::Disconnecting);

    sm.handle_event(StateMachineEvent::ConnectionClosed(None))
        .unwrap();
    assert!(matches!(
        sm.current_state(),
        ConnectionState::Disconnected { reason: None }
    ));
}

#[test]
fn state_machine_rejects_invalid_transition() {
    let mut sm = WebSocketStateMachine::new();
    let err = sm
        .handle_event(StateMachineEvent::DataReceived)
        .unwrap_err();
    assert!(err.contains("Invalid state transition"));
}

#[tokio::test]
async fn data_frame_processing_forwards_payload_and_returns_ok_response() {
    let (payload_tx, mut payload_rx) = mpsc::unbounded_channel::<Vec<u8>>();
    let handler = EventDispatcherHandler::builder()
        .payload_sender(payload_tx)
        .build();

    let payload = br#"{"header":{"event_type":"im.message.receive_v1"}}"#.to_vec();
    let frame = create_data_frame(payload.clone());
    let processed = FrameHandler::handle_frame(frame, &handler)
        .await
        .expect("event frame should produce a response");

    assert_eq!(
        payload_rx.try_recv().expect("payload should be forwarded"),
        payload
    );

    let response_payload = processed.payload.expect("response payload should exist");
    let response_json: Value =
        serde_json::from_slice(&response_payload).expect("response payload should be valid json");
    assert_eq!(response_json["code"], 200);
}

#[tokio::test]
async fn event_type_specific_raw_handler_is_invoked() {
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

    let frame = create_data_frame(br#"{"header":{"event_type":"im.message.receive_v1"}}"#.to_vec());
    let processed = FrameHandler::handle_frame(frame, &handler)
        .await
        .expect("event frame should be handled");

    assert_eq!(calls.load(Ordering::SeqCst), 1);
    let response_payload = processed.payload.expect("response payload should exist");
    let response_json: Value = serde_json::from_slice(&response_payload).unwrap();
    assert_eq!(response_json["code"], 200);
}

#[tokio::test]
async fn failing_raw_handler_produces_error_response() {
    let handler = EventDispatcherHandler::builder()
        .register_raw("im.message.receive_v1", FailingHandler)
        .expect("failing handler should register")
        .build();

    let frame = create_data_frame(br#"{"header":{"event_type":"im.message.receive_v1"}}"#.to_vec());
    let processed = FrameHandler::handle_frame(frame, &handler)
        .await
        .expect("event frame should still return a response frame");

    let response_payload = processed.payload.expect("response payload should exist");
    let response_json: Value = serde_json::from_slice(&response_payload).unwrap();
    assert_eq!(response_json["code"], 500);
}

#[tokio::test]
async fn pong_frame_builder_and_parser_still_work() {
    let frame = FrameHandler::build_ping_frame(42);
    assert_eq!(frame.method, 0);
    assert_eq!(frame.service, 42);
    assert_eq!(frame.headers[0].key, "type");
    assert_eq!(frame.headers[0].value, "ping");

    let pong = Frame {
        seq_id: 1,
        log_id: 1,
        service: 42,
        method: 0,
        headers: vec![Header {
            key: "type".to_string(),
            value: "pong".to_string(),
        }],
        payload_encoding: None,
        payload_type: None,
        payload: Some(
            serde_json::to_vec(&serde_json::json!({
                "ReconnectCount": 1,
                "ReconnectInterval": 5,
                "ReconnectNonce": 7,
                "PingInterval": 10
            }))
            .unwrap(),
        ),
        log_id_new: None,
    };
    let effect = FrameHandler::interpret_control_frame(&pong).expect("valid pong");
    match effect {
        ControlFrameEffect::UpdateClientConfig(cfg) => {
            assert_eq!(cfg.ping_interval, 10);
            assert_eq!(cfg.reconnect_count, 1);
        }
        other => panic!("expected UpdateClientConfig, got {other:?}"),
    }
}
