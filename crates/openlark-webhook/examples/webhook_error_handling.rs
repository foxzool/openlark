//! Webhook Error Handling Example
//!
//! This example demonstrates proper error handling when sending webhook messages.
//!
//! Run with:
//! ```bash
//! cargo run --example webhook_error_handling -p openlark-webhook
//! ```

use openlark_webhook::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let webhook_url = "https://open.feishu.cn/open-apis/bot/v2/hook/INVALID_KEY".to_string();

    match SendWebhookMessageRequest::new(webhook_url)
        .text("This message will fail".to_string())
        .execute()
        .await
    {
        Ok(response) => {
            println!("Message sent successfully!");
            println!("Response code: {}", response.code);
            println!("Response message: {}", response.msg);
        }
        Err(e) => {
            eprintln!("Failed to send message: {e}");
            eprintln!("Error type: {e:?}");
        }
    }

    Ok(())
}
