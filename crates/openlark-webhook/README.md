# openlark-webhook

[![crates.io](https://img.shields.io/crates/v/openlark-webhook)](https://crates.io/crates/openlark-webhook)
[![Documentation](https://docs.rs/openlark-webhook/badge.svg)](https://docs.rs/openlark-webhook)

飞书开放平台 Webhook 自定义机器人 API 模块。

## 功能特性

- ✅ **5 种消息类型**: 文本、富文本、图片、文件、交互式卡片
- ✅ **Builder 模式 API**: 流畅的链式调用体验
- ✅ **可选签名验证**: HMAC-SHA256 签名增强安全性
- ✅ **Feature flags**: 按需编译，轻量级
- ✅ **类型安全**: 编译时验证所有参数
- ✅ **完整测试覆盖**: 单元测试 + 集成测试

## 安装

```toml
[dependencies]
openlark-webhook = "0.15"
```

或通过根 crate 使用：

```toml
[dependencies]
openlark = { version = "0.15", features = ["webhook"] }
```

## 快速开始

### 发送文本消息

```rust,no_run
use openlark_webhook::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = SendWebhookMessageRequest::new(
        "https://open.feishu.cn/open-apis/bot/v2/hook/YOUR_WEBHOOK_KEY".to_string()
    )
    .text("Hello from OpenLark!".to_string())
    .execute()
    .await?;

    println!("Message sent: {:?}", response);
    Ok(())
}
```

### 使用 WebhookClient（推荐）

```rust,no_run
use openlark_webhook::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WebhookClient::new();
    let webhook_url = "https://open.feishu.cn/open-apis/bot/v2/hook/YOUR_WEBHOOK_KEY";

    // 发送文本消息
    let response = client
        .send_text(webhook_url, "Hello from WebhookClient!".to_string())
        .await?;
    println!("Text sent: {}", response.msg);

    // 发送图片消息
    let response = client
        .send_image(webhook_url, "img_xxxxxx".to_string())
        .await?;
    println!("Image sent: {}", response.msg);

    Ok(())
}
```

### 发送卡片消息

```rust,no_run
use openlark_webhook::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let card = json!({
        "elements": [
            {
                "tag": "div",
                "text": {
                    "content": "Hello from card!"
                }
            }
        ]
    });

    let response = SendWebhookMessageRequest::new(
        "https://open.feishu.cn/open-apis/bot/v2/hook/YOUR_WEBHOOK_KEY".to_string()
    )
    .card(card)
    .execute()
    .await?;

    println!("Card sent: {:?}", response);
    Ok(())
}
```

## Feature Flags

| Feature | 说明 | 默认 |
|---------|------|------|
| `robot` | 机器人消息发送功能 | ✅ 启用 |
| `signature` | 签名验证功能（需要 hmac, sha2, base64） | ❌ 禁用 |
| `card` | 卡片消息支持 | ❌ 禁用 |

### 启用所有功能

```toml
[dependencies]
openlark-webhook = { version = "0.15", features = ["robot", "signature", "card"] }
```

## 消息类型

### 1. 文本消息

```rust
.text("Hello, World!".to_string())
```

### 2. 富文本消息

```rust
.post(r#"{"title":"标题","content":[{"tag":"text","text":"内容"}]}"#.to_string())
```

### 3. 图片消息

```rust
.image("img_xxxxxx".to_string())
```

### 4. 文件消息

```rust
.file("file_xxxxxx".to_string())
```

### 5. 交互式卡片

```rust
.card(json!({...}))
```

## 签名验证

启用 `signature` feature 后，可以增强安全性：

```rust,no_run
use openlark_webhook::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webhook_url = "https://open.feishu.cn/open-apis/bot/v2/hook/YOUR_WEBHOOK_KEY".to_string();
    let secret = "YOUR_WEBHOOK_SECRET".to_string();

    // 使用 SendWebhookMessageRequest 发送带签名的消息
    let response = SendWebhookMessageRequest::new(webhook_url.clone())
        .text("Secure message with signature".to_string())
        .with_secret(secret.clone())
        .execute()
        .await?;

    println!("Signed message sent: {}", response.msg);

    // 使用 WebhookClient 发送带签名的消息
    let client = WebhookClient::new().with_secret(secret);
    let response = client
        .send_text(&webhook_url, "Another secure message".to_string())
        .await?;

    println!("Client signed message sent: {}", response.msg);
    Ok(())
}
```

签名头会自动添加到请求中：
- `X-Lark-Signature`: base64(hmac-sha256(timestamp + "\n" + secret))
- `X-Lark-Timestamp`: Unix 时间戳

## 错误处理

```rust,no_run
use openlark_webhook::prelude::*;

match SendWebhookMessageRequest::new(webhook_url)
    .text("Hello".to_string())
    .execute()
    .await
{
    Ok(response) => println!("Success: {:?}", response),
    Err(WebhookError::Http(msg)) => eprintln!("HTTP Error: {}", msg),
    Err(WebhookError::Serialization(msg)) => eprintln!("Serialization Error: {}", msg),
    Err(e) => eprintln!("Error: {:?}", e),
}
```

## 示例代码

完整示例请参阅 `examples/` 目录：

- `webhook_text_message.rs` - 基础文本消息
- `webhook_card_message.rs` - 卡片消息
- `webhook_with_signature.rs` - 使用 SendWebhookMessageRequest 带签名验证
- `webhook_client_with_signature.rs` - 使用 WebhookClient 带签名验证
- `webhook_error_handling.rs` - 错误处理

## API 文档

- [飞书开放平台 - 自定义机器人](https://open.feishu.cn/document/feishu-cards/quick-start/send-message-cards-with-custom-bot)
- [飞书开放平台 - 消息卡片](https://open.feishu.cn/document/feishu-cards/card-components/overview-of-message-cards)

## 许可证

[Apache-2.0](../../LICENSE-APACHE)
