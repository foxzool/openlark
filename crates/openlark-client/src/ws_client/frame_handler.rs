//! 控制帧解释与数据帧派发（会话内部）。
//!
//! 方法分发由 [`super::session::Session`] 完成；本模块不再二次 match method。

use lark_websocket_protobuf::pbbp2::{Frame, Header};
use log::{debug, error, trace};
use serde::{Deserialize, Serialize};
use std::time::Instant;

use super::client::ClientConfig;
use super::dispatcher::EventDispatcherHandler;
use super::headers;

/// 飞书 WebSocket protobuf frame method：控制帧。
pub(crate) const FRAME_METHOD_CONTROL: i32 = 0;
/// 飞书 WebSocket protobuf frame method：数据帧。
pub(crate) const FRAME_METHOD_DATA: i32 = 1;

/// 控制帧解释结果。
#[derive(Debug, Clone)]
pub(crate) enum ControlFrameEffect {
    /// 合法 pong：仅更新 app-level ping 间隔（秒）。
    UpdatePingInterval(i32),
    /// 非 pong / 未知 type：忽略。
    Ignored,
}

/// 控制帧解释错误（例如 malformed pong）。
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub(crate) enum ControlFrameError {
    /// pong 缺少 payload 或 ClientConfig JSON 非法。
    #[error("malformed pong: {0}")]
    MalformedPong(String),
}

/// 数据帧事件应答（写回 peer 的 payload）。
#[derive(Serialize, Deserialize, Debug)]
struct EventAck {
    code: u16,
    headers: std::collections::HashMap<String, String>,
    data: Vec<u8>,
}

impl EventAck {
    fn ok() -> Self {
        Self {
            code: 200,
            headers: Default::default(),
            data: Default::default(),
        }
    }

    fn error() -> Self {
        Self {
            code: 500,
            headers: Default::default(),
            data: Default::default(),
        }
    }
}

/// 帧协议 helper（无状态）。
pub(crate) struct FrameHandler;

impl FrameHandler {
    /// 解释控制帧。
    pub(crate) fn interpret_control_frame(
        frame: &Frame,
    ) -> Result<ControlFrameEffect, ControlFrameError> {
        let frame_type =
            headers::header_value(&frame.headers, headers::HDR_TYPE).unwrap_or_default();
        trace!("Received control frame: {frame_type}");

        if frame_type != "pong" {
            if frame_type.is_empty() {
                debug!("control frame missing type header");
            } else {
                debug!("Unhandled control frame type: {frame_type}");
            }
            return Ok(ControlFrameEffect::Ignored);
        }

        let Some(payload) = frame.payload.as_ref() else {
            return Err(ControlFrameError::MalformedPong(
                "pong frame missing payload".to_string(),
            ));
        };

        match serde_json::from_slice::<ClientConfig>(payload) {
            Ok(config) => {
                debug!(
                    "Received pong with ping_interval={}s (other ClientConfig fields ignored)",
                    config.ping_interval
                );
                Ok(ControlFrameEffect::UpdatePingInterval(config.ping_interval))
            }
            Err(e) => Err(ControlFrameError::MalformedPong(format!(
                "invalid ClientConfig json: {e}"
            ))),
        }
    }

    /// 处理数据帧：派发事件并构造待写回的响应帧。
    ///
    /// 调用方（Session）负责经同一 sink 发送。
    pub(crate) fn handle_data_frame(
        mut frame: Frame,
        event_handler: &EventDispatcherHandler,
    ) -> Option<Frame> {
        let headers = &frame.headers;

        let msg_type = headers::header_value(headers, headers::HDR_TYPE).unwrap_or_default();
        let msg_id = headers::header_value(headers, headers::HDR_MESSAGE_ID).unwrap_or_default();
        let trace_id = headers::header_value(headers, headers::HDR_TRACE_ID).unwrap_or_default();

        let Some(payload) = frame.payload else {
            error!("Data frame missing payload");
            return None;
        };

        debug!(
            "Received data frame - type: {msg_type}, message_id: {msg_id}, trace_id: {trace_id}"
        );

        match msg_type {
            "event" | "" => {
                let response = Self::process_event(&payload, event_handler);

                if let Some(biz_rt) = response.headers.get("biz_rt") {
                    frame.headers.push(Header {
                        key: "biz_rt".to_string(),
                        value: biz_rt.clone(),
                    });
                }

                frame.payload = Some(serde_json::to_vec(&response).unwrap_or_else(|e| {
                    error!("Failed to serialize EventAck: {e:?}");
                    // 保证写回合法 JSON，避免空 payload 伪装成功
                    br#"{"code":500,"headers":{},"data":[]}"#.to_vec()
                }));

                Some(frame)
            }
            "card" => {
                debug!("Card frame received, skipping");
                None
            }
            other => {
                debug!("Unknown data frame type: {other}");
                None
            }
        }
    }

    fn process_event(payload: &[u8], event_handler: &EventDispatcherHandler) -> EventAck {
        let start = Instant::now();
        let result = event_handler.do_without_validation(payload);
        let elapsed = start.elapsed().as_millis();

        let mut response = match result {
            Ok(_) => EventAck::ok(),
            Err(err) => {
                error!("Failed to handle event: {err:?}");
                EventAck::error()
            }
        };
        response
            .headers
            .insert("biz_rt".to_string(), elapsed.to_string());
        response
    }

    /// 构建 app-level ping 控制帧。
    pub(crate) fn build_ping_frame(service_id: i32) -> Frame {
        Frame {
            seq_id: 0,
            log_id: 0,
            service: service_id,
            method: FRAME_METHOD_CONTROL,
            headers: vec![Header {
                key: "type".to_string(),
                value: "ping".to_string(),
            }],
            payload_encoding: None,
            payload_type: None,
            payload: None,
            log_id_new: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ws_client::EventHandler;
    use lark_websocket_protobuf::pbbp2::Header;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::sync::mpsc;

    struct CountingHandler {
        calls: Arc<AtomicUsize>,
    }

    impl EventHandler for CountingHandler {
        fn handle(&self, _payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    struct NoopHandler;

    impl EventHandler for NoopHandler {
        fn handle(&self, _payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    fn create_test_frame(method: i32, headers: Vec<Header>, payload: Option<Vec<u8>>) -> Frame {
        Frame {
            seq_id: 1,
            log_id: 12345,
            service: 1,
            method,
            headers,
            payload_encoding: None,
            payload_type: None,
            payload,
            log_id_new: None,
        }
    }

    fn create_control_frame(frame_type: &str, payload: Option<Vec<u8>>) -> Frame {
        create_test_frame(
            FRAME_METHOD_CONTROL,
            vec![Header {
                key: "type".to_string(),
                value: frame_type.to_string(),
            }],
            payload,
        )
    }

    fn create_data_frame(msg_type: &str, payload: Option<Vec<u8>>) -> Frame {
        create_test_frame(
            FRAME_METHOD_DATA,
            vec![
                Header {
                    key: "type".to_string(),
                    value: msg_type.to_string(),
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
            payload,
        )
    }

    #[test]
    fn test_header_value_existing() {
        let headers = vec![
            Header {
                key: "type".to_string(),
                value: "ping".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "123".to_string(),
            },
        ];
        assert_eq!(headers::header_value(&headers, "type"), Some("ping"));
        assert_eq!(headers::header_value(&headers, "message_id"), Some("123"));
    }

    #[test]
    fn test_header_value_nonexistent() {
        let headers = vec![Header {
            key: "type".to_string(),
            value: "ping".to_string(),
        }];
        assert_eq!(headers::header_value(&headers, "nonexistent"), None);
    }

    #[test]
    fn test_header_value_empty_list() {
        let headers: Vec<Header> = vec![];
        assert_eq!(headers::header_value(&headers, "type"), None);
    }

    #[test]
    fn test_header_value_duplicate_keys_returns_first() {
        let headers = vec![
            Header {
                key: "type".to_string(),
                value: "first".to_string(),
            },
            Header {
                key: "type".to_string(),
                value: "second".to_string(),
            },
        ];
        assert_eq!(headers::header_value(&headers, "type"), Some("first"));
    }

    #[test]
    fn test_build_ping_frame() {
        let frame = FrameHandler::build_ping_frame(42);
        assert_eq!(frame.service, 42);
        assert_eq!(frame.method, FRAME_METHOD_CONTROL);
        assert_eq!(frame.headers.len(), 1);
        assert_eq!(frame.headers[0].key, "type");
        assert_eq!(frame.headers[0].value, "ping");
        assert!(frame.payload.is_none());
    }

    #[test]
    fn test_interpret_control_frame_pong_valid() {
        let payload =
            br#"{"ReconnectCount":3,"ReconnectInterval":5,"ReconnectNonce":123,"PingInterval":30}"#
                .to_vec();
        let frame = create_control_frame("pong", Some(payload));
        let effect = FrameHandler::interpret_control_frame(&frame).expect("valid pong");
        match effect {
            ControlFrameEffect::UpdatePingInterval(secs) => {
                assert_eq!(secs, 30);
            }
            other => panic!("expected UpdatePingInterval, got {other:?}"),
        }
    }

    #[test]
    fn test_interpret_control_frame_pong_invalid_json() {
        let frame = create_control_frame("pong", Some(b"{ invalid json".to_vec()));
        let err = FrameHandler::interpret_control_frame(&frame).expect_err("malformed");
        assert!(matches!(err, ControlFrameError::MalformedPong(_)));
    }

    #[test]
    fn test_interpret_control_frame_pong_no_payload() {
        let frame = create_control_frame("pong", None);
        let err = FrameHandler::interpret_control_frame(&frame).expect_err("missing payload");
        assert!(matches!(err, ControlFrameError::MalformedPong(_)));
    }

    #[test]
    fn test_interpret_control_frame_unhandled_type() {
        let frame = create_control_frame("unknown_type", None);
        let effect = FrameHandler::interpret_control_frame(&frame).expect("ignored");
        assert!(matches!(effect, ControlFrameEffect::Ignored));
    }

    #[test]
    fn test_interpret_control_frame_no_type_header() {
        let frame = create_test_frame(FRAME_METHOD_CONTROL, vec![], None);
        let effect = FrameHandler::interpret_control_frame(&frame).expect("ignored");
        assert!(matches!(effect, ControlFrameEffect::Ignored));
    }

    #[test]
    fn test_handle_data_frame_event_success() {
        let event_handler = EventDispatcherHandler::builder().build();
        let payload = b"test event data".to_vec();
        let frame = create_data_frame("event", Some(payload));
        let result = FrameHandler::handle_data_frame(frame, &event_handler);

        assert!(result.is_some());
        let returned = result.unwrap();
        assert_eq!(returned.method, FRAME_METHOD_DATA);
        assert!(returned.headers.iter().any(|h| h.key == "biz_rt"));
        let response_json = String::from_utf8(returned.payload.unwrap()).unwrap();
        assert!(response_json.contains("\"code\":200"));
    }

    #[test]
    fn test_handle_data_frame_event_no_payload() {
        let event_handler = EventDispatcherHandler::builder().build();
        let frame = create_data_frame("event", None);
        assert!(FrameHandler::handle_data_frame(frame, &event_handler).is_none());
    }

    #[test]
    fn test_handle_data_frame_card() {
        let event_handler = EventDispatcherHandler::builder().build();
        let frame = create_data_frame("card", Some(b"card data".to_vec()));
        assert!(FrameHandler::handle_data_frame(frame, &event_handler).is_none());
    }

    #[test]
    fn test_handle_data_frame_unknown_type() {
        let event_handler = EventDispatcherHandler::builder().build();
        let frame = create_data_frame("unknown_type", Some(b"data".to_vec()));
        assert!(FrameHandler::handle_data_frame(frame, &event_handler).is_none());
    }

    #[test]
    fn test_handle_data_frame_missing_headers_still_processes_as_event() {
        let event_handler = EventDispatcherHandler::builder().build();
        let frame = create_test_frame(FRAME_METHOD_DATA, vec![], Some(b"data".to_vec()));
        let result = FrameHandler::handle_data_frame(frame, &event_handler);
        assert!(result.is_some());
        assert_eq!(result.unwrap().method, FRAME_METHOD_DATA);
    }

    #[test]
    fn test_process_event_success() {
        let event_handler = EventDispatcherHandler::builder().build();
        let response = FrameHandler::process_event(b"test data", &event_handler);
        assert_eq!(response.code, 200);
        assert!(response.headers.contains_key("biz_rt"));
    }

    #[test]
    fn test_event_dispatcher_forwards_payload_when_sender_exists() {
        let (payload_tx, mut payload_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let handler = EventDispatcherHandler::builder()
            .payload_sender(payload_tx)
            .build();

        let payload = b"payload-forward-test".to_vec();
        assert!(handler.do_without_validation(&payload).is_ok());
        assert_eq!(
            payload_rx.try_recv().expect("payload should be forwarded"),
            payload
        );
    }

    #[test]
    fn test_event_dispatcher_no_sender_still_ok() {
        let handler = EventDispatcherHandler::builder().build();
        assert!(
            handler
                .do_without_validation(b"payload-without-sender")
                .is_ok()
        );
    }

    #[test]
    fn test_event_dispatcher_returns_err_when_sender_closed() {
        let (payload_tx, payload_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        drop(payload_rx);
        let handler = EventDispatcherHandler::builder()
            .payload_sender(payload_tx)
            .build();
        assert!(handler.do_without_validation(b"closed-channel").is_err());
    }

    #[test]
    fn test_event_dispatcher_registers_raw_catch_all_handler() {
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

        let payload = br#"{"header":{"event_type":"im.message.receive_v1"}}"#;
        assert!(handler.do_without_validation(payload).is_ok());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_event_dispatcher_registers_event_type_specific_handler() {
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
        assert!(handler.do_without_validation(payload).is_ok());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_event_dispatcher_rejects_duplicate_raw_handler_keys() {
        let handler = EventDispatcherHandler::builder()
            .register_raw("raw", NoopHandler)
            .expect("first registration should work");
        assert!(handler.register_raw("raw", NoopHandler).is_err());
    }

    #[test]
    fn test_event_ack_serialization() {
        let response = EventAck::ok();
        let json = serde_json::to_string(&response).unwrap();
        let deserialized: EventAck = serde_json::from_str(&json).expect("JSON 反序列化失败");
        assert_eq!(response.code, deserialized.code);
        assert_eq!(response.headers, deserialized.headers);
        assert_eq!(response.data, deserialized.data);
    }
}
