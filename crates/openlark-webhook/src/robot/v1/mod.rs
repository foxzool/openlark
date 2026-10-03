//! 自定义机器人 v1 API。

/// Webhook 机器人客户端。
pub mod client;
/// Webhook 消息发送请求。
pub mod send;

/// Webhook 机器人客户端。
pub use client::WebhookClient;
/// Webhook 消息发送请求构建器。
pub use send::SendWebhookMessageRequest;
