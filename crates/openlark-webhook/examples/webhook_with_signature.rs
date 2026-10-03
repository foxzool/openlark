//! Webhook Signature Verification Example
//!
//! This example demonstrates how to verify webhook signatures using HMAC-SHA256.
//! Requires the "signature" feature to be enabled.
//!
//! Run with:
//! ```bash
//! cargo run --example webhook_with_signature -p openlark-webhook --features signature
//! ```

#[cfg(feature = "signature")]
use openlark_webhook::prelude::*;

#[cfg(feature = "signature")]
#[tokio::main]
async fn main() -> Result<()> {
    let webhook_url = "https://open.feishu.cn/open-apis/bot/v2/hook/YOUR_WEBHOOK_KEY".to_string();
    let secret = "YOUR_WEBHOOK_SECRET".to_string();

    let response = SendWebhookMessageRequest::new(webhook_url)
        .text("Message with signature verification".to_string())
        .with_secret(secret.clone())
        .execute()
        .await?;

    println!("Signed message sent successfully!");
    println!("Response code: {}", response.code);
    println!("Response message: {}", response.msg);
    println!("Secret used: {secret}");

    Ok(())
}

#[cfg(not(feature = "signature"))]
fn main() {
    println!("This example requires the 'signature' feature to be enabled.");
    println!(
        "Run with: cargo run --example webhook_with_signature -p openlark-webhook --features signature"
    );
}
