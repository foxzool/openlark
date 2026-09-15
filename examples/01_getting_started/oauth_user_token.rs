//! OAuth user access token 显式传入示例。
//!
//! OpenLark **不会**自动刷新用户 token。调用方拿到 user_access_token 后，必须经
//! `RequestOption::builder().user_access_token(...)` 显式传给
//! `execute_with_options`。本示例默认只打印构造好的 option，不发真实请求。
//!
//! ```bash
//! cargo run --example oauth_user_token --no-default-features --features "auth,communication"
//! ```

use open_lark::communication::im::v1::message::create::{CreateMessageBody, CreateMessageRequest};
use open_lark::communication::im::v1::message::models::ReceiveIdType;
use open_lark::prelude::*;
use open_lark::RequestOption;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_id = std::env::var("OPENLARK_APP_ID").unwrap_or_else(|_| "cli_demo".into());
    let app_secret = std::env::var("OPENLARK_APP_SECRET").unwrap_or_else(|_| "demo_secret".into());
    // 占位用户 token；真实场景由 OAuth 授权码交换 / 你自己的刷新逻辑提供
    let user_token =
        std::env::var("OPENLARK_USER_ACCESS_TOKEN").unwrap_or_else(|_| "u-demo-token".into());

    let client = Client::builder()
        .app_id(app_id)
        .app_secret(app_secret)
        .build()?;

    let option = RequestOption::builder()
        .user_access_token(user_token.clone())
        .build();
    println!(
        "✅ 已构造 RequestOption（user_access_token 前缀 {}...）",
        &user_token[..user_token.len().min(8)]
    );
    println!("说明: SDK 不会后台刷新该 token；过期后请自行换发并再次传入。");

    if std::env::var("OPENLARK_LIVE_USER_CALL").ok().as_deref() != Some("1") {
        let _ = (&client, &option);
        println!("未设置 OPENLARK_LIVE_USER_CALL=1，跳过真实 API 调用");
        return Ok(());
    }

    let receive_id = std::env::var("OPENLARK_RECEIVE_OPEN_ID")?;
    let body = CreateMessageBody {
        receive_id,
        msg_type: "text".into(),
        content: serde_json::json!({"text":"hello from user token"}).to_string(),
        uuid: None,
    };
    let request =
        CreateMessageRequest::new(client.config().clone()).receive_id_type(ReceiveIdType::OpenId);
    let resp = request.execute_with_options(body, option).await?;
    println!("发送结果: {resp}");
    Ok(())
}
