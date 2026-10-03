//! Webhook 模块错误类型。

use thiserror::Error;

/// Webhook 调用过程中可能出现的错误。
#[derive(Debug, Error)]
pub enum WebhookError {
    /// HTTP 请求发送或服务端返回失败。
    #[error("HTTP error: {0}")]
    Http(String),

    /// 请求或响应 JSON 序列化失败。
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// 签名校验失败。
    #[error("Invalid signature")]
    InvalidSignature,

    /// 缺少必填字段。
    #[error("Missing required field: {0}")]
    MissingField(String),
}

impl PartialEq for WebhookError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Http(a), Self::Http(b)) => a == b,
            (Self::Serialization(_), Self::Serialization(_)) => true,
            (Self::InvalidSignature, Self::InvalidSignature) => true,
            (Self::MissingField(a), Self::MissingField(b)) => a == b,
            _ => false,
        }
    }
}

/// Webhook 模块统一结果类型。
pub type Result<T> = std::result::Result<T, WebhookError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_error_display_http() {
        let err = WebhookError::Http("connection refused".to_string());
        assert_eq!(err.to_string(), "HTTP error: connection refused");
    }

    #[test]
    fn test_webhook_error_display_serialization() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let err = WebhookError::Serialization(json_err);
        assert!(err.to_string().starts_with("Serialization error:"));
    }

    #[test]
    fn test_webhook_error_display_invalid_signature() {
        let err = WebhookError::InvalidSignature;
        assert_eq!(err.to_string(), "Invalid signature");
    }

    #[test]
    fn test_webhook_error_display_missing_field() {
        let err = WebhookError::MissingField("webhook_url".to_string());
        assert_eq!(err.to_string(), "Missing required field: webhook_url");
    }

    #[test]
    fn test_webhook_error_debug() {
        let err = WebhookError::InvalidSignature;
        let debug_str = format!("{err:?}");
        assert!(debug_str.contains("InvalidSignature"));
    }

    #[test]
    fn test_result_type() {
        let ok_result: Result<i32> = Ok(42);
        assert_eq!(ok_result, Ok(42));

        let err_result: Result<i32> = Err(WebhookError::InvalidSignature);
        assert!(err_result.is_err());
    }
}
