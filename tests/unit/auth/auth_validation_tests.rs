use std::time::Duration;

use openlark_auth::auth::auth::v3::*;
use openlark_core::{
    api::responses::RawResponse,
    config::Config,
    error::{timeout_error, CoreError, ErrorTrait},
};

fn create_test_config(base_url: &str, timeout_ms: u64) -> Config {
    Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url(base_url)
        .req_timeout(Duration::from_millis(timeout_ms))
        .build()
}

fn macro_required_check(value: &str) -> openlark_core::SDKResult<()> {
    openlark_core::validate_required!(value, "字段不能为空");
    Ok(())
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_validate_required_empty_string() {
        assert!(openlark_core::Validatable::is_empty_trimmed(&""));
    }

    #[test]
    fn test_validate_required_whitespace_only() {
        assert!(openlark_core::Validatable::is_empty_trimmed(&"   \t\n"));
    }

    #[test]
    fn test_validate_required_non_empty_after_trim() {
        assert!(!openlark_core::Validatable::is_empty_trimmed(&"  app_123  "));
    }

    #[test]
    fn test_validate_required_special_characters_valid() {
        assert!(!openlark_core::Validatable::is_empty_trimmed(&"!@#$%^&*()"));
    }

    #[test]
    fn test_validate_required_unicode_valid() {
        assert!(!openlark_core::Validatable::is_empty_trimmed(&"应用ID-测试"));
    }

    #[test]
    fn test_validate_required_macro_empty_returns_validation_error() {
        let result = macro_required_check("");
        assert!(matches!(result, Err(CoreError::Validation { .. })));
        let err = result.expect_err("应返回验证错误");
        assert_eq!(err.to_string(), "验证错误 general: 字段不能为空");
    }

    #[test]
    fn test_validate_required_macro_whitespace_not_trimmed() {
        let result = macro_required_check("   ");
        assert!(result.is_ok());
    }

    #[test]
    fn test_api_error_response_parsing() {
        let raw: RawResponse = serde_json::from_str(
            r#"{
                "code": 99991663,
                "msg": "invalid app credentials",
                "request_id": "req_123"
            }"#,
        )
        .expect("错误响应 JSON 应可解析");

        assert_eq!(raw.code, 99991663);
        assert_eq!(raw.msg, "invalid app credentials");
        assert_eq!(raw.request_id.as_deref(), Some("req_123"));
        assert!(!raw.is_success());
    }

    #[tokio::test]
    async fn test_app_access_token_network_error_handling() {
        let config = create_test_config("http://nonexistent.invalid", 300);
        let service = AuthServiceV3::new(config);

        let result = service
            .app_access_token()
            .app_id("valid_app_id")
            .app_secret("valid_app_secret")
            .execute()
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_whitespace_app_id_not_blocked_by_macro_and_fails_on_network() {
        let config = create_test_config("http://nonexistent.invalid", 300);
        let service = AuthServiceV3::new(config);

        let result = service
            .app_access_token()
            .app_id("   ")
            .app_secret("valid_app_secret")
            .execute()
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_app_id_returns_validation_error_before_network() {
        let config = create_test_config("http://127.0.0.1:1", 300);
        let service = AuthServiceV3::new(config);

        let result = service
            .app_access_token()
            .app_id("")
            .app_secret("valid_app_secret")
            .execute()
            .await;

        assert!(matches!(result, Err(CoreError::Validation { .. })));
        let err = result.expect_err("应返回验证错误");
        assert!(err.to_string().contains("应用ID不能为空"));
    }

    #[tokio::test]
    async fn test_empty_app_secret_returns_validation_error_before_network() {
        let config = create_test_config("http://127.0.0.1:1", 300);
        let service = AuthServiceV3::new(config);

        let result = service
            .app_access_token()
            .app_id("valid_app_id")
            .app_secret("")
            .execute()
            .await;

        assert!(matches!(result, Err(CoreError::Validation { .. })));
        let err = result.expect_err("应返回验证错误");
        assert!(err.to_string().contains("应用密钥不能为空"));
    }

    #[test]
    fn test_timeout_error_handling() {
        let err = timeout_error(Duration::from_millis(50), Some("auth/token".to_string()));
        assert!(matches!(err, CoreError::Timeout { .. }));
        assert!(err.to_string().contains("auth/token"));
    }

    #[test]
    fn test_network_error_is_retryable() {
        let err = openlark_core::error::network_error("network down");
        assert!(err.is_retryable());
        assert_eq!(err.user_message(), Some("网络连接异常，请稍后重试"));
    }
}
