use crate::common::error::Result;
use crate::robot::v1::send::{SendWebhookMessageRequest, SendWebhookMessageResponse};

/// Webhook 客户端。
///
/// 内部复用进程级共享的 `reqwest::Client`（连接池），不走 `openlark_core::Transport`
/// ——webhook 是出站自定义机器人 URL，非飞书开放平台 API（见 `send` 模块说明 + issue #214）。
///
/// 这是 `Transport` 边界的 **by-design 例外**（白名单见 `ARCHITECTURE.md`
/// 「Transport HTTP 边界」小节，#270）。
///
/// `WebhookClient` 是 `SendWebhookMessageRequest` 的薄 wrapper：`send` 委托
/// `Request::raw + with_client + execute`，复用共享发送管道（#310）。
#[derive(Debug, Clone)]
pub struct WebhookClient {
    client: reqwest::Client,
    #[cfg(feature = "signature")]
    secret: Option<String>,
}

impl WebhookClient {
    /// 创建新的 Webhook 客户端（复用进程级共享 HTTP 连接池）
    pub fn new() -> Self {
        Self {
            client: super::send::shared_client().clone(),
            #[cfg(feature = "signature")]
            secret: None,
        }
    }

    /// 使用自定义 HTTP 客户端创建 Webhook 客户端
    ///
    /// 允许配置连接池、超时等参数：
    /// ```rust,no_run
    /// use openlark_webhook::prelude::*;
    ///
    /// let http_client = reqwest::Client::builder()
    ///     .timeout(std::time::Duration::from_secs(30))
    ///     .pool_max_idle_per_host(10)
    ///     .build()
    ///     .expect("Failed to build HTTP client");
    ///
    /// let client = WebhookClient::with_client(http_client);
    /// ```
    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            client,
            #[cfg(feature = "signature")]
            secret: None,
        }
    }

    /// 设置签名密钥（启用签名验证）
    #[cfg(feature = "signature")]
    pub fn with_secret(mut self, secret: String) -> Self {
        self.secret = Some(secret);
        self
    }

    /// 发送原始 JSON 负载到指定 webhook。
    ///
    /// `payload` 需要符合飞书自定义机器人消息协议。委托 `SendWebhookMessageRequest::raw`
    /// + `with_client` + `execute`（共享发送管道，#310）。
    ///
    /// 如需 typed 消息（text/post/image/file/card），用 `SendWebhookMessageRequest` 的
    /// `.text()` / `.post()` / `.image()` / `.file()` / `.card()` 构造器。
    pub async fn send(
        &self,
        webhook_url: &str,
        payload: serde_json::Value,
    ) -> Result<SendWebhookMessageResponse> {
        let mut req = SendWebhookMessageRequest::new(webhook_url.to_string())
            .raw(payload)
            .with_client(self.client.clone());
        #[cfg(feature = "signature")]
        if let Some(secret) = &self.secret {
            req = req.with_secret(secret.clone());
        }
        req.execute().await
    }
}

impl Default for WebhookClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_client_creation() {
        let _client = WebhookClient::new();
        let _default_client = WebhookClient::default();
    }

    #[tokio::test]
    async fn test_webhook_client_send_construction() {
        // 验证 send 方法存在且可调用（实际 HTTP 需 mock）
        let client = WebhookClient::new();
        let _client_ref = &client;
    }

    #[cfg(feature = "signature")]
    #[test]
    fn test_webhook_client_with_secret() {
        let client = WebhookClient::new().with_secret("my-secret".to_string());
        let _ = client;
    }
}
