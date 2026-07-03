//! `openlark-webhook` 常用类型预导出。

/// 核心错误与结果类型。
pub use openlark_core::{SDKResult, error::CoreError};

#[cfg(feature = "robot")]
/// 自定义机器人客户端。
pub use crate::robot::v1::client::WebhookClient;

#[cfg(feature = "robot")]
/// 发送 webhook 消息的请求构建器。
pub use crate::robot::v1::send::SendWebhookMessageRequest;

/// Webhook 模块错误与结果类型。
pub use crate::common::error::{Result, WebhookError};

#[cfg(feature = "signature")]
/// 签名校验辅助函数。
pub use crate::common::signature::verify_signature;
