//! WebSocket typed 事件 handler 示例。
//!
//! 演示 `register_im_message_receive_v1`。默认不建立长连接、不访问飞书；
//! 仅用 fixture JSON 走 `EventDispatcherHandler`。设置 `OPENLARK_WS_CONNECT=1`
//! 且提供真实 `OPENLARK_APP_ID` / `OPENLARK_APP_SECRET` 时才会调用 `LarkWsClient::open`。
//!
//! ```bash
//! cargo run --example websocket_typed_handler --no-default-features --features "communication,websocket"
//! ```

use std::sync::Arc;

use open_lark::Config;
use open_lark::ws_client::{
    EventDispatcherHandler, ImMessageReceiveV1, ImMessageReceiveV1Handler, LarkWsClient,
};

struct PrintIm;

impl ImMessageReceiveV1Handler for PrintIm {
    fn handle(
        &self,
        event: ImMessageReceiveV1,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!(
            "typed im.message.receive_v1 message_id={} chat_id={}",
            event.event.message.message_id, event.event.message.chat_id
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let fixture = br#"{"schema":"2.0","header":{"event_type":"im.message.receive_v1"},"event":{"message":{"message_id":"om_typed_demo","chat_id":"oc_demo","message_type":"text","content":"{\"text\":\"hello\"}"}}}"#;
    let handler = EventDispatcherHandler::builder()
        .register_im_message_receive_v1(PrintIm)
        .map_err(|e| format!("注册失败: {e}"))?
        .build();
    handler.do_without_validation(fixture)?;
    println!("✅ typed fixture 分发完成");

    if std::env::var("OPENLARK_WS_CONNECT").ok().as_deref() != Some("1") {
        println!("未设置 OPENLARK_WS_CONNECT=1，跳过真实长连接");
        return Ok(());
    }

    let app_id = std::env::var("OPENLARK_APP_ID")?;
    let app_secret = std::env::var("OPENLARK_APP_SECRET")?;
    let config = Config::builder()
        .app_id(app_id)
        .app_secret(app_secret)
        .build();
    println!("🔌 正在建立长连接...");
    LarkWsClient::open(Arc::new(config), handler).await?;
    Ok(())
}
