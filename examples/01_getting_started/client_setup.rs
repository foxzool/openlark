//! 客户端初始化与 helper 配置示例
//!
//! 演示如何创建 `Client`，并按需触发 communication helper。
//!
//! 运行方式：
//! ```bash
//! export OPENLARK_APP_ID="your_app_id"
//! export OPENLARK_APP_SECRET="your_app_secret"
//! cargo run --example client_setup --features "auth,communication"
//! ```

use open_lark::communication::endpoints::IM_V1_MESSAGES;
use open_lark::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("openlark SDK 客户端配置示例\n");

    // 从环境变量加载配置
    dotenvy::dotenv().ok();

    let app_id = std::env::var("OPENLARK_APP_ID").unwrap_or_else(|_| "cli_demo".to_string());
    let app_secret =
        std::env::var("OPENLARK_APP_SECRET").unwrap_or_else(|_| "demo_secret".to_string());

    println!("应用ID: {}", &app_id[..8.min(app_id.len())]);

    let client = Client::builder()
        .app_id(app_id)
        .app_secret(app_secret)
        .build()?;

    println!("✅ Client 创建成功");
    println!("认证入口已启用: {}", client.auth.app.config().app_id());

    // 访问 communication 模块的常量
    let endpoint = IM_V1_MESSAGES;
    println!("消息API端点: {endpoint}");

    if let Ok(user_name) = std::env::var("OPENLARK_USER_SEARCH_NAME") {
        if !user_name.trim().is_empty() {
            let user = client
                .communication
                .contact
                .find_user_by_name(&user_name)
                .await?;
            println!("👤 命中用户: {} ({})", user.name, user.open_id);
        } else {
            println!("OPENLARK_USER_SEARCH_NAME 为空，跳过用户查找 helper");
        }
    } else {
        println!("未设置 OPENLARK_USER_SEARCH_NAME，跳过用户查找 helper");
    }

    if let Ok(chat_name) = std::env::var("OPENLARK_CHAT_SEARCH_NAME") {
        if !chat_name.trim().is_empty() {
            let chat = client
                .communication
                .im
                .find_chat_by_name(&chat_name)
                .await?;
            println!("💬 命中群聊: {} ({})", chat.name, chat.chat_id);
        } else {
            println!("OPENLARK_CHAT_SEARCH_NAME 为空，跳过群聊查找 helper");
        }
    } else {
        println!("未设置 OPENLARK_CHAT_SEARCH_NAME，跳过群聊查找 helper");
    }

    // 构建消息请求
    let message = json!({
        "text": "Hello from openlark SDK!"
    });

    println!("\n📤 消息内容: {message}");
    println!("\n✅ 示例完成 - 这是一个客户端初始化 / helper 配置演示");
    println!(
        "   如果你想触发真实的 lookup 请求，请设置 OPENLARK_USER_SEARCH_NAME / OPENLARK_CHAT_SEARCH_NAME 后再运行本示例"
    );

    Ok(())
}
