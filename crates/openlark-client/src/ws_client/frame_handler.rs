//! 控制帧解释与数据帧派发（会话内部）。
//!
//! 方法分发由 [`super::session::Session`] 完成；本模块不再二次 match method。

use lark_websocket_protobuf::pbbp2::{Frame, Header};
use log::{debug, error, trace, warn};
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
///
/// wire 格式对齐官方 SDK（`ws/model.go` Response / `ws/client.py` Response）：
/// `{"code":200,"headers":{...},"data":"<base64>"}`；无业务数据时省略 `data` 字段。
#[derive(Serialize, Deserialize, Debug)]
struct EventAck {
    code: u16,
    headers: std::collections::HashMap<String, String>,
    #[serde(
        with = "ack_data_base64",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    data: Option<Vec<u8>>,
}

/// ACK `data` 字段编解码：base64 字符串。
///
/// 官方各家实现一致（Go `ws/model.go` `Response.Data []byte` 经 encoding/json、
/// Python `ws/client.py` `base64.b64encode`、Node `ws-client/index.ts`
/// `Buffer.toString("base64")`、Java `ws/model/Base64TypeAdapterFactory`），
/// 业务数据以 `base64(JSON(handler 返回值))` 携带；
/// serde 对 `Vec<u8>` 默认的 JSON 数组是错误格式。
mod ack_data_base64 {
    use base64::Engine;
    use serde::{Deserializer, Serializer};

    pub(super) fn serialize<S>(data: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match data {
            Some(bytes) => {
                serializer.serialize_str(&base64::engine::general_purpose::STANDARD.encode(bytes))
            }
            None => serializer.serialize_none(),
        }
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Option<Vec<u8>>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("base64 string or null")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                base64::engine::general_purpose::STANDARD
                    .decode(value)
                    .map(Some)
                    .map_err(|e| E::custom(format!("invalid base64 in ack data: {e}")))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                // serde_json 的 deserialize_option 对非 null 值走 visit_some；
                // null 会在此处的 deserialize_str 里落到 visit_unit。
                deserializer.deserialize_str(self)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_option(Visitor)
    }
}

impl EventAck {
    fn ok() -> Self {
        Self {
            code: 200,
            headers: Default::default(),
            data: None,
        }
    }

    /// 携带业务响应的成功应答（data 以 base64 写回，官方 callback 通道）。
    fn ok_with_data(data: Vec<u8>) -> Self {
        Self {
            code: 200,
            headers: Default::default(),
            data: Some(data),
        }
    }

    fn error() -> Self {
        Self {
            code: 500,
            headers: Default::default(),
            data: None,
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
                    // 保证写回合法 JSON，避免空 payload 伪装成功（无业务数据，不带 data）
                    br#"{"code":500,"headers":{}}"#.to_vec()
                }));

                Some(frame)
            }
            // 官方 Go/Python/Java/Node SDK 对 type=card 帧一律丢弃（不分发、不回 ACK）：
            // Go ws/client.go `case MessageTypeCard: return` 自 2023-10 初版至今未变，
            // `WithCardHandler` 为被注释掉的死代码。新版卡片回调（card.action.trigger）
            // 官方经 type=event 帧 + payload `header.event_type` 走 event 分支；
            // 旧版消息卡片回调官方明示不支持长连接。若线上收到 card 帧，通常是应用
            // 回调订阅配置未生效（参照 larksuite/oapi-sdk-python#126），重新发布配置后
            // 回调会改经 event 帧到达，故打 warn 提示而非静默丢弃。
            "card" => {
                warn!(
                    "Card frame received, skipping (official SDKs drop type=card frames; card callbacks arrive as type=event frames with header.event_type=card.action.trigger)"
                );
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
        let result = event_handler.dispatch_with_response(payload);
        let elapsed = start.elapsed().as_millis();

        let mut response = match result {
            // callback 型业务响应写入 ACK data（base64），对齐官方
            // `if rsp != nil { resp.Data = json.Marshal(rsp) }`
            Ok(Some(data)) => EventAck::ok_with_data(data),
            Ok(None) => EventAck::ok(),
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
    use base64::Engine;
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
        // 官方 Go/Python/Java/Node SDK 对 type=card 帧一律丢弃（不分发、不回 ACK）；
        // 新版卡片回调官方路径是 type=event 帧 + payload header.event_type=card.action.trigger
        //（见 test_event_frame_card_action_trigger_dispatches_by_event_type）。
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

    #[test]
    fn test_event_ack_data_serializes_as_base64_string() {
        // 官方 ACK wire 格式：data 为 base64 字符串（Go []byte+json.Marshal / Python
        // b64encode / Node Buffer.toString("base64") / Java Base64TypeAdapterFactory 一致），
        // 绝不是 serde 对 Vec<u8> 默认的 JSON 数组。
        let response = EventAck {
            code: 200,
            headers: Default::default(),
            data: Some(vec![1, 2, 3]),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains(r#""data":"AQID""#), "got: {json}");
    }

    #[test]
    fn test_event_ack_data_base64_round_trip() {
        let json = r#"{"code":200,"headers":{},"data":"AQID"}"#;
        let ack: EventAck = serde_json::from_str(json).expect("base64 data 应可反序列化");
        assert_eq!(ack.data, Some(vec![1, 2, 3]));
        // Serialize/Deserialize 对称：往返后字节一致
        assert_eq!(serde_json::to_string(&ack).unwrap(), json);
    }

    #[test]
    fn test_event_ack_ok_omits_data_field() {
        // 无业务数据时省略 data 字段（Python/Java/Node 行为；Go 输出 null，两者皆为官方变体）
        let json = serde_json::to_string(&EventAck::ok()).unwrap();
        assert!(!json.contains("data"), "got: {json}");
        let json = serde_json::to_string(&EventAck::error()).unwrap();
        assert!(!json.contains("data"), "got: {json}");
    }

    #[test]
    fn test_event_ack_rejects_json_array_data() {
        // 旧的错误 wire 格式（JSON 数组）必须被拒绝，防止回归
        let json = r#"{"code":200,"headers":{},"data":[1,2,3]}"#;
        assert!(serde_json::from_str::<EventAck>(json).is_err());
    }

    #[test]
    fn test_event_frame_card_action_trigger_dispatches_by_event_type() {
        // 官方长连接卡片回调路径 fixture（依官方文档《卡片回传交互回调》schema 2.0 结构）：
        // type=event 帧 + payload header.event_type=card.action.trigger，
        // 由 dispatcher 按 event_type 路由（对齐官方 OnP2CardActionTrigger/register_p2_card_action_trigger）。
        let calls = Arc::new(AtomicUsize::new(0));
        let event_handler = EventDispatcherHandler::builder()
            .register_raw(
                "card.action.trigger",
                CountingHandler {
                    calls: Arc::clone(&calls),
                },
            )
            .expect("card.action.trigger handler should register")
            .build();

        let payload = br#"{"schema":"2.0","header":{"event_id":"f7984f25108f8137722bb63c1d00bd823c2","event_type":"card.action.trigger","create_time":"1603977298000000","token":"066zT6pS4QCbgj5Do145GfDbbagrRzvV3","app_id":"cli_a511af62e2b5d07f","tenant_key":"736588c9260f175d"},"event":{"operator":{"tenant_key":"736588c9260f175d","user_id":"on_8f6f0d15799e5c45","open_id":"ou_4063d88c980c9f2d"},"token":"c-295eed59e6dbb014b72cba6f2ff6d48da9971e99","action":{"value":{"key":"value"},"tag":"button","timezone":"8","name":"btn","form_value":{},"input_value":"","option":"","options":[],"checked":false},"host":"im_message","context":{"open_message_id":"om_dc0d7ab6d7b734d5bff5c0556e8e7616","open_chat_id":"oc_4d83f1dc8596c773a09e86f50a931b77"}}}"#.to_vec();
        let frame = create_data_frame("event", Some(payload));

        let result = FrameHandler::handle_data_frame(frame, &event_handler);
        assert_eq!(calls.load(Ordering::SeqCst), 1, "卡片回调应经 event 帧分发");
        let returned = result.expect("event 帧必须回写 ACK");
        let body = String::from_utf8(returned.payload.expect("ACK payload")).unwrap();
        assert!(body.contains(r#""code":200"#), "got: {body}");
        // 当前 EventHandler 无返回值通道，业务响应恒为空 → 不携带 data 字段
        assert!(!body.contains(r#""data":"#), "got: {body}");
    }

    /// 返回 toast 业务响应的 callback handler fixture（官方 CardActionTriggerResponse 形态）。
    struct ToastCallback {
        calls: Arc<AtomicUsize>,
        response: Option<serde_json::Value>,
    }

    impl crate::ws_client::CallbackEventHandler for ToastCallback {
        fn handle(
            &self,
            _payload: &[u8],
        ) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(self.response.clone())
        }
    }

    /// 官方 schema 2.0 的 card.action.trigger event 帧 payload。
    fn card_action_trigger_payload() -> Vec<u8> {
        br#"{"schema":"2.0","header":{"event_id":"f7984f25108f8137722bb63c1d00bd823c2","event_type":"card.action.trigger","create_time":"1603977298000000","token":"066zT6pS4QCbgj5Do145GfDbbagrRzvV3","app_id":"cli_a511af62e2b5d07f","tenant_key":"736588c9260f175d"},"event":{"operator":{"tenant_key":"736588c9260f175d","user_id":"on_8f6f0d15799e5c45","open_id":"ou_4063d88c980c9f2d"},"token":"c-295eed59e6dbb014b72cba6f2ff6d48da9971e99","action":{"value":{"key":"value"},"tag":"button","timezone":"8","name":"btn"},"host":"im_message","context":{"open_message_id":"om_dc0d7ab6d7b734d5bff5c0556e8e7616","open_chat_id":"oc_4d83f1dc8596c773a09e86f50a931b77"}}}"#.to_vec()
    }

    #[test]
    fn test_callback_response_writes_base64_ack_data() {
        // callback handler 返回业务响应（toast）→ ACK data = base64(JSON)，
        // 对齐官方 Go ws/client.go `if rsp != nil { resp.Data = json.Marshal(rsp) }`。
        let calls = Arc::new(AtomicUsize::new(0));
        let toast = serde_json::json!({"toast": {"type": "success", "content": "卡片交互成功"}});
        let event_handler = EventDispatcherHandler::builder()
            .register_callback(
                "card.action.trigger",
                ToastCallback {
                    calls: Arc::clone(&calls),
                    response: Some(toast.clone()),
                },
            )
            .expect("callback handler should register")
            .build();

        let frame = create_data_frame("event", Some(card_action_trigger_payload()));
        let returned =
            FrameHandler::handle_data_frame(frame, &event_handler).expect("event 帧必须回写 ACK");
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        let body: serde_json::Value =
            serde_json::from_slice(&returned.payload.expect("ACK payload")).unwrap();
        assert_eq!(body["code"], 200, "got: {body}");
        // data 是 base64(JSON(toast))，解码后与 handler 返回值逐字节一致
        let data_b64 = body["data"].as_str().expect("data 应为 base64 字符串");
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(data_b64)
            .expect("data 应为合法 base64");
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&decoded).unwrap(),
            toast
        );
    }

    #[test]
    fn test_callback_none_response_omits_ack_data() {
        let event_handler = EventDispatcherHandler::builder()
            .register_callback(
                "card.action.trigger",
                ToastCallback {
                    calls: Arc::new(AtomicUsize::new(0)),
                    response: None,
                },
            )
            .expect("callback handler should register")
            .build();

        let frame = create_data_frame("event", Some(card_action_trigger_payload()));
        let returned =
            FrameHandler::handle_data_frame(frame, &event_handler).expect("event 帧必须回写 ACK");
        let body = String::from_utf8(returned.payload.expect("ACK payload")).unwrap();
        assert!(body.contains(r#""code":200"#), "got: {body}");
        assert!(
            !body.contains(r#""data":"#),
            "无业务响应应省略 data: {body}"
        );
    }

    #[test]
    fn test_callback_error_yields_ack_500() {
        struct FailingCallback;
        impl crate::ws_client::CallbackEventHandler for FailingCallback {
            fn handle(
                &self,
                _payload: &[u8],
            ) -> Result<Option<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>>
            {
                Err("callback failed".into())
            }
        }

        let event_handler = EventDispatcherHandler::builder()
            .register_callback("card.action.trigger", FailingCallback)
            .expect("callback handler should register")
            .build();

        let frame = create_data_frame("event", Some(card_action_trigger_payload()));
        let returned =
            FrameHandler::handle_data_frame(frame, &event_handler).expect("event 帧必须回写 ACK");
        let body = String::from_utf8(returned.payload.expect("ACK payload")).unwrap();
        assert!(body.contains(r#""code":500"#), "got: {body}");
    }

    #[test]
    fn test_callback_takes_precedence_over_raw_for_same_event_type() {
        // 对齐官方 dispatcher.Do：callback map 命中即返回，不再走普通事件 handler
        let callback_calls = Arc::new(AtomicUsize::new(0));
        let raw_calls = Arc::new(AtomicUsize::new(0));
        let event_handler = EventDispatcherHandler::builder()
            .register_callback(
                "card.action.trigger",
                ToastCallback {
                    calls: Arc::clone(&callback_calls),
                    response: None,
                },
            )
            .expect("callback handler should register")
            .register_raw(
                "card.action.trigger",
                CountingHandler {
                    calls: Arc::clone(&raw_calls),
                },
            )
            .expect("raw handler should register")
            .build();

        let payload = card_action_trigger_payload();
        event_handler
            .do_without_validation(&payload)
            .expect("dispatch should succeed");
        assert_eq!(callback_calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            raw_calls.load(Ordering::SeqCst),
            0,
            "callback 命中不应落 raw"
        );
    }

    #[test]
    fn test_callback_unregistered_event_falls_back_to_raw() {
        let raw_calls = Arc::new(AtomicUsize::new(0));
        let event_handler = EventDispatcherHandler::builder()
            .register_callback(
                "card.action.trigger",
                ToastCallback {
                    calls: Arc::new(AtomicUsize::new(0)),
                    response: None,
                },
            )
            .expect("callback handler should register")
            .register_raw(
                EventDispatcherHandler::RAW_EVENT_KEY,
                CountingHandler {
                    calls: Arc::clone(&raw_calls),
                },
            )
            .expect("raw handler should register")
            .build();

        // 非 callback 事件照常走 raw 路径
        let payload = br#"{"header":{"event_type":"im.message.receive_v1"}}"#;
        event_handler
            .do_without_validation(payload)
            .expect("dispatch should succeed");
        assert_eq!(raw_calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_register_callback_rejects_duplicate_key() {
        let handler = EventDispatcherHandler::builder()
            .register_callback(
                "card.action.trigger",
                ToastCallback {
                    calls: Arc::new(AtomicUsize::new(0)),
                    response: None,
                },
            )
            .expect("first registration should work");
        assert!(
            handler
                .register_callback(
                    "card.action.trigger",
                    ToastCallback {
                        calls: Arc::new(AtomicUsize::new(0)),
                        response: None,
                    },
                )
                .is_err()
        );
    }

    #[test]
    fn test_callback_path_still_forwards_payload_sender() {
        // payload_tx 转发在 callback 路径仍然发生（channel 消费方不应漏掉 callback 事件）
        let (payload_tx, mut payload_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let event_handler = EventDispatcherHandler::builder()
            .payload_sender(payload_tx)
            .register_callback(
                "card.action.trigger",
                ToastCallback {
                    calls: Arc::new(AtomicUsize::new(0)),
                    response: None,
                },
            )
            .expect("callback handler should register")
            .build();

        let payload = card_action_trigger_payload();
        event_handler
            .do_without_validation(&payload)
            .expect("dispatch should succeed");
        assert_eq!(
            payload_rx.try_recv().expect("payload should be forwarded"),
            payload
        );
    }
}
