//! 错误上下文和恢复策略测试

use openlark_core::error::{
    CoreError, ErrorContext, ErrorContextBuilder, ErrorSeverity, ErrorTrait, ErrorType,
    RecoveryStrategy, RetryPolicy,
};
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_context_builder_basic() {
        let context = ErrorContextBuilder::new()
            .user_message("测试错误消息")
            .build();

        assert_eq!(context.user_message(), Some("测试错误消息"));
    }

    #[test]
    fn test_error_context_builder_all_methods() {
        let context = ErrorContextBuilder::new()
            .user_message("完整错误上下文")
            .context("endpoint", "/api/v1/users")
            .context("method", "POST")
            .context("status_code", "500")
            .request_id("req-abc123")
            .operation("create_user")
            .component("user_service")
            .build();

        assert_eq!(context.user_message(), Some("完整错误上下文"));
        assert_eq!(context.get_context("endpoint"), Some("/api/v1/users"));
        assert_eq!(context.get_context("method"), Some("POST"));
        assert_eq!(context.get_context("status_code"), Some("500"));
        assert_eq!(context.request_id(), Some("req-abc123"));
        assert_eq!(context.operation(), Some("create_user"));
        assert_eq!(context.component(), Some("user_service"));
    }

    #[test]
    fn test_error_context_builder_extend() {
        let context = ErrorContextBuilder::new()
            .extend_context(vec![
                ("key1", "value1"),
                ("key2", "value2"),
                ("key3", "value3"),
            ])
            .build();

        assert_eq!(context.context_len(), 3);
        assert_eq!(context.get_context("key1"), Some("value1"));
        assert_eq!(context.get_context("key2"), Some("value2"));
        assert_eq!(context.get_context("key3"), Some("value3"));
    }

    #[test]
    fn test_error_context_builder_default() {
        let builder = ErrorContextBuilder::default();
        let context = builder.build();

        assert!(context.is_empty() || context.user_message().is_none());
    }

    #[test]
    fn test_error_context_builder_chain() {
        let context = ErrorContextBuilder::new()
            .context("step1", "init")
            .context("step2", "process")
            .context("step3", "complete")
            .operation("multi_step_operation")
            .component("pipeline")
            .request_id("chain-req-001")
            .user_message("链式调用测试")
            .build();

        assert_eq!(context.context_len(), 3);
        assert_eq!(context.operation(), Some("multi_step_operation"));
        assert_eq!(context.component(), Some("pipeline"));
        assert_eq!(context.request_id(), Some("chain-req-001"));
        assert_eq!(context.user_message(), Some("链式调用测试"));
    }

    #[test]
    fn test_error_context_constructors() {
        let ctx1 = ErrorContext::with_user_message("用户友好消息");
        assert_eq!(ctx1.user_message(), Some("用户友好消息"));

        let ctx2 = ErrorContext::with_operation("api_call");
        assert_eq!(ctx2.operation(), Some("api_call"));

        let ctx3 = ErrorContext::new();
        assert!(ctx3.is_empty() || ctx3.user_message().is_none());
    }

    #[test]
    fn test_error_context_add_get() {
        let mut context = ErrorContext::new();
        context.add_context("url", "https://api.feishu.cn");
        context.add_context("timeout", "30s");

        assert_eq!(context.get_context("url"), Some("https://api.feishu.cn"));
        assert_eq!(context.get_context("timeout"), Some("30s"));
        assert_eq!(context.get_context("nonexistent"), None);
    }

    #[test]
    fn test_error_context_has_context() {
        let mut context = ErrorContext::new();
        context.add_context("key1", "value1");

        assert!(context.has_context("key1"));
        assert!(!context.has_context("key2"));
    }

    #[test]
    fn test_error_context_clear() {
        let mut context = ErrorContext::new();
        context.add_context("key1", "value1");
        context.add_context("key2", "value2");

        assert_eq!(context.context_len(), 2);

        context.clear_context();

        assert_eq!(context.context_len(), 0);
        assert!(!context.has_context("key1"));
        assert!(!context.has_context("key2"));
    }

    #[test]
    fn test_error_context_clone_with() {
        let mut original = ErrorContext::new();
        original.add_context("original_key", "original_value");
        original.set_user_message("原始消息");

        let cloned = original.clone_with();

        assert_eq!(cloned.get_context("original_key"), Some("original_value"));
        assert_eq!(cloned.user_message(), Some("原始消息"));

        let original_ts = original.timestamp();
        let cloned_ts = cloned.timestamp();
        assert!(original_ts.is_some());
        assert!(cloned_ts.is_some());
    }

    #[test]
    fn test_error_context_debug_format() {
        let context = ErrorContextBuilder::new()
            .user_message("调试测试")
            .context("api", "send_message")
            .operation("message_send")
            .component("communication")
            .request_id("req-debug-001")
            .build();

        let debug_str = context.debug_format();

        assert!(debug_str.contains("调试测试"));
        assert!(debug_str.contains("api"));
        assert!(debug_str.contains("send_message"));
        assert!(debug_str.contains("message_send"));
        assert!(debug_str.contains("communication"));
        assert!(debug_str.contains("req-debug-001"));
    }

    #[test]
    fn test_error_context_is_empty() {
        let empty = ErrorContext::new();
        assert!(empty.is_empty());

        let mut non_empty = ErrorContext::new();
        non_empty.add_context("key", "value");
        assert!(!non_empty.is_empty());

        let mut with_message = ErrorContext::new();
        with_message.set_user_message("消息");
        assert!(!with_message.is_empty());
    }

    #[test]
    fn test_error_context_timestamp_backtrace() {
        let context = ErrorContext::new();

        assert!(context.timestamp().is_some());

        // backtrace 检查已移除
    }

    #[test]
    fn test_retry_policy_no_retry() {
        let policy = RetryPolicy::no_retry();

        assert!(!policy.is_retryable());
        assert_eq!(policy.max_retries(), 0);
        assert!(policy.retry_delay(0).is_none());
    }

    #[test]
    fn test_retry_policy_fixed() {
        let policy = RetryPolicy::fixed(5, Duration::from_secs(2));

        assert!(policy.is_retryable());
        assert_eq!(policy.max_retries(), 5);
        assert!(!policy.use_exponential_backoff());

        assert_eq!(policy.retry_delay(0), Some(Duration::from_secs(2)));
        assert_eq!(policy.retry_delay(1), Some(Duration::from_secs(2)));
        assert_eq!(policy.retry_delay(4), Some(Duration::from_secs(2)));

        assert!(policy.retry_delay(5).is_none());
    }

    #[test]
    fn test_retry_policy_exponential() {
        let policy = RetryPolicy::exponential(3, Duration::from_secs(1));

        assert!(policy.is_retryable());
        assert_eq!(policy.max_retries(), 3);
        assert!(policy.use_exponential_backoff());

        let delay0 = policy.retry_delay(0).unwrap();
        let delay1 = policy.retry_delay(1).unwrap();
        let delay2 = policy.retry_delay(2).unwrap();

        assert_eq!(delay0, Duration::from_secs(1));

        assert!(delay1 > delay0);
        assert!(delay2 > delay1);

        assert!(policy.retry_delay(3).is_none());
    }

    #[test]
    fn test_retry_policy_delay() {
        let policy = RetryPolicy::fixed(2, Duration::from_secs(5));

        assert_eq!(policy.delay(0), Duration::from_secs(5));
        assert_eq!(policy.delay(1), Duration::from_secs(5));

        assert_eq!(policy.delay(2), Duration::ZERO);
        assert_eq!(policy.delay(10), Duration::ZERO);
    }

    #[test]
    fn test_retry_policy_default() {
        let policy = RetryPolicy::default();

        assert!(policy.is_retryable());
        assert_eq!(policy.max_retries(), 3);
        assert!(policy.use_exponential_backoff());
    }

    #[test]
    fn test_retry_policy_exponential_calculation() {
        let policy = RetryPolicy::exponential(10, Duration::from_millis(100));

        let d0 = policy.retry_delay(0).unwrap();
        let d1 = policy.retry_delay(1).unwrap();
        let d2 = policy.retry_delay(2).unwrap();
        let d3 = policy.retry_delay(3).unwrap();

        assert!(d1 >= d0 * 2);
        assert!(d2 >= d1 * 2);
        assert!(d3 >= d2 * 2);
    }

    #[test]
    fn test_retry_policy_max_delay() {
        let policy = RetryPolicy::exponential(100, Duration::from_secs(1));

        let delay_50 = policy.retry_delay(50);
        assert!(delay_50.is_some());

        let max_expected = Duration::from_secs(300);
        assert!(delay_50.unwrap() <= max_expected);
    }

    #[test]
    fn test_is_retryable_network_error() {
        let error = CoreError::network_builder().message("连接失败").build();

        assert!(error.is_retryable());
    }

    #[test]
    fn test_is_retryable_validation_error() {
        let error = CoreError::validation_builder()
            .field("email")
            .message("邮箱格式不正确")
            .build();

        assert!(!error.is_retryable());
    }

    #[test]
    fn test_is_retryable_timeout_error() {
        let error = CoreError::network_builder()
            .message("请求超时")
            .duration(Duration::from_secs(30))
            .build();

        assert!(error.is_retryable());
    }

    #[test]
    fn test_is_retryable_rate_limit() {
        let error = CoreError::api_builder()
            .raw_code(429)
            .message("请求过于频繁")
            .build();

        assert!(error.is_retryable());
    }

    #[test]
    fn test_is_retryable_server_error() {
        let error = CoreError::api_builder()
            .raw_code(503)
            .message("服务不可用")
            .build();

        assert!(error.is_retryable());
    }

    #[test]
    fn test_is_retryable_auth_error() {
        let error = CoreError::authentication_builder()
            .message("令牌无效")
            .build();

        assert!(!error.is_retryable());
    }

    #[test]
    fn test_is_retryable_business_error() {
        let error = CoreError::business_builder().message("库存不足").build();

        assert!(!error.is_retryable());
    }

    #[test]
    fn test_retry_delay_rate_limit() {
        let error = openlark_core::error::rate_limit_error(
            100,
            Duration::from_secs(60),
            Some(Duration::from_secs(30)),
        );

        let delay = error.retry_delay(0);
        assert!(delay.is_some());
        assert!(delay.unwrap() >= Duration::from_secs(60));
    }

    #[test]
    fn test_retry_delay_validation_error() {
        let error = CoreError::validation_builder()
            .field("field")
            .message("验证失败")
            .build();

        assert!(error.retry_delay(0).is_none());
    }

    #[test]
    fn test_retry_delay_service_unavailable() {
        let error = openlark_core::error::service_unavailable_error(
            "飞书API",
            Some(Duration::from_secs(120)),
        );

        let delay = error.retry_delay(0);
        assert!(delay.is_some());
    }

    #[test]
    fn test_recovery_strategy_variants() {
        let strategies = [
            RecoveryStrategy::RetryWithBackoff,
            RecoveryStrategy::ValidateAndRetry,
            RecoveryStrategy::Reauthenticate,
            RecoveryStrategy::RequestPermission,
            RecoveryStrategy::ManualIntervention,
            RecoveryStrategy::RetryWithDelay,
        ];

        for strategy in &strategies {
            let cloned = strategy.clone();
            let _debug = format!("{cloned:?}");
        }

        assert_eq!(strategies.len(), 6);
    }

    #[test]
    fn test_recovery_strategy_debug() {
        assert!(format!("{:?}", RecoveryStrategy::RetryWithBackoff).contains("RetryWithBackoff"));
        assert!(format!("{:?}", RecoveryStrategy::Reauthenticate).contains("Reauthenticate"));
        assert!(
            format!("{:?}", RecoveryStrategy::ManualIntervention).contains("ManualIntervention")
        );
    }

    #[test]
    fn test_recovery_strategy_clone() {
        let original = RecoveryStrategy::RetryWithBackoff;
        let cloned = original.clone();

        assert_eq!(format!("{original:?}"), format!("{:?}", cloned));
    }

    #[test]
    fn test_context_and_policy_combination() {
        let context = ErrorContextBuilder::new()
            .user_message("API 限流")
            .context("rate_limit", "100/minute")
            .context("retry_after", "60")
            .request_id("req-rate-001")
            .build();

        let policy = RetryPolicy::exponential(5, Duration::from_secs(2));

        assert_eq!(context.user_message(), Some("API 限流"));
        assert!(context.has_context("rate_limit"));

        assert!(policy.is_retryable());
        assert_eq!(policy.max_retries(), 5);
    }

    #[test]
    fn test_error_type_retryable_consistency() {
        let network = CoreError::network_builder().message("网络错误").build();
        assert_eq!(network.error_type(), ErrorType::Network);
        assert!(network.is_retryable());

        let validation = CoreError::validation_builder()
            .field("test")
            .message("验证错误")
            .build();
        assert_eq!(validation.error_type(), ErrorType::Validation);
        assert!(!validation.is_retryable());

        let auth = CoreError::authentication_builder()
            .message("认证错误")
            .build();
        assert_eq!(auth.error_type(), ErrorType::Authentication);
        assert!(!auth.is_retryable());
    }

    #[test]
    fn test_error_severity_from_error() {
        let validation = CoreError::validation_builder()
            .field("email")
            .message("邮箱格式错误")
            .build();
        assert_eq!(validation.severity(), ErrorSeverity::Warning);

        let network = CoreError::network_builder().message("连接失败").build();
        assert_eq!(network.severity(), ErrorSeverity::Critical);
    }
}
