//! 高频 typed 事件 payload（schema 2.0）。
//!
//! 本阶段只覆盖机器人主路径：
//! - [`ImMessageReceiveV1`]（`im.message.receive_v1`）
//! - [`CardActionTrigger`]（`card.action.trigger`）
//!
//! [`EventDispatcherHandler::register_im_message_receive_v1`] 写入 raw map，对齐官方
//! `OnP2MessageReceiveV1` / `register_p2_im_message_receive_v1`。
//! [`EventDispatcherHandler::register_card_action_trigger`] 写入 callback map，对齐官方
//! `OnP2CardActionTrigger` / `register_p2_card_action_trigger`，ACK 业务 JSON 仍走
//! [`CallbackEventHandler`]。
//!
//! `url.preview.get` 本阶段不做 typed 注册，继续用
//! [`EventDispatcherHandler::register_callback`]。全量约 200 个 `register_p2_*` 不在本模块范围。

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::serialization_error;

use super::{CallbackEventHandler, EventDispatcherHandler, EventHandler};

/// schema 2.0 事件头（高频字段）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct EventHeader {
    /// 事件 ID。
    #[serde(default)]
    pub event_id: String,
    /// 事件类型。
    #[serde(default)]
    pub event_type: String,
    /// 创建时间（毫秒字符串）。
    #[serde(default)]
    pub create_time: String,
    /// verification token。
    #[serde(default)]
    pub token: String,
    /// 应用 ID。
    #[serde(default)]
    pub app_id: String,
    /// 租户 key。
    #[serde(default)]
    pub tenant_key: String,
}

/// `im.message.receive_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImMessageReceiveV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImMessageReceiveV1Event,
}

/// `im.message.receive_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImMessageReceiveV1Event {
    /// 发送者。
    #[serde(default)]
    pub sender: Option<ImMessageSender>,
    /// 消息。
    pub message: ImReceivedMessage,
}

/// 消息发送者。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ImMessageSender {
    /// sender_id 结构。
    #[serde(default)]
    pub sender_id: Option<ImSenderId>,
    /// 发送者类型，例如 `user`。
    #[serde(default)]
    pub sender_type: String,
    /// 租户 key。
    #[serde(default)]
    pub tenant_key: String,
}

/// 发送者 ID 集合。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct ImSenderId {
    /// union_id。
    #[serde(default)]
    pub union_id: String,
    /// user_id。
    #[serde(default)]
    pub user_id: String,
    /// open_id。
    #[serde(default)]
    pub open_id: String,
}

/// 收到的 IM 消息。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ImReceivedMessage {
    /// 消息 ID。
    #[serde(default)]
    pub message_id: String,
    /// 根消息 ID。
    #[serde(default)]
    pub root_id: String,
    /// 父消息 ID。
    #[serde(default)]
    pub parent_id: String,
    /// 创建时间。
    #[serde(default)]
    pub create_time: String,
    /// 更新时间。
    #[serde(default)]
    pub update_time: String,
    /// 聊天 ID。
    #[serde(default)]
    pub chat_id: String,
    /// 话题 ID。无话题时为空。
    #[serde(default)]
    pub thread_id: String,
    /// 聊天类型。
    #[serde(default)]
    pub chat_type: String,
    /// 消息类型。
    #[serde(default)]
    pub message_type: String,
    /// 内容 JSON 字符串。
    #[serde(default)]
    pub content: String,
    /// 被 @ 的用户。
    #[serde(default)]
    pub mentions: Vec<ImMention>,
}

/// 消息中的 @ 提及。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct ImMention {
    /// 序号，例如 `@_user_1`。
    #[serde(default)]
    pub key: String,
    /// 被提及用户 ID。
    #[serde(default)]
    pub id: Option<ImSenderId>,
    /// `user` 或 `bot`。
    #[serde(default)]
    pub mentioned_type: String,
    /// 姓名。
    #[serde(default)]
    pub name: String,
    /// 租户 key。
    #[serde(default)]
    pub tenant_key: String,
}

/// `im.message.receive_v1` typed 处理器。
///
/// 闭包只要签名匹配也会自动实现本 trait。
pub trait ImMessageReceiveV1Handler: Send + Sync + 'static {
    /// 处理反序列化后的事件。
    fn handle(
        &self,
        event: ImMessageReceiveV1,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

impl<F> ImMessageReceiveV1Handler for F
where
    F: Fn(ImMessageReceiveV1) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
        + Send
        + Sync
        + 'static,
{
    fn handle(
        &self,
        event: ImMessageReceiveV1,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self(event)
    }
}

struct ImMessageReceiveV1Adapter<H>(H);

impl<H: ImMessageReceiveV1Handler> EventHandler for ImMessageReceiveV1Adapter<H> {
    fn handle(&self, payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let event: ImMessageReceiveV1 = serde_json::from_slice(payload).map_err(|e| {
            serialization_error(format!("反序列化 im.message.receive_v1 失败: {e}"))
        })?;
        self.0.handle(event)
    }

    fn handle_from_value(
        &self,
        value: &Value,
        _payload: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let event: ImMessageReceiveV1 = ImMessageReceiveV1::deserialize(value).map_err(|e| {
            serialization_error(format!("反序列化 im.message.receive_v1 失败: {e}"))
        })?;
        self.0.handle(event)
    }
}

/// `card.action.trigger` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardActionTrigger {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CardActionTriggerEvent,
}

/// 卡片回传事件体。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardActionTriggerEvent {
    /// 操作者。
    #[serde(default)]
    pub operator: Option<CardOperator>,
    /// 回传 token。
    #[serde(default)]
    pub token: String,
    /// 动作。
    #[serde(default)]
    pub action: Option<CardAction>,
    /// 宿主，例如 `im_message`。
    #[serde(default)]
    pub host: String,
    /// 卡片发送渠道，例如 `url_preview`。
    #[serde(default)]
    pub delivery_type: String,
    /// 上下文。
    #[serde(default)]
    pub context: Option<CardContext>,
}

/// 卡片操作者。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CardOperator {
    /// 租户 key。
    #[serde(default)]
    pub tenant_key: String,
    /// user_id。
    #[serde(default)]
    pub user_id: String,
    /// open_id。
    #[serde(default)]
    pub open_id: String,
}

/// 卡片点击动作。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CardAction {
    /// 自定义 value。结构因卡片而异。
    #[serde(default)]
    pub value: Option<Value>,
    /// 组件 tag。
    #[serde(default)]
    pub tag: String,
    /// 时区。
    #[serde(default)]
    pub timezone: String,
    /// 组件 name。
    #[serde(default)]
    pub name: String,
    /// 表单值。
    #[serde(default)]
    pub form_value: Option<Value>,
    /// 输入值。
    #[serde(default)]
    pub input_value: String,
    /// 单选项。
    #[serde(default)]
    pub option: String,
    /// 多选项。
    #[serde(default)]
    pub options: Vec<String>,
    /// 是否勾选。
    #[serde(default)]
    pub checked: bool,
}

/// 卡片上下文。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct CardContext {
    /// 消息 ID。
    #[serde(default)]
    pub open_message_id: String,
    /// 会话 ID。
    #[serde(default)]
    pub open_chat_id: String,
    /// 预览 URL。
    #[serde(default)]
    pub url: String,
    /// 预览 token。
    #[serde(default)]
    pub preview_token: String,
}

/// 卡片 callback 业务响应（写入 ACK `data`）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CardActionTriggerResponse {
    /// toast。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toast: Option<CardToast>,
    /// 更新后的卡片（`type` + `data`，对齐官方 Go `callback.Card`）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardActionCard>,
}

/// toast 提示。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CardToast {
    /// 类型，例如 `info` / `success`。
    #[serde(default)]
    pub r#type: String,
    /// 标题。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// ACK 中的卡片更新体。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CardActionCard {
    /// `raw` / `template` / `card_json`。
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub r#type: String,
    /// 卡片数据。模板变量或卡片 JSON，形状因 `type` 而异。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// `card.action.trigger` typed 回调处理器。
///
/// 闭包只要签名匹配也会自动实现本 trait。
pub trait CardActionTriggerHandler: Send + Sync + 'static {
    /// 处理卡片回传，可选返回 toast / 新卡片。
    fn handle(
        &self,
        event: CardActionTrigger,
    ) -> Result<Option<CardActionTriggerResponse>, Box<dyn std::error::Error + Send + Sync>>;
}

impl<F> CardActionTriggerHandler for F
where
    F: Fn(
            CardActionTrigger,
        )
            -> Result<Option<CardActionTriggerResponse>, Box<dyn std::error::Error + Send + Sync>>
        + Send
        + Sync
        + 'static,
{
    fn handle(
        &self,
        event: CardActionTrigger,
    ) -> Result<Option<CardActionTriggerResponse>, Box<dyn std::error::Error + Send + Sync>> {
        self(event)
    }
}

struct CardActionTriggerAdapter<H>(H);

impl<H: CardActionTriggerHandler> CallbackEventHandler for CardActionTriggerAdapter<H> {
    fn handle(
        &self,
        payload: &[u8],
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        let event: CardActionTrigger = serde_json::from_slice(payload)
            .map_err(|e| serialization_error(format!("反序列化 card.action.trigger 失败: {e}")))?;
        match self.0.handle(event)? {
            Some(resp) => Ok(Some(serde_json::to_value(resp)?)),
            None => Ok(None),
        }
    }

    fn handle_from_value(
        &self,
        value: &Value,
        _payload: &[u8],
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        let event: CardActionTrigger = CardActionTrigger::deserialize(value)
            .map_err(|e| serialization_error(format!("反序列化 card.action.trigger 失败: {e}")))?;
        match self.0.handle(event)? {
            Some(resp) => Ok(Some(serde_json::to_value(resp)?)),
            None => Ok(None),
        }
    }
}

impl EventDispatcherHandler {
    /// 事件类型 `im.message.receive_v1`。
    pub const IM_MESSAGE_RECEIVE_V1: &'static str = "im.message.receive_v1";
    /// 事件类型 `card.action.trigger`。
    pub const CARD_ACTION_TRIGGER: &'static str = "card.action.trigger";

    /// 注册 `im.message.receive_v1` typed 处理器。
    ///
    /// 写入 raw map。与 [`Self::register_raw`] 使用同一 `event_type` key，重复注册会失败。
    pub fn register_im_message_receive_v1<H>(self, handler: H) -> Result<Self, String>
    where
        H: ImMessageReceiveV1Handler,
    {
        self.register_raw(
            Self::IM_MESSAGE_RECEIVE_V1,
            ImMessageReceiveV1Adapter(handler),
        )
    }

    /// 注册 `card.action.trigger` typed 回调处理器（可回写 ACK）。
    ///
    /// 写入 callback map，命中后不再走 raw。与 [`Self::register_callback`] 使用同一 key。
    pub fn register_card_action_trigger<H>(self, handler: H) -> Result<Self, String>
    where
        H: CardActionTriggerHandler,
    {
        self.register_callback(Self::CARD_ACTION_TRIGGER, CardActionTriggerAdapter(handler))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::time::Instant;

    const IM_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e1","event_type":"im.message.receive_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "sender":{"sender_id":{"open_id":"ou_1"},"sender_type":"user","tenant_key":"tk"},
        "message":{
          "message_id":"om_hello",
          "chat_id":"oc_1",
          "chat_type":"p2p",
          "message_type":"text",
          "content":"{\"text\":\"hi\"}",
          "mentions":[{"key":"@_user_1","id":{"open_id":"ou_mentioned"},"name":"Ada","mentioned_type":"user"}]
        }
      }
    }"#;

    const CARD_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"f7984f25108f8137722bb63c1d00bd823c2","event_type":"card.action.trigger","create_time":"1603977298000000","token":"tok","app_id":"cli_a511af62e2b5d07f","tenant_key":"736588c9260f175d"},
      "event":{"operator":{"tenant_key":"736588c9260f175d","user_id":"on_8f6f0d15799e5c45","open_id":"ou_4063d88c980c9f2d"},"token":"c-295eed59e6dbb014b72cba6f2ff6d48da9971e99","action":{"value":{"key":"value"},"tag":"button","timezone":"8","name":"btn","form_value":{},"input_value":"","option":"","options":[],"checked":false},"host":"im_message","context":{"open_message_id":"om_dc0d7ab6d7b734d5bff5c0556e8e7616","open_chat_id":"oc_4d83f1dc8596c773a09e86f50a931b77"}}
    }"#;

    struct CaptureIm(Arc<Mutex<Option<String>>>);
    impl ImMessageReceiveV1Handler for CaptureIm {
        fn handle(
            &self,
            event: ImMessageReceiveV1,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            *self.0.lock().expect("lock") = Some(event.event.message.message_id);
            Ok(())
        }
    }

    struct ToastCard;
    impl CardActionTriggerHandler for ToastCard {
        fn handle(
            &self,
            event: CardActionTrigger,
        ) -> Result<Option<CardActionTriggerResponse>, Box<dyn std::error::Error + Send + Sync>>
        {
            assert_eq!(event.header.event_type, "card.action.trigger");
            let action = event.event.action.expect("action");
            assert_eq!(action.tag, "button");
            assert_eq!(action.name, "btn");
            Ok(Some(CardActionTriggerResponse {
                toast: Some(CardToast {
                    r#type: "info".into(),
                    title: Some("hi".into()),
                    content: None,
                }),
                card: None,
            }))
        }
    }

    #[test]
    fn typed_im_message_receive_deserializes_message_id() {
        let seen = Arc::new(Mutex::new(None));
        let handler = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(CaptureIm(Arc::clone(&seen)))
            .expect("register")
            .build();
        handler
            .do_without_validation(IM_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("om_hello"));
    }

    #[test]
    fn typed_im_message_receive_deserializes_mention_open_id() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(move |event: ImMessageReceiveV1| {
                let mention = &event.event.message.mentions[0];
                *seen_h.lock().expect("lock") = mention.id.as_ref().map(|id| id.open_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(IM_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("ou_mentioned"));
    }

    #[test]
    fn typed_card_action_returns_toast_ack() {
        let handler = EventDispatcherHandler::builder()
            .register_card_action_trigger(ToastCard)
            .expect("register")
            .build();
        let ack = handler
            .dispatch_with_response(CARD_FIXTURE.as_bytes())
            .expect("dispatch")
            .expect("ack");
        let v: Value = serde_json::from_slice(&ack).expect("json");
        assert_eq!(v["toast"]["type"], "info");
        assert_eq!(v["toast"]["title"], "hi");
        assert!(v.get("card").is_none());
    }

    #[test]
    fn register_raw_still_works_for_im_message() {
        struct Raw(Arc<Mutex<bool>>);
        impl EventHandler for Raw {
            fn handle(
                &self,
                payload: &[u8],
            ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                assert!(payload.starts_with(b"{"));
                *self.0.lock().expect("lock") = true;
                Ok(())
            }
        }
        let seen = Arc::new(Mutex::new(false));
        let handler = EventDispatcherHandler::builder()
            .register_raw("im.message.receive_v1", Raw(Arc::clone(&seen)))
            .expect("register")
            .build();
        handler
            .do_without_validation(IM_FIXTURE.as_bytes())
            .expect("dispatch");
        assert!(*seen.lock().expect("lock"));
    }

    #[test]
    fn typed_card_short_circuits_raw_for_same_event_type() {
        struct Raw(Arc<Mutex<bool>>);
        impl EventHandler for Raw {
            fn handle(
                &self,
                _payload: &[u8],
            ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                *self.0.lock().expect("lock") = true;
                Ok(())
            }
        }
        let raw_seen = Arc::new(Mutex::new(false));
        let handler = EventDispatcherHandler::builder()
            .register_card_action_trigger(ToastCard)
            .expect("callback")
            .register_raw("card.action.trigger", Raw(Arc::clone(&raw_seen)))
            .expect("raw")
            .build();
        let ack = handler
            .dispatch_with_response(CARD_FIXTURE.as_bytes())
            .expect("dispatch")
            .expect("ack");
        let v: Value = serde_json::from_slice(&ack).expect("json");
        assert_eq!(v["toast"]["type"], "info");
        assert!(!*raw_seen.lock().expect("lock"));
    }

    #[test]
    fn typed_im_bad_json_errors() {
        let handler = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(|_event: ImMessageReceiveV1| Ok(()))
            .expect("register")
            .build();
        let err = handler
            .do_without_validation(br#"{"header":{"event_type":"im.message.receive_v1"}}"#)
            .expect_err("bad json");
        assert!(
            err.contains("im.message.receive_v1"),
            "error should name the event type, got {err}"
        );
        assert!(
            err.contains("反序列化") || err.contains("missing field"),
            "error should mention deserialize failure, got {err}"
        );
    }

    #[test]
    fn typed_card_bad_json_errors() {
        let handler = EventDispatcherHandler::builder()
            .register_card_action_trigger(|_event: CardActionTrigger| Ok(None))
            .expect("register")
            .build();
        let err = handler
            .dispatch_with_response(br#"{"header":{"event_type":"card.action.trigger"}}"#)
            .expect_err("bad json");
        assert!(
            err.contains("card.action.trigger"),
            "error should name the event type, got {err}"
        );
    }

    #[test]
    fn typed_im_duplicate_register_fails() {
        let err = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(|_event: ImMessageReceiveV1| Ok(()))
            .expect("first")
            .register_raw("im.message.receive_v1", {
                struct Noop;
                impl EventHandler for Noop {
                    fn handle(
                        &self,
                        _payload: &[u8],
                    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                        Ok(())
                    }
                }
                Noop
            })
            .expect_err("dup");
        assert!(err.contains("already registered"));
    }

    #[test]
    fn typed_im_dispatch_1000_under_budget() {
        let fixture = IM_FIXTURE.as_bytes();
        let raw = EventDispatcherHandler::builder()
            .register_raw("im.message.receive_v1", {
                struct Noop;
                impl EventHandler for Noop {
                    fn handle(
                        &self,
                        _payload: &[u8],
                    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                        Ok(())
                    }
                }
                Noop
            })
            .expect("raw")
            .build();
        let typed = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(|event: ImMessageReceiveV1| {
                assert_eq!(event.event.message.message_id, "om_hello");
                Ok(())
            })
            .expect("register")
            .build();

        for _ in 0..100 {
            raw.do_without_validation(fixture).expect("warm raw");
            typed.do_without_validation(fixture).expect("warm typed");
        }

        let raw_started = Instant::now();
        for _ in 0..1000 {
            raw.do_without_validation(fixture).expect("raw");
        }
        let raw_elapsed = raw_started.elapsed();

        let typed_started = Instant::now();
        for _ in 0..1000 {
            typed.do_without_validation(fixture).expect("typed");
        }
        let typed_elapsed = typed_started.elapsed();

        eprintln!(
            "perf668 raw={raw_elapsed:?} typed={typed_elapsed:?} ratio={:.3}",
            typed_elapsed.as_secs_f64() / raw_elapsed.as_secs_f64().max(1e-12)
        );
        assert!(
            typed_elapsed.as_millis() < 500,
            "1000 typed dispatches took {typed_elapsed:?}"
        );
        assert!(
            typed_elapsed.as_secs_f64() <= raw_elapsed.as_secs_f64() * 2.0,
            "typed {typed_elapsed:?} exceeds 2x raw {raw_elapsed:?}"
        );
    }
}
