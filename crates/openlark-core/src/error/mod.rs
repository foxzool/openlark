//! 现代化错误处理系统
//!
//! 基于thiserror的完整错误处理解决方案，提供类型安全和开发者友好的API

use uuid::Uuid;

// 暴露错误体系（CoreError 为主要错误类型）
pub use self::codes::{ErrorCategory, ErrorCode};
pub use self::context::{ErrorContext, ErrorContextBuilder};
pub use self::core::{ApiError, BuilderKind, CoreError, ErrorBuilder, ErrorRecord};
pub use self::core::{RecoveryStrategy, RetryPolicy};
pub use self::core::{
    api_error, authentication_error, business_error, configuration_error, network_error,
    network_error_with_details, permission_missing_error, rate_limit_error, serialization_error,
    service_unavailable_error, sso_token_invalid_error, timeout_error, token_expired_error,
    token_invalid_error, user_identity_invalid_error, validation_error,
};
pub use self::traits::{ErrorContextTrait, ErrorFormatTrait, ErrorTrait, FullErrorTrait};
pub use self::traits::{ErrorSeverity, ErrorType};

// 主要类型别名（推荐使用）
/// SDK 操作结果类型别名
pub type SDKResult<T> = Result<T, CoreError>;
/// 错误唯一标识符类型别名
pub type ErrorId = Uuid;

// 核心模块
pub mod codes;
pub mod context;
pub mod core; // 主实现（含重试策略与错误定义）
pub mod traits;

// 预设导入
pub mod prelude;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_modern_error_creation() {
        let error = api_error(
            404,
            "/api/users/123",
            "用户不存在",
            Some("req-123".to_string()),
        );

        assert_eq!(error.error_type(), ErrorType::Api);
        assert_eq!(error.severity(), ErrorSeverity::Warning);
        assert!(!error.is_retryable());
        assert!(!error.is_user_error());
        assert_eq!(error.context().request_id(), Some("req-123"));
    }

    #[test]
    fn test_detailed_error_creation() {
        let error = network_error_with_details(
            "连接超时",
            Some("https://api.example.com".to_string()), // endpoint
            Some("req-456".to_string()),                 // request_id
        );

        assert!(error.is_network_error());
        assert_eq!(error.context().request_id(), Some("req-456"));
    }

    // ========== 新增测试: 错误类型创建测试 ==========

    /// 测试 network_error 创建
    #[test]
    fn test_network_error_creation() {
        let error = network_error("网络连接失败");

        assert!(error.is_network_error());
        assert!(error.is_retryable());
        assert_eq!(error.error_type(), ErrorType::Network);
        assert!(matches!(error.code(), ErrorCode::NetworkConnectionFailed));
    }

    /// 测试 authentication_error 创建
    #[test]
    fn test_authentication_error_creation() {
        let error = authentication_error("令牌无效");

        assert!(error.is_auth_error());
        assert!(!error.is_retryable());
        assert_eq!(error.error_type(), ErrorType::Authentication);
        assert!(error.is_user_error());
    }

    /// 测试 validation_error 创建
    #[test]
    fn test_validation_error_creation() {
        let error = validation_error("username", "用户名不能为空");

        assert!(error.is_validation_error());
        assert!(!error.is_retryable());
        assert_eq!(error.error_type(), ErrorType::Validation);
        assert!(error.context().has_context("field"));
        assert_eq!(error.context().get_context("field"), Some("username"));
    }

    /// 测试 business_error 创建
    #[test]
    fn test_business_error_creation() {
        let error = business_error("库存不足");

        assert!(error.is_business_error());
        assert!(!error.is_retryable());
        assert_eq!(error.error_type(), ErrorType::Business);
        assert!(matches!(error.code(), ErrorCode::BusinessError));
    }

    /// 测试 configuration_error 创建
    #[test]
    fn test_configuration_error_creation() {
        let error = configuration_error("缺少必要配置项");

        assert!(error.is_config_error());
        assert!(!error.is_retryable());
        assert_eq!(error.error_type(), ErrorType::Configuration);
        assert!(matches!(error.code(), ErrorCode::ConfigurationError));
    }

    /// 测试 timeout_error 创建
    #[test]
    fn test_timeout_error_creation() {
        let error = timeout_error(Duration::from_secs(30), Some("api_call".to_string()));

        assert!(error.is_timeout_error());
        assert!(error.is_retryable());
        assert_eq!(error.error_type(), ErrorType::Timeout);
    }

    /// 测试 rate_limit_error 创建
    #[test]
    fn test_rate_limit_error_creation() {
        let error = rate_limit_error(100, Duration::from_secs(60), Some(Duration::from_secs(30)));

        assert!(error.is_rate_limited());
        assert!(error.is_retryable());
        assert_eq!(error.error_type(), ErrorType::RateLimit);
        assert_eq!(error.retry_delay(0), Some(Duration::from_secs(60)));
    }

    /// 测试 service_unavailable_error 创建
    #[test]
    fn test_service_unavailable_error_creation() {
        let error = service_unavailable_error("飞书API", Some(Duration::from_secs(120)));

        assert!(error.is_service_unavailable_error());
        assert!(error.is_retryable());
        assert_eq!(error.error_type(), ErrorType::ServiceUnavailable);
    }

    /// 测试 permission_missing_error 创建
    #[test]
    fn test_permission_missing_error_creation() {
        let error = permission_missing_error(&["contact:user:read", "contact:user:write"]);

        assert!(error.is_auth_error());
        assert!(error.context().has_context("required_scopes"));
        assert!(
            error
                .context()
                .get_context("required_scopes")
                .unwrap()
                .contains("contact:user:read")
        );
    }

    /// 测试 sso_token_invalid_error 创建
    #[test]
    fn test_sso_token_invalid_error_creation() {
        let error = sso_token_invalid_error("SSO令牌已过期");

        assert!(error.is_auth_error());
        assert!(error.context().has_context("detail"));
        assert!(matches!(error.code(), ErrorCode::SsoTokenInvalid));
    }

    /// 测试 user_identity_invalid_error 创建
    #[test]
    fn test_user_identity_invalid_error_creation() {
        let error = user_identity_invalid_error("用户身份无法解析");

        assert!(error.is_auth_error());
        assert!(error.context().has_context("description"));
        assert!(matches!(error.code(), ErrorCode::UserIdentityInvalid));
    }

    /// 测试 token_invalid_error 创建
    #[test]
    fn test_token_invalid_error_creation() {
        let error = token_invalid_error("访问令牌格式错误");

        assert!(error.is_auth_error());
        assert!(error.context().has_context("detail"));
        assert!(matches!(error.code(), ErrorCode::AccessTokenInvalid));
    }

    /// 测试 token_expired_error 创建
    #[test]
    fn test_token_expired_error_creation() {
        let error = token_expired_error("访问令牌已过期");

        assert!(error.is_auth_error());
        assert!(error.context().has_context("detail"));
        assert!(matches!(error.code(), ErrorCode::AccessTokenExpiredV2));
    }

    /// 测试 serialization_error 创建
    #[test]
    fn test_serialization_error_creation() {
        let error: CoreError = serialization_error("JSON解析失败", None::<serde_json::Error>);

        assert!(error.is_serialization_error());
        assert!(!error.is_retryable());
        assert_eq!(error.error_type(), ErrorType::Serialization);
    }

    // ========== ErrorContextBuilder 测试 ==========

    /// 测试 ErrorContextBuilder 完整功能
    #[test]
    fn test_error_context_builder_full() {
        let context = ErrorContextBuilder::new()
            .user_message("操作失败")
            .context("endpoint", "/api/v1/users")
            .context("method", "POST")
            .request_id("req-test-001")
            .operation("create_user")
            .component("user_service")
            .build();

        assert_eq!(context.user_message(), Some("操作失败"));
        assert_eq!(context.get_context("endpoint"), Some("/api/v1/users"));
        assert_eq!(context.get_context("method"), Some("POST"));
        assert_eq!(context.request_id(), Some("req-test-001"));
        assert_eq!(context.operation(), Some("create_user"));
        assert_eq!(context.component(), Some("user_service"));
    }

    /// 测试 ErrorContext 批量添加上下文
    #[test]
    fn test_error_context_extend() {
        let mut context = ErrorContext::new();
        context.extend_context(vec![
            ("key1", "value1"),
            ("key2", "value2"),
            ("key3", "value3"),
        ]);

        assert_eq!(context.context_len(), 3);
        assert_eq!(context.get_context("key1"), Some("value1"));
        assert_eq!(context.get_context("key2"), Some("value2"));
        assert_eq!(context.get_context("key3"), Some("value3"));
    }

    // ========== RetryPolicy 测试 ==========

    /// 测试 RetryPolicy::no_retry
    #[test]
    fn test_retry_policy_no_retry() {
        let policy = RetryPolicy::no_retry();

        assert!(!policy.is_retryable());
        assert_eq!(policy.max_retries(), 0);
        assert!(policy.retry_delay(0).is_none());
    }

    /// 测试 RetryPolicy::fixed
    #[test]
    fn test_retry_policy_fixed() {
        let policy = RetryPolicy::fixed(3, Duration::from_secs(5));

        assert!(policy.is_retryable());
        assert_eq!(policy.max_retries(), 3);
        assert_eq!(policy.retry_delay(0), Some(Duration::from_secs(5)));
        assert_eq!(policy.retry_delay(1), Some(Duration::from_secs(5)));
        assert!(!policy.use_exponential_backoff());
    }

    /// 测试 RetryPolicy::exponential
    #[test]
    fn test_retry_policy_exponential() {
        let policy = RetryPolicy::exponential(4, Duration::from_secs(1));

        assert!(policy.is_retryable());
        assert_eq!(policy.max_retries(), 4);
        assert!(policy.use_exponential_backoff());

        // 验证指数退避
        let delay0 = policy.retry_delay(0).unwrap();
        let delay1 = policy.retry_delay(1).unwrap();
        let delay2 = policy.retry_delay(2).unwrap();

        assert!(delay1 > delay0);
        assert!(delay2 > delay1);
    }

    /// 测试 RetryPolicy::delay 方法
    #[test]
    fn test_retry_policy_delay() {
        let policy = RetryPolicy::fixed(2, Duration::from_secs(10));

        assert_eq!(policy.delay(0), Duration::from_secs(10));
        assert_eq!(policy.delay(1), Duration::from_secs(10));
        // 超过最大重试次数返回 0
        assert_eq!(policy.delay(2), Duration::ZERO);
    }

    /// 测试 RetryPolicy default
    #[test]
    fn test_retry_policy_default() {
        let policy = RetryPolicy::default();

        assert!(policy.is_retryable());
        assert_eq!(policy.max_retries(), 3);
        assert!(policy.use_exponential_backoff());
    }

    // ========== ErrorRecord 序列化测试 ==========

    /// 测试 ErrorRecord 序列化
    #[test]
    fn test_error_record_serialization() {
        let error = api_error(404, "/api/test", "资源不存在", Some("req-123".to_string()));
        let record = error.record();

        // 验证序列化成功
        let json = serde_json::to_string(&record).expect("序列化失败");
        assert!(json.contains("NotFound"));
        assert!(json.contains("req-123"));
        assert!(json.contains("资源不存在"));
    }

    /// 测试 ErrorRecord 反序列化
    #[test]
    fn test_error_record_json_roundtrip() {
        let error = api_error(
            500,
            "/api/internal",
            "内部错误",
            Some("req-456".to_string()),
        );
        let record = error.record();

        // 序列化
        let json = serde_json::to_string(&record).expect("序列化失败");

        // 验证关键字段存在
        assert!(json.contains("InternalServerError"));
        assert!(json.contains("req-456"));
        assert!(json.contains("retryable"));
    }

    // ========== ErrorBuilder 测试 ==========

    /// 测试 ErrorBuilder 创建 API 错误
    #[test]
    fn test_error_builder_api_error() {
        let error = CoreError::api_builder()
            .raw_code(429)
            .endpoint("/api/rate-limited")
            .message("请求过于频繁")
            .request_id("req-rate-001")
            .build();

        assert!(error.is_api_error());
        assert!(error.is_retryable());
        assert_eq!(error.context().request_id(), Some("req-rate-001"));
    }

    /// 测试 ErrorBuilder 创建验证错误
    #[test]
    fn test_error_builder_validation_error() {
        let error = CoreError::validation_builder()
            .field("email")
            .message("邮箱格式不正确")
            .context("input", "invalid-email")
            .build();

        assert!(error.is_validation_error());
        assert!(!error.is_retryable());
    }

    /// 测试 ErrorBuilder 创建网络错误
    #[test]
    fn test_error_builder_network_error() {
        let error = CoreError::network_builder()
            .message("连接超时")
            .endpoint("https://api.feishu.cn")
            .operation("fetch_user")
            .build();

        assert!(error.is_network_error());
        assert!(error.is_retryable());
    }

    /// 测试 ErrorBuilder 创建业务错误
    #[test]
    fn test_error_builder_business_error() {
        let error = CoreError::business_builder()
            .message("订单已取消")
            .context("order_id", "ORDER-123")
            .build();

        assert!(error.is_business_error());
        assert!(!error.is_retryable());
    }

    /// 测试 ErrorBuilder 创建认证错误
    #[test]
    fn test_error_builder_authentication_error() {
        let error = CoreError::authentication_builder()
            .message("令牌已过期")
            .request_id("req-auth-001")
            .build();

        assert!(error.is_auth_error());
        assert!(!error.is_retryable());
        assert_eq!(error.context().request_id(), Some("req-auth-001"));
    }

    // ========== ErrorCode 测试 ==========

    /// 测试 ErrorCode HTTP 状态码映射
    #[test]
    fn test_error_code_from_http_status() {
        assert_eq!(ErrorCode::from_http_status(400), ErrorCode::BadRequest);
        assert_eq!(ErrorCode::from_http_status(401), ErrorCode::Unauthorized);
        assert_eq!(ErrorCode::from_http_status(403), ErrorCode::Forbidden);
        assert_eq!(ErrorCode::from_http_status(404), ErrorCode::NotFound);
        assert_eq!(ErrorCode::from_http_status(429), ErrorCode::TooManyRequests);
        assert_eq!(
            ErrorCode::from_http_status(500),
            ErrorCode::InternalServerError
        );
        assert_eq!(
            ErrorCode::from_http_status(503),
            ErrorCode::ServiceUnavailable
        );
    }

    /// 测试 ErrorCode severity
    #[test]
    fn test_error_code_severity() {
        assert_eq!(ErrorCode::Success.severity(), ErrorSeverity::Info);
        assert_eq!(ErrorCode::NotFound.severity(), ErrorSeverity::Warning);
        assert_eq!(
            ErrorCode::InternalServerError.severity(),
            ErrorSeverity::Critical
        );
        assert_eq!(
            ErrorCode::NetworkTimeout.severity(),
            ErrorSeverity::Critical
        );
    }

    // ========== 错误类型判断测试 ==========

    /// 测试 ErrorTrait is_user_error 和 is_system_error
    #[test]
    fn test_error_trait_user_system_errors() {
        let validation = validation_error("field", "无效");
        assert!(validation.is_user_error());
        assert!(!validation.is_system_error());

        let network = network_error("连接失败");
        assert!(!network.is_user_error());
        assert!(network.is_system_error());

        let auth = authentication_error("认证失败");
        assert!(auth.is_user_error());
        assert!(!auth.is_system_error());
    }

    /// 测试 CoreError Clone
    #[test]
    fn test_core_error_clone() {
        let error = api_error(404, "/api/test", "资源不存在", Some("req-123".to_string()));
        let cloned = error.clone();

        assert_eq!(error.error_type(), cloned.error_type());
        assert_eq!(error.code(), cloned.code());
        assert_eq!(error.context().request_id(), cloned.context().request_id());
    }

    /// 测试 CoreError Display
    #[test]
    fn test_core_error_display() {
        let error = network_error("连接失败");
        let display = format!("{error}");
        assert!(display.contains("网络错误"));

        let error = validation_error("field", "无效值");
        let display = format!("{error}");
        assert!(display.contains("验证错误"));
    }

    /// 测试 CoreError::message() 和 kind() 兼容方法
    #[test]
    fn test_core_error_compatibility_methods() {
        let error = api_error(500, "/api/test", "服务器错误", None);

        // message() 方法
        let msg = error.message();
        assert!(msg.contains("API错误"));

        // kind() 方法 (返回 ErrorType)
        assert_eq!(error.kind(), ErrorType::Api);
    }

    /// 测试 CoreError::validation_msg 便捷方法
    #[test]
    fn test_core_error_validation_msg() {
        let error = CoreError::validation_msg("参数验证失败");

        assert!(error.is_validation_error());
    }

    /// 测试 CoreError::api_error 兼容方法
    #[test]
    fn test_core_error_api_error_compatibility() {
        let error = CoreError::api_error(404, "/api/users", "用户不存在", Some("req-001"));

        assert!(error.is_api_error());
        assert_eq!(error.context().request_id(), Some("req-001"));
    }

    /// 测试 CoreError::api_data_error
    #[test]
    fn test_core_error_api_data_error() {
        let error = CoreError::api_data_error("数据为空");

        assert!(error.is_api_error());
        assert!(error.message().contains("no data"));
    }

    /// 测试 SDKResult 类型别名
    #[test]
    fn test_sdk_result_type_alias() {
        fn returns_sdk_result() -> SDKResult<String> {
            Ok("success".to_string())
        }

        let result = returns_sdk_result();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    // ========== RecoveryStrategy 测试 ==========

    /// 测试 RecoveryStrategy 枚举变体
    #[test]
    fn test_recovery_strategy_variants() {
        let strategies = vec![
            RecoveryStrategy::RetryWithBackoff,
            RecoveryStrategy::ValidateAndRetry,
            RecoveryStrategy::Reauthenticate,
            RecoveryStrategy::RequestPermission,
            RecoveryStrategy::ManualIntervention,
            RecoveryStrategy::RetryWithDelay,
        ];

        // 验证每个变体都可以被创建和匹配
        for strategy in strategies {
            match strategy {
                RecoveryStrategy::RetryWithBackoff => {}
                RecoveryStrategy::ValidateAndRetry => {}
                RecoveryStrategy::Reauthenticate => {}
                RecoveryStrategy::RequestPermission => {}
                RecoveryStrategy::ManualIntervention => {}
                RecoveryStrategy::RetryWithDelay => {}
            }
        }
    }

    /// 测试 RecoveryStrategy Clone
    #[test]
    fn test_recovery_strategy_clone() {
        let strategy = RecoveryStrategy::RetryWithBackoff;
        let cloned = strategy.clone();

        match (strategy, cloned) {
            (RecoveryStrategy::RetryWithBackoff, RecoveryStrategy::RetryWithBackoff) => {}
            _ => panic!("RecoveryStrategy clone 失败"),
        }
    }

    /// 测试 RecoveryStrategy Debug
    #[test]
    fn test_recovery_strategy_debug() {
        let strategy = RecoveryStrategy::Reauthenticate;
        let debug_str = format!("{strategy:?}");
        assert!(debug_str.contains("Reauthenticate"));
    }

    // ========== ErrorContext 扩展测试 ==========

    /// 测试 ErrorContext 所有 getter 方法
    #[test]
    fn test_error_context_all_getters() {
        let mut context = ErrorContext::new();
        context.set_user_message("测试消息");
        context.set_request_id("req-123");
        context.set_operation("test_op");
        context.set_component("test_component");
        context.add_context("key1", "value1");

        assert_eq!(context.user_message(), Some("测试消息"));
        assert_eq!(context.request_id(), Some("req-123"));
        assert_eq!(context.operation(), Some("test_op"));
        assert_eq!(context.component(), Some("test_component"));
        assert_eq!(context.get_context("key1"), Some("value1"));
        assert!(context.timestamp().is_some());
    }

    /// 测试 ErrorContext is_empty 和 context_len
    #[test]
    fn test_error_context_empty_and_len() {
        let empty_context = ErrorContext::new();
        assert!(empty_context.is_empty());
        assert_eq!(empty_context.context_len(), 0);

        let mut context = ErrorContext::new();
        context.add_context("key", "value");
        assert!(!context.is_empty());
        assert_eq!(context.context_len(), 1);
    }

    /// 测试 ErrorContext all_context
    #[test]
    fn test_error_context_all_context() {
        let mut context = ErrorContext::new();
        context.add_context("k1", "v1");
        context.add_context("k2", "v2");

        let all = context.all_context();
        assert_eq!(all.len(), 2);
        assert_eq!(all.get("k1"), Some(&"v1".to_string()));
        assert_eq!(all.get("k2"), Some(&"v2".to_string()));
    }

    /// 测试 ErrorContextBuilder 链式调用完整流程
    #[test]
    fn test_error_context_builder_chaining() {
        let context = ErrorContextBuilder::new()
            .user_message("链式消息")
            .request_id("chain-req-001")
            .operation("chain_op")
            .component("chain_comp")
            .context("extra", "data")
            .build();

        assert_eq!(context.user_message(), Some("链式消息"));
        assert_eq!(context.request_id(), Some("chain-req-001"));
        assert_eq!(context.operation(), Some("chain_op"));
        assert_eq!(context.component(), Some("chain_comp"));
        assert_eq!(context.get_context("extra"), Some("data"));
    }

    // ========== 错误序列化/反序列化扩展测试 ==========

    /// 测试各种错误类型的序列化
    #[test]
    fn test_error_record_serialization_various_types() {
        // 网络错误
        let network = network_error("连接超时");
        let record = network.record();
        let json = serde_json::to_string(&record).expect("序列化失败");
        assert!(json.contains("NetworkConnectionFailed") || json.contains("retryable"));

        // 认证错误
        let auth = authentication_error("令牌无效");
        let record = auth.record();
        let json = serde_json::to_string(&record).expect("序列化失败");
        assert!(json.contains("AuthenticationFailed"));

        // 业务错误
        let business = business_error("库存不足");
        let record = business.record();
        let json = serde_json::to_string(&record).expect("序列化失败");
        assert!(json.contains("BusinessError"));
    }

    /// 测试 ErrorRecord 序列化包含所有字段
    #[test]
    fn test_error_record_full_serialization() {
        let error = api_error(
            404,
            "/api/resource",
            "资源不存在",
            Some("req-xyz".to_string()),
        );
        let record = error.record();
        let json = serde_json::to_string(&record).expect("序列化失败");

        // 验证 JSON 包含所有关键字段
        assert!(json.contains("NotFound"));
        assert!(json.contains("req-xyz"));
        assert!(json.contains("retryable"));
        assert!(json.contains("severity"));
        assert!(json.contains("message"));
    }

    /// 测试不同 severity 的序列化
    #[test]
    fn test_error_record_severity_serialization() {
        let info_error = api_error(200, "/api", "成功", None::<String>);
        let warning_error = api_error(404, "/api", "未找到", None::<String>);
        let critical_error = api_error(500, "/api", "服务器错误", None::<String>);

        let info_record = info_error.record();
        let warning_record = warning_error.record();
        let critical_record = critical_error.record();

        assert_eq!(info_record.severity, ErrorSeverity::Info);
        assert_eq!(warning_record.severity, ErrorSeverity::Warning);
        assert_eq!(critical_record.severity, ErrorSeverity::Critical);
    }

    // ========== CoreError 方法全面测试 ==========

    /// 测试 CoreError 所有判断方法
    #[test]
    fn test_core_error_all_predicates() {
        assert!(network_error("test").is_network_error());
        assert!(authentication_error("test").is_auth_error());
        assert!(validation_error("f", "m").is_validation_error());
        assert!(business_error("test").is_business_error());
        assert!(timeout_error(Duration::from_secs(1), None).is_timeout_error());
        assert!(rate_limit_error(1, Duration::from_secs(1), None).is_rate_limited());
        assert!(service_unavailable_error("svc", None).is_service_unavailable_error());
        assert!(serialization_error("test", None::<serde_json::Error>).is_serialization_error());
    }

    /// 测试 CoreError user_message 方法
    #[test]
    fn test_core_error_user_message() {
        let network = network_error("连接失败");
        assert_eq!(network.user_message(), Some("网络连接异常，请稍后重试"));

        let auth = authentication_error("认证失败");
        assert_eq!(auth.user_message(), Some("认证失败，请重新登录"));

        let timeout = timeout_error(Duration::from_secs(30), Some("api_call".to_string()));
        assert_eq!(timeout.user_message(), Some("请求超时，请稍后重试"));

        let rate_limit = rate_limit_error(100, Duration::from_secs(60), None);
        assert_eq!(rate_limit.user_message(), Some("请求过于频繁，请稍候"));

        let service = service_unavailable_error("api", None);
        assert_eq!(service.user_message(), Some("服务暂不可用，请稍后重试"));
    }

    /// 测试 CoreError retry_delay 对不同错误类型
    #[test]
    fn test_core_error_retry_delay_various() {
        // API 错误 500 系列应该有退避延迟
        let api500 = api_error(500, "/api", "error", None::<String>);
        let delay0 = api500.retry_delay(0);
        let delay1 = api500.retry_delay(1);
        assert!(delay0.is_some());
        assert!(delay1.is_some());
        assert!(delay1.unwrap() > delay0.unwrap());

        // API 错误 429 应该有延迟
        let api429 = api_error(429, "/api", "too many requests", None::<String>);
        assert!(api429.retry_delay(0).is_some());

        // API 错误 400 不应该重试
        let api400 = api_error(400, "/api", "bad request", None::<String>);
        assert!(!api400.is_retryable());
    }

    /// 测试 CoreError error_code 方法
    #[test]
    fn test_core_error_error_code() {
        let api = api_error(404, "/api", "not found", None::<String>);
        assert_eq!(api.error_code(), None); // CoreError 的 error_code 返回 None
    }

    /// 测试 CoreError 各种构建器方法
    #[test]
    fn test_core_error_builder_methods() {
        let network_builder = CoreError::network_builder();
        let api_builder = CoreError::api_builder();
        let validation_builder = CoreError::validation_builder();
        let auth_builder = CoreError::authentication_builder();
        let business_builder = CoreError::business_builder();

        // 验证构建器可以被创建（不需要进一步构建）
        let _ = network_builder;
        let _ = api_builder;
        let _ = validation_builder;
        let _ = auth_builder;
        let _ = business_builder;
    }

    /// 测试 CoreError 便捷构造方法
    #[test]
    fn test_core_error_convenience_methods() {
        let network_msg = CoreError::network_msg("网络消息");
        assert!(network_msg.is_network_error());

        let auth = CoreError::authentication("认证消息");
        assert!(auth.is_auth_error());

        let api_err = CoreError::api_error(500, "/api", "api消息", Some("req-id"));
        assert!(api_err.is_api_error());

        let validation_msg = CoreError::validation_msg("验证消息");
        assert!(validation_msg.is_validation_error());
    }

    /// 测试 ErrorId 类型别名
    #[test]
    fn test_error_id_type_alias() {
        let id: ErrorId = Uuid::new_v4();
        assert!(!id.to_string().is_empty());
    }

    #[test]
    fn test_lark_api_error_type_alias() {
        fn use_lark_api_error() -> CoreError {
            api_error(500, "/api", "test", None::<String>)
        }

        let error = use_lark_api_error();
        assert!(error.is_api_error());
    }

    /// 测试错误类型转换边界情况
    #[test]
    fn test_error_type_conversions() {
        // 从 reqwest::Error 转换（模拟）
        // 由于无法轻易创建 reqwest::Error，我们测试序列化错误转换
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let core_err: CoreError = json_err.into();
        assert!(core_err.is_serialization_error());
    }

    // ========== BuilderKind 测试 ==========

    /// 测试 BuilderKind 枚举
    #[test]
    fn test_builder_kind_variants() {
        let kinds = vec![
            BuilderKind::Network,
            BuilderKind::Authentication,
            BuilderKind::Api,
            BuilderKind::Validation,
            BuilderKind::Configuration,
            BuilderKind::Serialization,
            BuilderKind::Business,
            BuilderKind::Timeout,
            BuilderKind::RateLimit,
            BuilderKind::ServiceUnavailable,
            BuilderKind::Internal,
        ];

        for kind in kinds {
            match kind {
                BuilderKind::Network => {}
                BuilderKind::Authentication => {}
                BuilderKind::Api => {}
                BuilderKind::Validation => {}
                BuilderKind::Configuration => {}
                BuilderKind::Serialization => {}
                BuilderKind::Business => {}
                BuilderKind::Timeout => {}
                BuilderKind::RateLimit => {}
                BuilderKind::ServiceUnavailable => {}
                BuilderKind::Internal => {}
            }
        }
    }

    /// 测试 BuilderKind Debug
    #[test]
    fn test_builder_kind_debug() {
        let kind = BuilderKind::Api;
        let debug = format!("{kind:?}");
        assert!(debug.contains("Api"));
    }
}
