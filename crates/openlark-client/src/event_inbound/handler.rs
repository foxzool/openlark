//! 框架无关的 HTTP 事件入站处理（原始 headers + body）。

use std::collections::HashMap;

use serde::Deserialize;
use serde_json::json;

use crate::error::{Result, validation_error};
use crate::ws_client::EventDispatcherHandler;

use super::crypto::{
    HEADER_REQUEST_NONCE, HEADER_REQUEST_TIMESTAMP, HEADER_SIGNATURE, decrypt_event,
    verify_inbound_signature,
};

const REQ_TYPE_CHALLENGE: &str = "url_verification";
const CONTENT_TYPE_JSON: &str = "application/json; charset=utf-8";

/// 入站 HTTP 请求视图（不绑定具体 web 框架）。
#[derive(Debug, Clone)]
pub struct HttpEventRequest {
    /// HTTP 头（大小写不敏感查找）。
    pub headers: HashMap<String, String>,
    /// 原始 body 字节。
    pub body: Vec<u8>,
}

impl HttpEventRequest {
    /// 从 header map 与 body 构造。
    pub fn new(headers: HashMap<String, String>, body: impl Into<Vec<u8>>) -> Self {
        Self {
            headers,
            body: body.into(),
        }
    }

    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

/// 入站处理结果（建议原样写回 HTTP 响应）。
#[derive(Debug, Clone)]
pub struct HttpEventResponse {
    /// HTTP 状态码。
    pub status: u16,
    /// 响应 body。
    pub body: Vec<u8>,
    /// `Content-Type`。
    pub content_type: &'static str,
}

impl HttpEventResponse {
    fn json_ok(body: Vec<u8>) -> Self {
        Self {
            status: 200,
            body,
            content_type: CONTENT_TYPE_JSON,
        }
    }
}

#[derive(Debug, Deserialize)]
struct EncryptEnvelope {
    encrypt: String,
}

#[derive(Debug, Deserialize)]
struct FuzzyEvent {
    #[serde(default)]
    encrypt: String,
    #[serde(default)]
    #[serde(rename = "type")]
    req_type: String,
    #[serde(default)]
    challenge: String,
    #[serde(default)]
    token: String,
    #[serde(default)]
    header: Option<FuzzyHeader>,
}

#[derive(Debug, Deserialize)]
struct FuzzyHeader {
    #[serde(default)]
    token: String,
}

/// HTTP 事件入站处理器。
///
/// 解密 / Challenge / 验签后，把明文事件交给 [`EventDispatcherHandler`] 路由，
/// 以便与 WS 路径共享同一套 `register_raw` / typed / callback 注册。
#[derive(Debug, Clone)]
pub struct HttpEventInbound {
    encrypt_key: String,
    verification_token: String,
    skip_sign_verify: bool,
    dispatcher: EventDispatcherHandler,
}

/// [`HttpEventInbound`] 构建器。
#[derive(Debug, Clone)]
pub struct HttpEventInboundBuilder {
    encrypt_key: String,
    verification_token: String,
    skip_sign_verify: bool,
    dispatcher: EventDispatcherHandler,
}

impl HttpEventInbound {
    /// 创建构建器。
    ///
    /// `verification_token` / `encrypt_key` 来自开发者后台「事件订阅」。
    /// 未配置加密时 `encrypt_key` 传空字符串，明文路径仍可用。
    pub fn builder(
        verification_token: impl Into<String>,
        encrypt_key: impl Into<String>,
    ) -> HttpEventInboundBuilder {
        HttpEventInboundBuilder {
            encrypt_key: encrypt_key.into(),
            verification_token: verification_token.into(),
            skip_sign_verify: false,
            dispatcher: EventDispatcherHandler::builder().build(),
        }
    }

    /// 处理一帧入站 HTTP 请求。
    pub fn handle(&self, req: &HttpEventRequest) -> Result<HttpEventResponse> {
        let body_str = std::str::from_utf8(&req.body)
            .map_err(|e| validation_error("body", format!("body is not utf-8: {e}")))?;

        let (cipher_or_plain, used_encrypt_envelope) = self.extract_payload(body_str)?;

        let plain = if !self.encrypt_key.is_empty() && used_encrypt_envelope {
            decrypt_event(&cipher_or_plain, &self.encrypt_key)?
        } else {
            cipher_or_plain.into_bytes()
        };

        let fuzzy: FuzzyEvent = serde_json::from_slice(&plain)
            .map_err(|e| validation_error("body", format!("event json parse failed: {e}")))?;

        if !fuzzy.encrypt.is_empty() && self.encrypt_key.is_empty() {
            return Err(validation_error(
                "encrypt_key",
                "event data is encrypted, set EncryptKey for your app",
            ));
        }

        let req_type = if fuzzy.req_type.is_empty() {
            // schema 2.0 事件通常无顶层 type，视为 event_callback
            "event_callback".to_string()
        } else {
            fuzzy.req_type.clone()
        };

        if req_type != REQ_TYPE_CHALLENGE && !self.encrypt_key.is_empty() && !self.skip_sign_verify
        {
            self.verify_sign(req, body_str)?;
        }

        if req_type == REQ_TYPE_CHALLENGE {
            return self.auth_challenge(&fuzzy);
        }

        let callback_body = self
            .dispatcher
            .dispatch_with_response(&plain)
            .map_err(|e| validation_error("dispatcher", e))?;

        match callback_body {
            Some(bytes) => Ok(HttpEventResponse::json_ok(bytes)),
            None => Ok(HttpEventResponse::json_ok(br"{}".to_vec())),
        }
    }

    fn extract_payload(&self, body_str: &str) -> Result<(String, bool)> {
        if self.encrypt_key.is_empty() {
            return Ok((body_str.to_string(), false));
        }
        match serde_json::from_str::<EncryptEnvelope>(body_str) {
            Ok(env) if !env.encrypt.is_empty() => Ok((env.encrypt, true)),
            Ok(_) => Err(validation_error("encrypt", "encrypted message is blank")),
            Err(_) => {
                // 非 envelope：当作明文 JSON（便于单测 / 无加密配置）
                Ok((body_str.to_string(), false))
            }
        }
    }

    fn verify_sign(&self, req: &HttpEventRequest, body_str: &str) -> Result<()> {
        let timestamp = req
            .header(HEADER_REQUEST_TIMESTAMP)
            .ok_or_else(|| validation_error("headers", "missing X-Lark-Request-Timestamp"))?;
        let nonce = req
            .header(HEADER_REQUEST_NONCE)
            .ok_or_else(|| validation_error("headers", "missing X-Lark-Request-Nonce"))?;
        let signature = req
            .header(HEADER_SIGNATURE)
            .ok_or_else(|| validation_error("headers", "missing X-Lark-Signature"))?;

        if verify_inbound_signature(timestamp, nonce, &self.encrypt_key, body_str, signature) {
            Ok(())
        } else {
            Err(validation_error(
                "signature",
                "the result of signature verification failed",
            ))
        }
    }

    fn auth_challenge(&self, fuzzy: &FuzzyEvent) -> Result<HttpEventResponse> {
        let token = if !fuzzy.token.is_empty() {
            fuzzy.token.as_str()
        } else if let Some(h) = &fuzzy.header {
            h.token.as_str()
        } else {
            ""
        };

        if !self.verification_token.is_empty() && token != self.verification_token {
            return Err(validation_error(
                "token",
                "the result of auth by challenge failed",
            ));
        }

        let body = serde_json::to_vec(&json!({ "challenge": fuzzy.challenge }))
            .map_err(|e| validation_error("challenge", format!("serialize failed: {e}")))?;
        Ok(HttpEventResponse::json_ok(body))
    }
}

impl HttpEventInboundBuilder {
    /// 跳过入站签名校验（仅测试）。
    pub fn skip_sign_verify(mut self, skip: bool) -> Self {
        self.skip_sign_verify = skip;
        self
    }

    /// 挂载事件分发器（与 WS 共用注册表）。
    pub fn dispatcher(mut self, dispatcher: EventDispatcherHandler) -> Self {
        self.dispatcher = dispatcher;
        self
    }

    /// 完成构建。
    pub fn build(self) -> HttpEventInbound {
        HttpEventInbound {
            encrypt_key: self.encrypt_key,
            verification_token: self.verification_token,
            skip_sign_verify: self.skip_sign_verify,
            dispatcher: self.dispatcher,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_inbound::crypto::test_support::encrypt_event_for_test;
    use crate::event_inbound::crypto::{
        HEADER_REQUEST_NONCE, HEADER_REQUEST_TIMESTAMP, HEADER_SIGNATURE, inbound_signature,
    };
    use crate::ws_client::{EventDispatcherHandler, EventHandler};
    use std::sync::{Arc, Mutex};

    struct CaptureHandler {
        seen: Arc<Mutex<Vec<u8>>>,
    }

    impl EventHandler for CaptureHandler {
        fn handle(
            &self,
            payload: &[u8],
        ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
            self.seen.lock().expect("lock").extend_from_slice(payload);
            Ok(())
        }
    }

    fn headers(ts: &str, nonce: &str, sig: &str) -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert(HEADER_REQUEST_TIMESTAMP.to_string(), ts.to_string());
        m.insert(HEADER_REQUEST_NONCE.to_string(), nonce.to_string());
        m.insert(HEADER_SIGNATURE.to_string(), sig.to_string());
        m
    }

    #[test]
    fn plaintext_event_dispatches() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let dispatcher = EventDispatcherHandler::builder()
            .register_raw(
                EventDispatcherHandler::RAW_EVENT_KEY,
                CaptureHandler {
                    seen: Arc::clone(&seen),
                },
            )
            .expect("register");
        let inbound = HttpEventInbound::builder("tok", "")
            .dispatcher(dispatcher)
            .build();

        let body = br#"{"schema":"2.0","header":{"event_type":"im.message.receive_v1","token":"tok"},"event":{"message":{"message_id":"om_1"}}}"#;
        let resp = inbound
            .handle(&HttpEventRequest::new(HashMap::new(), body.to_vec()))
            .expect("handle");
        assert_eq!(resp.status, 200);
        assert_eq!(&seen.lock().expect("lock")[..], body);
    }

    #[test]
    fn challenge_returns_challenge_json() {
        let inbound = HttpEventInbound::builder("verify-tok", "").build();
        let body = br#"{"challenge":"c-123","token":"verify-tok","type":"url_verification"}"#;
        let resp = inbound
            .handle(&HttpEventRequest::new(HashMap::new(), body.to_vec()))
            .expect("handle");
        assert_eq!(resp.status, 200);
        let v: serde_json::Value = serde_json::from_slice(&resp.body).expect("json");
        assert_eq!(v["challenge"], "c-123");
    }

    #[test]
    fn ciphertext_event_decrypts_and_dispatches() {
        let key = "enc-key-1";
        let plain =
            br#"{"schema":"2.0","header":{"event_type":"im.message.receive_v1"},"event":{}}"#;
        let encrypt = encrypt_event_for_test(plain, key, &[3u8; 16]);
        let envelope = format!(r#"{{"encrypt":"{encrypt}"}}"#);

        let seen = Arc::new(Mutex::new(Vec::new()));
        let dispatcher = EventDispatcherHandler::builder()
            .register_raw(
                "im.message.receive_v1",
                CaptureHandler {
                    seen: Arc::clone(&seen),
                },
            )
            .expect("register");

        let ts = "1700000000";
        let nonce = "n1";
        let sig = inbound_signature(ts, nonce, key, &envelope);
        let inbound = HttpEventInbound::builder("tok", key)
            .dispatcher(dispatcher)
            .build();

        let resp = inbound
            .handle(&HttpEventRequest::new(
                headers(ts, nonce, &sig),
                envelope.into_bytes(),
            ))
            .expect("handle");
        assert_eq!(resp.status, 200);
        assert_eq!(&seen.lock().expect("lock")[..], plain);
    }

    #[test]
    fn bad_signature_rejected() {
        let key = "enc-key-1";
        let plain =
            br#"{"schema":"2.0","header":{"event_type":"im.message.receive_v1"},"event":{}}"#;
        let encrypt = encrypt_event_for_test(plain, key, &[3u8; 16]);
        let envelope = format!(r#"{{"encrypt":"{encrypt}"}}"#);
        let inbound = HttpEventInbound::builder("tok", key).build();
        let err = inbound
            .handle(&HttpEventRequest::new(
                headers("1700000000", "n1", "00".repeat(32).as_str()),
                envelope.into_bytes(),
            ))
            .expect_err("sig");
        assert!(err.to_string().contains("signature") || err.to_string().contains("verification"));
    }

    #[test]
    fn typed_im_handler_via_http_inbound() {
        use crate::ws_client::{ImMessageReceiveV1, ImMessageReceiveV1Handler};

        struct Capture(Arc<Mutex<Option<String>>>);
        impl ImMessageReceiveV1Handler for Capture {
            fn handle(
                &self,
                event: ImMessageReceiveV1,
            ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
                *self.0.lock().expect("lock") = Some(event.event.message.message_id);
                Ok(())
            }
        }

        let seen = Arc::new(Mutex::new(None));
        let dispatcher = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(Capture(Arc::clone(&seen)))
            .expect("register");
        let inbound = HttpEventInbound::builder("tok", "")
            .dispatcher(dispatcher)
            .build();
        let body = br#"{"schema":"2.0","header":{"event_type":"im.message.receive_v1","token":"tok"},"event":{"message":{"message_id":"om_typed"}}}"#;
        inbound
            .handle(&HttpEventRequest::new(HashMap::new(), body.to_vec()))
            .expect("handle");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("om_typed"));
    }

    #[test]
    fn typed_card_callback_via_http_inbound() {
        use crate::ws_client::{
            CardActionTrigger, CardActionTriggerHandler, CardActionTriggerResponse, CardToast,
        };

        struct Toast;
        impl CardActionTriggerHandler for Toast {
            fn handle(
                &self,
                event: CardActionTrigger,
            ) -> std::result::Result<
                Option<CardActionTriggerResponse>,
                Box<dyn std::error::Error + Send + Sync>,
            > {
                assert_eq!(event.header.event_type, "card.action.trigger");
                Ok(Some(CardActionTriggerResponse {
                    toast: Some(CardToast {
                        r#type: "success".into(),
                        title: None,
                        content: Some("ok".into()),
                    }),
                    card: None,
                }))
            }
        }

        let dispatcher = EventDispatcherHandler::builder()
            .register_card_action_trigger(Toast)
            .expect("register");
        let inbound = HttpEventInbound::builder("tok", "")
            .dispatcher(dispatcher)
            .build();
        let body = br#"{"schema":"2.0","header":{"event_type":"card.action.trigger","token":"tok"},"event":{"action":{"tag":"button","name":"btn"}}}"#;
        let resp = inbound
            .handle(&HttpEventRequest::new(HashMap::new(), body.to_vec()))
            .expect("handle");
        assert_eq!(resp.status, 200);
        let v: serde_json::Value = serde_json::from_slice(&resp.body).expect("json");
        assert_eq!(v["toast"]["content"], "ok");
        assert_eq!(v["toast"]["type"], "success");
    }
}
