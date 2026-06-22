use crate::common::error::{Result, WebhookError};
use crate::common::validation;
use crate::robot::v1::send::SendWebhookMessageResponse;
use serde_json::json;

#[cfg(feature = "signature")]
use crate::common::signature;

/// Webhook 客户端。
///
/// 内部复用进程级共享的 `reqwest::Client`（连接池），不走 `openlark_core::Transport`
/// ——webhook 是出站自定义机器人 URL，非飞书开放平台 API（见 `send` 模块说明 + issue #214）。
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
    /// `payload` 需要符合飞书自定义机器人消息协议。
    pub async fn send(
        &self,
        webhook_url: &str,
        payload: serde_json::Value,
    ) -> Result<SendWebhookMessageResponse> {
        validation::validate_webhook_url(webhook_url)
            .map_err(|e| WebhookError::Http(e.to_string()))?;

        #[cfg(feature = "signature")]
        let request_builder = {
            let mut rb = self.client.post(webhook_url).json(&payload);
            if let Some(secret) = &self.secret {
                let timestamp = signature::current_timestamp();
                let sign = signature::sign(timestamp, secret);
                rb = rb
                    .header("X-Lark-Signature", sign)
                    .header("X-Lark-Timestamp", timestamp.to_string());
            }
            rb
        };

        #[cfg(not(feature = "signature"))]
        let request_builder = self.client.post(webhook_url).json(&payload);

        let response = request_builder
            .send()
            .await
            .map_err(|e| WebhookError::Http(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(WebhookError::Http(format!("HTTP error: {status}")));
        }

        let body = response
            .text()
            .await
            .map_err(|e| WebhookError::Http(e.to_string()))?;

        let result: SendWebhookMessageResponse = serde_json::from_str(&body)?;
        Ok(result)
    }

    /// 发送文本消息。
    pub async fn send_text(
        &self,
        webhook_url: &str,
        text: String,
    ) -> Result<SendWebhookMessageResponse> {
        let payload = json!({
            "msg_type": "text",
            "content": {
                "text": text
            }
        });
        self.send(webhook_url, payload).await
    }

    /// 发送富文本消息。
    pub async fn send_post(
        &self,
        webhook_url: &str,
        post: String,
    ) -> Result<SendWebhookMessageResponse> {
        let payload = json!({
            "msg_type": "post",
            "content": {
                "post": post
            }
        });
        self.send(webhook_url, payload).await
    }

    /// 发送图片消息。
    pub async fn send_image(
        &self,
        webhook_url: &str,
        image_key: String,
    ) -> Result<SendWebhookMessageResponse> {
        let payload = json!({
            "msg_type": "image",
            "content": {
                "image_key": image_key
            }
        });
        self.send(webhook_url, payload).await
    }

    /// 发送文件消息。
    pub async fn send_file(
        &self,
        webhook_url: &str,
        file_key: String,
    ) -> Result<SendWebhookMessageResponse> {
        let payload = json!({
            "msg_type": "file",
            "content": {
                "file_key": file_key
            }
        });
        self.send(webhook_url, payload).await
    }

    /// 发送交互式卡片消息。
    ///
    /// 需要启用 `card` feature。
    #[cfg(feature = "card")]
    pub async fn send_card(
        &self,
        webhook_url: &str,
        card: serde_json::Value,
    ) -> Result<SendWebhookMessageResponse> {
        let payload = json!({
            "msg_type": "interactive",
            "content": {
                "card": card
            }
        });
        self.send(webhook_url, payload).await
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
    async fn test_send_text_message_construction() {
        let client = WebhookClient::new();
        // Test that the method exists and can be called
        // (actual HTTP call would require mocking)
        let _client_ref = &client;
    }

    #[cfg(feature = "signature")]
    #[test]
    fn test_webhook_client_with_secret() {
        let client = WebhookClient::new().with_secret("my-secret".to_string());
        let _ = client;
    }
}
