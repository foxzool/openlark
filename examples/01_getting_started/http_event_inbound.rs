//! HTTP 事件入站示例（框架无关 bytes handler）。
//!
//! 对接控制台「将事件发送至开发者服务器」。这与根 crate `webhook` feature /
//! `openlark-webhook`（自定义机器人 **出站** `bot/v2/hook` HMAC）不是同一条路径。
//!
//! 默认只跑本地 fixture（challenge / 明文事件），不监听端口、不访问飞书。
//! 把 `HttpEventInbound::handle` 接到 Axum/Actix 时，传入原始 headers + body 即可。
//!
//! ```bash
//! cargo run --example http_event_inbound --no-default-features --features event-http
//! ```

use std::collections::HashMap;

use open_lark::event_inbound::{HttpEventInbound, HttpEventRequest};
use open_lark::ws_client::{EventDispatcherHandler, ImMessageReceiveV1, ImMessageReceiveV1Handler};

struct LoggingIm;

impl ImMessageReceiveV1Handler for LoggingIm {
    fn handle(
        &self,
        event: ImMessageReceiveV1,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "收到 im.message.receive_v1 message_id={}",
            event.event.message.message_id
        );
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let verification_token =
        std::env::var("OPENLARK_VERIFICATION_TOKEN").unwrap_or_else(|_| "demo_token".into());
    // 空 encrypt_key = 明文路径；配置了 encrypt_key 时平台会下发 {"encrypt":"..."}
    let encrypt_key = std::env::var("OPENLARK_ENCRYPT_KEY").unwrap_or_default();

    let dispatcher = EventDispatcherHandler::builder()
        .register_im_message_receive_v1(LoggingIm)
        .map_err(|e| format!("注册 typed handler 失败: {e}"))?
        .build();

    let inbound = HttpEventInbound::builder(verification_token, encrypt_key)
        .dispatcher(dispatcher)
        .build();

    let challenge_body =
        br#"{"challenge":"c-demo","token":"demo_token","type":"url_verification"}"#;
    let challenge_resp = inbound.handle(&HttpEventRequest::new(
        HashMap::new(),
        challenge_body.to_vec(),
    ))?;
    println!(
        "challenge 响应 status={} body={}",
        challenge_resp.status,
        String::from_utf8_lossy(&challenge_resp.body)
    );

    let event_body = br#"{"schema":"2.0","header":{"event_type":"im.message.receive_v1","token":"demo_token"},"event":{"message":{"message_id":"om_demo","content":"{\"text\":\"hi\"}"}}}"#;
    let event_resp = inbound.handle(&HttpEventRequest::new(HashMap::new(), event_body.to_vec()))?;
    println!(
        "事件响应 status={} body={}",
        event_resp.status,
        String::from_utf8_lossy(&event_resp.body)
    );

    println!("✅ HTTP 事件入站 fixture 完成（未发起真实网络请求）");
    Ok(())
}
