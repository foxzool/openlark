use crate::common::error::{Result, WebhookError};
use crate::common::validation;
use crate::models::{FileContent, ImageContent, PostContent, TextContent};
use serde_json::json;
use std::sync::OnceLock;

#[cfg(feature = "signature")]
use crate::common::signature;

#[cfg(feature = "card")]
use crate::models::InteractiveContent;

/// 进程级共享的 `reqwest::Client`（连接池复用）。
///
/// # 为什么不走 `openlark_core::Transport`？
///
/// Webhook 自定义机器人**不是飞书开放平台 API**：目标 URL 是用户配置的绝对地址，
/// 鉴权用 URL 里携带的签名密钥（非 Bearer token），响应体是 `{code,msg}` 等非标准
/// 包装（非 `{code,msg,data}`）。`Transport` 固定 `/open-apis/` 基址、强制 token 注入、
/// 且把响应解析为 `ApiResponse<R>`，三者都不适用。因此 webhook **有意保留独立的
/// reqwest 路径**（见 GitHub issue #214 的调研结论），但通过共享单个 `reqwest::Client`
/// 避免每个请求 `reqwest::Client::new()` 新建连接池的开销。
///
/// 这是 `Transport` 边界的 **by-design 例外**——架构约定与白名单见
/// `ARCHITECTURE.md`「Transport HTTP 边界」小节，并由
/// `tools/check_reqwest_boundary.sh` 守卫（#270）。
pub(super) fn shared_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(reqwest::Client::new)
}

/// 共享的 validate / sign / POST / deserialize 管道。
///
/// `SendWebhookMessageRequest::execute` 与 `WebhookClient::send` 复用此 helper，
/// 消除两份逐字重复的发送管道（#310）。
pub(super) async fn post_payload(
    client: &reqwest::Client,
    webhook_url: &str,
    payload: serde_json::Value,
    #[cfg(feature = "signature")] secret: Option<&str>,
) -> Result<SendWebhookMessageResponse> {
    validation::validate_webhook_url(webhook_url).map_err(|e| WebhookError::Http(e.to_string()))?;

    #[cfg(feature = "signature")]
    let request_builder = {
        let mut rb = client.post(webhook_url).json(&payload);
        if let Some(secret) = secret {
            let timestamp = signature::current_timestamp();
            let sign = signature::sign(timestamp, secret);
            rb = rb
                .header("X-Lark-Signature", sign)
                .header("X-Lark-Timestamp", timestamp.to_string());
        }
        rb
    };

    #[cfg(not(feature = "signature"))]
    let request_builder = client.post(webhook_url).json(&payload);

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

/// 发送 Webhook 消息请求构建器。
#[derive(Debug, Clone)]
pub struct SendWebhookMessageRequest {
    webhook_url: String,
    msg_type: String,
    content: serde_json::Value,
    /// raw 模式：直接作为完整 payload 发送（跳过 `{msg_type, content}` 包装）。
    raw_payload: Option<serde_json::Value>,
    /// 注入的 HTTP client（None = 用 `shared_client()`）。
    client: Option<reqwest::Client>,
    #[cfg(feature = "signature")]
    secret: Option<String>,
}

impl SendWebhookMessageRequest {
    /// 创建新的发送请求
    pub fn new(webhook_url: String) -> Self {
        Self {
            webhook_url,
            msg_type: "text".to_string(),
            content: json!({}),
            raw_payload: None,
            client: None,
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

    /// 设置原始 payload（跳过 `{msg_type, content}` 包装，直接发送完整 payload）。
    ///
    /// 用于 `WebhookClient::send` 等需要完全自定义 payload 的场景。
    pub fn raw(mut self, payload: serde_json::Value) -> Self {
        self.raw_payload = Some(payload);
        self
    }

    /// 注入自定义 HTTP client（覆盖进程级 `shared_client`）。
    ///
    /// 允许配置连接池、超时等。不设置则用 `shared_client()`。
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    /// 将请求内容设置为文本消息。
    pub fn text(mut self, text: String) -> Self {
        self.msg_type = "text".to_string();
        self.content = serde_json::to_value(TextContent::new(text)).unwrap_or_else(|_| json!({}));
        self
    }

    /// 将请求内容设置为富文本消息。
    pub fn post(mut self, post: String) -> Self {
        self.msg_type = "post".to_string();
        self.content = serde_json::to_value(PostContent::new(post)).unwrap_or_else(|_| json!({}));
        self
    }

    /// 将请求内容设置为图片消息。
    pub fn image(mut self, image_key: String) -> Self {
        self.msg_type = "image".to_string();
        self.content =
            serde_json::to_value(ImageContent::new(image_key)).unwrap_or_else(|_| json!({}));
        self
    }

    /// 将请求内容设置为文件消息。
    pub fn file(mut self, file_key: String) -> Self {
        self.msg_type = "file".to_string();
        self.content =
            serde_json::to_value(FileContent::new(file_key)).unwrap_or_else(|_| json!({}));
        self
    }

    /// 将请求内容设置为交互式卡片消息。
    ///
    /// 需要启用 `card` feature。
    #[cfg(feature = "card")]
    pub fn card(mut self, card: serde_json::Value) -> Self {
        self.msg_type = "interactive".to_string();
        self.content =
            serde_json::to_value(InteractiveContent::new(card)).unwrap_or_else(|_| json!({}));
        self
    }

    /// 执行发送请求并返回飞书响应。
    pub async fn execute(self) -> Result<SendWebhookMessageResponse> {
        let payload = if let Some(raw) = self.raw_payload {
            raw
        } else {
            json!({
                "msg_type": self.msg_type,
                "content": self.content,
            })
        };
        let client: &reqwest::Client = self.client.as_ref().unwrap_or_else(|| shared_client());
        #[cfg(feature = "signature")]
        {
            post_payload(client, &self.webhook_url, payload, self.secret.as_deref()).await
        }
        #[cfg(not(feature = "signature"))]
        {
            post_payload(client, &self.webhook_url, payload).await
        }
    }
}

/// Webhook 消息发送响应
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SendWebhookMessageResponse {
    /// 返回码
    pub code: i32,
    /// 返回信息
    pub msg: String,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_send_webhook_message_request_text() {
        let req = SendWebhookMessageRequest::new("https://example.com/webhook".to_string())
            .text("Hello, World!".to_string());

        assert_eq!(req.msg_type, "text");
        assert_eq!(req.webhook_url, "https://example.com/webhook");
    }

    #[test]
    fn test_send_webhook_message_request_post() {
        let req = SendWebhookMessageRequest::new("https://example.com/webhook".to_string())
            .post(r#"{"title":"Test"}"#.to_string());

        assert_eq!(req.msg_type, "post");
    }

    #[test]
    fn test_send_webhook_message_request_image() {
        let req = SendWebhookMessageRequest::new("https://example.com/webhook".to_string())
            .image("img_abc123".to_string());

        assert_eq!(req.msg_type, "image");
    }

    #[test]
    fn test_send_webhook_message_request_file() {
        let req = SendWebhookMessageRequest::new("https://example.com/webhook".to_string())
            .file("file_xyz789".to_string());

        assert_eq!(req.msg_type, "file");
    }

    #[cfg(feature = "card")]
    #[test]
    fn test_send_webhook_message_request_card() {
        let card = serde_json::json!({
            "type": "template",
            "data": {
                "template_id": "test_template"
            }
        });
        let req =
            SendWebhookMessageRequest::new("https://example.com/webhook".to_string()).card(card);

        assert_eq!(req.msg_type, "interactive");
    }

    #[test]
    fn test_send_webhook_message_response_serialization() {
        let json = r#"{"code":0,"msg":"ok"}"#;
        let response: SendWebhookMessageResponse =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(response.code, 0);
        assert_eq!(response.msg, "ok");
    }

    #[cfg(feature = "signature")]
    #[test]
    fn test_send_webhook_message_request_with_secret() {
        let req = SendWebhookMessageRequest::new("https://example.com/webhook".to_string())
            .text("Hello".to_string())
            .with_secret("my-secret".to_string());

        assert!(req.secret.is_some());
        assert_eq!(req.secret.unwrap(), "my-secret");
    }
}
