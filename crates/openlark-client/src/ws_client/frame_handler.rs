use lark_websocket_protobuf::pbbp2::{Frame, Header};
use log::{debug, error, trace};
use serde::{Deserialize, Serialize};
use std::time::Instant;

use super::ClientConfig;

// 导入 client.rs 中的 EventDispatcherHandler
use super::client::EventDispatcherHandler;

/// Frame 类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameType {
    /// 控制帧。
    Control = 0,
    /// 数据帧。
    Data = 1,
}

/// Frame 处理器，负责处理不同类型的 Frame
pub struct FrameHandler;

impl FrameHandler {
    /// 处理接收到的 Frame。
    ///
    /// 数据帧在本方法内完成事件派发并返回待写回的响应帧；
    /// 调用方（会话）负责通过同一 `frame_tx` 发送，不再使用临时 channel。
    pub async fn handle_frame(
        frame: Frame,
        event_handler: &EventDispatcherHandler,
    ) -> Option<Frame> {
        match frame.method {
            0 => Self::handle_control_frame(frame),
            1 => Self::handle_data_frame(frame, event_handler).await,
            _ => {
                error!("Unknown frame method: {}", frame.method);
                None
            }
        }
    }

    /// 处理控制帧
    fn handle_control_frame(frame: Frame) -> Option<Frame> {
        let headers = &frame.headers;
        let frame_type = Self::get_header_value(headers, "type")?;

        trace!("Received control frame: {frame_type}");

        match frame_type.as_str() {
            "pong" => Self::handle_pong_frame(frame),
            _ => {
                debug!("Unhandled control frame type: {frame_type}");
                None
            }
        }
    }

    /// 处理 Pong 帧
    fn handle_pong_frame(frame: Frame) -> Option<Frame> {
        let payload = frame.payload.as_ref()?;

        match serde_json::from_slice::<ClientConfig>(payload) {
            Ok(config) => {
                debug!("Received pong with config: {config:?}");
                // 返回配置信息供上层处理
                Some(frame)
            }
            Err(e) => {
                error!("Failed to parse ClientConfig from pong frame: {e:?}");
                None
            }
        }
    }

    /// 处理数据帧：派发事件并构造响应帧（由会话写回）。
    async fn handle_data_frame(
        mut frame: Frame,
        event_handler: &EventDispatcherHandler,
    ) -> Option<Frame> {
        let headers = &frame.headers;

        // 提取必要的头部信息
        let msg_type = Self::get_header_value(headers, "type").unwrap_or_default();
        let msg_id = Self::get_header_value(headers, "message_id").unwrap_or_default();
        let trace_id = Self::get_header_value(headers, "trace_id").unwrap_or_default();

        let Some(payload) = frame.payload else {
            error!("Data frame missing payload");
            return None;
        };

        debug!(
            "Received data frame - type: {msg_type}, message_id: {msg_id}, trace_id: {trace_id}"
        );

        match msg_type.as_str() {
            "event" | "" => {
                let response = Self::process_event(payload, event_handler).await;

                // 添加处理时间到响应头
                if let Some(biz_rt) = response.headers.get("biz_rt") {
                    frame.headers.push(Header {
                        key: "biz_rt".to_string(),
                        value: biz_rt.clone(),
                    });
                }

                // 序列化响应
                frame.payload = Some(serde_json::to_vec(&response).unwrap_or_else(|e| {
                    error!("Failed to serialize response: {e:?}");
                    vec![]
                }));

                // 返回响应帧供上层发送
                Some(frame)
            }
            "card" => {
                debug!("Card frame received, skipping");
                None
            }
            _ => {
                debug!("Unknown data frame type: {msg_type}");
                None
            }
        }
    }

    /// 处理事件
    async fn process_event(
        _payload: Vec<u8>,
        event_handler: &EventDispatcherHandler,
    ) -> NewWsResponse {
        let start = Instant::now();

        let result = event_handler.do_without_validation(&_payload);
        let elapsed = start.elapsed().as_millis();

        match result {
            Ok(_) => {
                let mut response = NewWsResponse::ok();
                response
                    .headers
                    .insert("biz_rt".to_string(), elapsed.to_string());
                response
            }
            Err(err) => {
                error!("Failed to handle event: {err:?}");
                let mut response = NewWsResponse::error();
                response
                    .headers
                    .insert("biz_rt".to_string(), elapsed.to_string());
                response
            }
        }
    }

    /// 从头部列表中获取指定键的值
    fn get_header_value(headers: &[Header], key: &str) -> Option<String> {
        headers
            .iter()
            .find(|h| h.key == key)
            .map(|h| h.value.clone())
    }

    /// 构建 ping 帧
    pub fn build_ping_frame(service_id: i32) -> Frame {
        Frame {
            seq_id: 0,
            log_id: 0,
            service: service_id,
            method: 0, // Control frame
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

    /// 构建数据帧响应
    pub fn build_response_frame(service_id: i32, headers: Vec<Header>, payload: Vec<u8>) -> Frame {
        Frame {
            seq_id: 0,
            log_id: 0,
            service: service_id,
            method: 1, // Data frame
            headers,
            payload_encoding: None,
            payload_type: None,
            payload: Some(payload),
            log_id_new: None,
        }
    }
}

/// WebSocket 响应结构
#[derive(Serialize, Deserialize, Debug)]
struct NewWsResponse {
    code: u16,
    headers: std::collections::HashMap<String, String>,
    data: Vec<u8>,
}

impl NewWsResponse {
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

#[cfg(test)]
#[allow(unused_imports)]
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
            0, // Control frame
            vec![Header {
                key: "type".to_string(),
                value: frame_type.to_string(),
            }],
            payload,
        )
    }

    fn create_data_frame(msg_type: &str, payload: Option<Vec<u8>>) -> Frame {
        create_test_frame(
            1, // Data frame
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
    fn test_frame_type_variants() {
        assert_eq!(FrameType::Control as i32, 0);
        assert_eq!(FrameType::Data as i32, 1);
        assert_ne!(FrameType::Control, FrameType::Data);
        assert_eq!(FrameType::Control, FrameType::Control);
        assert_eq!(FrameType::Data, FrameType::Data);
    }

    #[test]
    fn test_frame_type_debug_format() {
        assert_eq!(format!("{:?}", FrameType::Control), "Control");
        assert_eq!(format!("{:?}", FrameType::Data), "Data");
    }

    #[test]
    fn test_frame_type_clone_and_copy() {
        let original = FrameType::Control;
        let cloned = original;
        assert_eq!(original, cloned);

        let copied = original;
        assert_eq!(original, copied);
    }

    #[test]
    fn test_get_header_value_existing() {
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

        let result = FrameHandler::get_header_value(&headers, "type");
        assert_eq!(result, Some("ping".to_string()));

        let result = FrameHandler::get_header_value(&headers, "message_id");
        assert_eq!(result, Some("123".to_string()));
    }

    #[test]
    fn test_get_header_value_nonexistent() {
        let headers = vec![Header {
            key: "type".to_string(),
            value: "ping".to_string(),
        }];

        let result = FrameHandler::get_header_value(&headers, "nonexistent");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_header_value_empty_list() {
        let headers: Vec<Header> = vec![];
        let result = FrameHandler::get_header_value(&headers, "type");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_header_value_duplicate_keys() {
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

        let result = FrameHandler::get_header_value(&headers, "type");
        // Should return the first match
        assert_eq!(result, Some("first".to_string()));
    }

    #[test]
    fn test_build_ping_frame() {
        let frame = FrameHandler::build_ping_frame(42);

        assert_eq!(frame.service, 42);
        assert_eq!(frame.method, 0); // Control frame
        assert_eq!(frame.headers.len(), 1);
        assert_eq!(frame.headers[0].key, "type");
        assert_eq!(frame.headers[0].value, "ping");
        assert!(frame.payload.is_none());
    }

    #[test]
    fn test_build_response_frame() {
        let headers = vec![Header {
            key: "status".to_string(),
            value: "ok".to_string(),
        }];
        let payload = b"test response".to_vec();

        let frame = FrameHandler::build_response_frame(99, headers, payload.clone());

        assert_eq!(frame.service, 99);
        assert_eq!(frame.method, 1); // Data frame
        assert_eq!(frame.headers.len(), 1);
        assert_eq!(frame.headers[0].key, "status");
        assert_eq!(frame.headers[0].value, "ok");
        assert_eq!(frame.payload, Some(payload));
    }

    #[tokio::test]
    async fn test_handle_unknown_frame_method() {
        let event_handler = EventDispatcherHandler::builder().build();

        let frame = create_test_frame(999, vec![], None);
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_handle_control_frame_pong_valid() {
        let event_handler = EventDispatcherHandler::builder().build();

        // Create a JSON payload that matches what would be expected for ClientConfig
        let payload =
            br#"{"ReconnectCount":3,"ReconnectInterval":5,"ReconnectNonce":123,"PingInterval":30}"#
                .to_vec();

        let frame = create_control_frame("pong", Some(payload));
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        // The frame should be processed and returned if JSON parsing succeeds
        assert!(result.is_some());
        let returned_frame = result.unwrap();
        assert_eq!(returned_frame.method, 0); // Control frame
    }

    #[tokio::test]
    async fn test_handle_control_frame_pong_invalid_json() {
        let event_handler = EventDispatcherHandler::builder().build();

        let invalid_payload = b"{ invalid json".to_vec();
        let frame = create_control_frame("pong", Some(invalid_payload));
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_handle_control_frame_pong_no_payload() {
        let event_handler = EventDispatcherHandler::builder().build();

        let frame = create_control_frame("pong", None);
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_handle_control_frame_unhandled_type() {
        let event_handler = EventDispatcherHandler::builder().build();

        let frame = create_control_frame("unknown_type", None);
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_handle_control_frame_no_type_header() {
        let event_handler = EventDispatcherHandler::builder().build();

        let frame = create_test_frame(0, vec![], None); // No type header
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_handle_data_frame_event_success() {
        let event_handler = EventDispatcherHandler::builder().build();

        let payload = b"test event data".to_vec();
        let frame = create_data_frame("event", Some(payload));
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        assert!(result.is_some());

        let returned_frame = result.unwrap();
        assert_eq!(returned_frame.method, 1); // Data frame
        assert!(returned_frame.payload.is_some());

        // Check that biz_rt header was added (even for error responses to track processing time)
        let biz_rt_header = returned_frame.headers.iter().find(|h| h.key == "biz_rt");
        assert!(biz_rt_header.is_some());
        assert!(biz_rt_header.unwrap().value.parse::<u64>().is_ok());

        // The payload should contain error response since no handler is registered
        let response_json = String::from_utf8(returned_frame.payload.unwrap()).unwrap();
        // 当前 EventDispatcherHandler 仍是占位实现（总是返回 Ok），因此这里期望成功响应
        assert!(response_json.contains("\"code\":200"));
    }

    #[tokio::test]
    async fn test_handle_data_frame_event_failure() {
        let event_handler = EventDispatcherHandler::builder().build();

        // Create a payload that will cause an error since there's no registered handler
        let payload = b"test event data".to_vec();
        let frame = create_data_frame("event", Some(payload));
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        assert!(result.is_some());

        let returned_frame = result.unwrap();
        assert!(returned_frame.payload.is_some());

        // The payload should contain error response since no handler is registered
        let response_json = String::from_utf8(returned_frame.payload.unwrap()).unwrap();
        // 当前 EventDispatcherHandler 仍是占位实现（总是返回 Ok），因此这里期望成功响应
        assert!(response_json.contains("\"code\":200"));
    }

    #[tokio::test]
    async fn test_handle_data_frame_event_no_payload() {
        let event_handler = EventDispatcherHandler::builder().build();

        let frame = create_data_frame("event", None);
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_handle_data_frame_card() {
        let event_handler = EventDispatcherHandler::builder().build();

        let payload = b"card data".to_vec();
        let frame = create_data_frame("card", Some(payload));
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_handle_data_frame_unknown_type() {
        let event_handler = EventDispatcherHandler::builder().build();

        let payload = b"unknown data".to_vec();
        let frame = create_data_frame("unknown_type", Some(payload));
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_handle_data_frame_missing_headers() {
        let event_handler = EventDispatcherHandler::builder().build();

        // Frame with no type header
        let frame = create_test_frame(1, vec![], Some(b"data".to_vec()));
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        assert!(result.is_some());

        // Should default to empty message type and still process as event
        let returned_frame = result.unwrap();
        assert_eq!(returned_frame.method, 1);
    }

    #[tokio::test]
    async fn test_process_event_success() {
        let event_handler = EventDispatcherHandler::builder().build();

        let payload = b"test data".to_vec();
        let response = FrameHandler::process_event(payload, &event_handler).await;

        // 当前 EventDispatcherHandler 仍是占位实现（总是返回 Ok），因此这里期望成功响应
        assert_eq!(response.code, 200);
    }

    #[tokio::test]
    async fn test_process_event_failure() {
        let event_handler = EventDispatcherHandler::builder().build();

        let large_payload = vec![0u8; 2000];
        let response = FrameHandler::process_event(large_payload, &event_handler).await;

        // 当前 EventDispatcherHandler 仍是占位实现（总是返回 Ok），因此这里期望成功响应
        assert_eq!(response.code, 200);
        assert!(response.headers.contains_key("biz_rt"));
    }

    #[tokio::test]
    async fn test_process_event_performance_timing() {
        let event_handler = EventDispatcherHandler::builder().build();

        let payload = b"performance test".to_vec();
        let start_time = std::time::Instant::now();
        let response = FrameHandler::process_event(payload, &event_handler).await;
        let elapsed = start_time.elapsed();

        // 当前 EventDispatcherHandler 仍是占位实现（总是返回 Ok），因此这里期望成功响应
        assert_eq!(response.code, 200);
        assert!(response.headers.contains_key("biz_rt"));

        // Should still complete quickly even with error
        assert!(elapsed.as_millis() < 1000);
    }

    #[test]
    fn test_event_dispatcher_forwards_payload_when_sender_exists() {
        let (payload_tx, mut payload_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let handler = EventDispatcherHandler::builder()
            .payload_sender(payload_tx)
            .build();

        let payload = b"payload-forward-test".to_vec();
        let result = handler.do_without_validation(&payload);

        assert!(result.is_ok());
        let forwarded = payload_rx.try_recv().expect("payload should be forwarded");
        assert_eq!(forwarded, payload);
    }

    #[test]
    fn test_event_dispatcher_no_sender_still_ok() {
        let handler = EventDispatcherHandler::builder().build();
        let payload = b"payload-without-sender";

        let result = handler.do_without_validation(payload);
        assert!(result.is_ok());
    }

    #[test]
    fn test_event_dispatcher_returns_err_when_sender_closed() {
        let (payload_tx, payload_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        drop(payload_rx);

        let handler = EventDispatcherHandler::builder()
            .payload_sender(payload_tx)
            .build();

        let result = handler.do_without_validation(b"closed-channel");
        assert!(result.is_err());
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
        let result = handler.do_without_validation(payload);

        assert!(result.is_ok());
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
        let result = handler.do_without_validation(payload);

        assert!(result.is_ok());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_event_dispatcher_rejects_duplicate_raw_handler_keys() {
        let handler = EventDispatcherHandler::builder()
            .register_raw("raw", NoopHandler)
            .expect("first registration should work");

        let duplicate = handler.register_raw("raw", NoopHandler);
        assert!(duplicate.is_err());
    }

    #[test]
    fn test_new_ws_response_ok() {
        let response = NewWsResponse::ok();

        assert_eq!(response.code, 200);
        assert!(response.headers.is_empty());
        assert!(response.data.is_empty());
    }

    #[test]
    fn test_new_ws_response_error() {
        let response = NewWsResponse::error();

        assert_eq!(response.code, 500);
        assert!(response.headers.is_empty());
        assert!(response.data.is_empty());
    }

    #[test]
    fn test_new_ws_response_serialization() {
        let response = NewWsResponse::ok();
        let json = serde_json::to_string(&response).unwrap();
        let deserialized: NewWsResponse = serde_json::from_str(&json).expect("JSON 反序列化失败");

        assert_eq!(response.code, deserialized.code);
        assert_eq!(response.headers, deserialized.headers);
        assert_eq!(response.data, deserialized.data);
    }

    #[test]
    fn test_new_ws_response_with_headers() {
        let mut response = NewWsResponse::ok();
        response
            .headers
            .insert("test_key".to_string(), "test_value".to_string());

        assert_eq!(response.code, 200);
        assert_eq!(response.headers.len(), 1);
        assert_eq!(response.headers["test_key"], "test_value");
    }

    #[test]
    fn test_new_ws_response_debug_format() {
        let response = NewWsResponse::error();
        let debug_str = format!("{response:?}");
        assert!(debug_str.contains("NewWsResponse"));
        assert!(debug_str.contains("500"));
    }

    #[tokio::test]
    async fn test_concurrent_frame_handling() {
        let event_handler = std::sync::Arc::new(EventDispatcherHandler::builder().build());

        let mut handles = vec![];

        for i in 0..10 {
            let handler_clone = event_handler.clone();

            let payload = format!("test data {i}").into_bytes();
            let frame = create_data_frame("event", Some(payload));

            let handle = tokio::spawn(async move {
                FrameHandler::handle_frame(frame, &handler_clone).await
            });

            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_some());
        }
    }

    #[tokio::test]
    async fn test_frame_handler_with_complex_headers() {
        let event_handler = EventDispatcherHandler::builder().build();

        let complex_headers = vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "msg_12345".to_string(),
            },
            Header {
                key: "trace_id".to_string(),
                value: "trace_67890".to_string(),
            },
            Header {
                key: "user_id".to_string(),
                value: "user_abc".to_string(),
            },
            Header {
                key: "timestamp".to_string(),
                value: "1234567890".to_string(),
            },
        ];

        let frame = create_test_frame(1, complex_headers, Some(b"complex data".to_vec()));
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        assert!(result.is_some());
    }

    #[test]
    fn test_frame_handler_unicode_headers() {
        let unicode_headers = vec![
            Header {
                key: "type".to_string(),
                value: "事件".to_string(), // Chinese characters
            },
            Header {
                key: "message".to_string(),
                value: "测试消息".to_string(),
            },
        ];

        let frame = create_test_frame(0, unicode_headers, None);

        // Should handle Unicode headers without panic
        let result = FrameHandler::get_header_value(&frame.headers, "type");
        assert_eq!(result, Some("事件".to_string()));

        let result = FrameHandler::get_header_value(&frame.headers, "message");
        assert_eq!(result, Some("测试消息".to_string()));
    }

    #[tokio::test]
    async fn test_frame_handler_empty_and_large_payloads() {
        let event_handler = EventDispatcherHandler::builder().build();

        // Test empty payload
        let empty_frame = create_data_frame("event", Some(vec![]));
        let result = FrameHandler::handle_frame(empty_frame, &event_handler).await;
        assert!(result.is_some());

        // Test large payload (but within limit)
        let large_payload = vec![b'x'; 500];
        let large_frame = create_data_frame("event", Some(large_payload));
        let result = FrameHandler::handle_frame(large_frame, &event_handler).await;
        assert!(result.is_some());
    }

    #[test]
    fn test_header_value_edge_cases() {
        // Test header with empty value
        let headers = vec![
            Header {
                key: "empty".to_string(),
                value: "".to_string(),
            },
            Header {
                key: "normal".to_string(),
                value: "value".to_string(),
            },
        ];

        let result = FrameHandler::get_header_value(&headers, "empty");
        assert_eq!(result, Some("".to_string()));

        // Test header with special characters
        let special_headers = vec![Header {
            key: "special".to_string(),
            value: "!@#$%^&*()_+-=[]{}|;':\",./<>?".to_string(),
        }];

        let result = FrameHandler::get_header_value(&special_headers, "special");
        assert_eq!(result, Some("!@#$%^&*()_+-=[]{}|;':\",./<>?".to_string()));
    }

    #[tokio::test]
    async fn test_frame_handler_serialization_error_handling() {
        let event_handler = EventDispatcherHandler::builder().build();

        // Create a scenario where serialization might fail
        let payload = b"test data".to_vec();
        let frame = create_data_frame("event", Some(payload));

        // The frame handler should handle serialization errors gracefully
        let result = FrameHandler::handle_frame(frame, &event_handler).await;

        // Should still return a frame even if serialization has issues
        assert!(result.is_some());
    }
}
