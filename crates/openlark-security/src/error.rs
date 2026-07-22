//! 安全服务错误处理模块
//!
//! 完全基于 CoreError 的现代化错误处理系统
//! 直接集成统一错误体系，提供类型安全和可观测性

use openlark_core::error::{
    CoreError, ErrorCode, ErrorContext, authentication_error, business_error, configuration_error,
    network_error_with_details, permission_missing_error, rate_limit_error, token_expired_error,
    validation_error,
};
use std::time::Duration;

// 导入内部结构体
use openlark_core::error::ApiError;

/// 安全服务错误类型 - 完全基于 CoreError
pub type SecurityError = CoreError;

/// 安全服务结果类型
pub type SecurityResult<T> = Result<T, SecurityError>;

/// 安全错误构建器 - 专门用于安全场景的便利函数
#[derive(Debug, Copy, Clone)]
pub struct SecurityErrorBuilder;

impl SecurityErrorBuilder {
    /// 设备未找到
    pub fn device_not_found(device_id: impl Into<String>) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("device_id", device_id.into());
        ctx.add_context("operation", "device_lookup");

        CoreError::Validation {
            field: "device_id".into(),
            message: "设备未找到，请检查设备ID是否正确".to_string(),
            code: ErrorCode::ValidationError,
            ctx: Box::new(ctx),
        }
    }

    /// 设备连接失败
    pub fn device_connection_failed(
        device_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("device_id", device_id.into());
        ctx.add_context("connection_reason", reason.into());
        ctx.add_context("operation", "device_connection");

        network_error_with_details(
            "设备连接失败",
            None::<String>,
            Some(format!(
                "device:{}",
                ctx.get_context("device_id").unwrap_or_default()
            )),
        )
    }

    /// 设备临时不可用（可重试）
    pub fn device_temporarily_unavailable(
        device_id: impl Into<String>,
        retry_after: Option<Duration>,
    ) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("device_id", device_id.into());
        ctx.add_context("availability", "temporary");

        CoreError::ServiceUnavailable {
            service: "security_device".into(),
            retry_after,
            code: ErrorCode::ServiceUnavailable,
            ctx: Box::new(ctx),
        }
    }

    /// 访问控制被拒绝
    pub fn access_denied(resource: impl Into<String>, reason: impl Into<String>) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("resource", resource.into());
        ctx.add_context("deny_reason", reason.into());
        ctx.add_context("operation", "access_control");

        permission_missing_error(&["security:access"])
    }

    /// 权限范围不足
    pub fn insufficient_permissions(
        required_permissions: &[impl AsRef<str>],
        current_permissions: &[impl AsRef<str>],
    ) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context(
            "required_permissions",
            required_permissions
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>()
                .join(","),
        );
        ctx.add_context(
            "current_permissions",
            current_permissions
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>()
                .join(","),
        );

        CoreError::Authentication {
            message: "安全权限不足".to_string(),
            code: ErrorCode::PermissionMissing,
            ctx: Box::new(ctx),
        }
    }

    /// 人脸识别失败
    pub fn face_recognition_failed(
        reason: impl Into<String>,
        image_id: Option<impl Into<String>>,
    ) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("recognition_reason", reason.into());
        if let Some(id) = image_id {
            ctx.add_context("image_id", id.into());
        }
        ctx.add_context("operation", "face_recognition");

        validation_error("face_image", "人脸识别失败，请重新上传清晰的人脸照片")
    }

    /// 人脸识别服务不可用
    pub fn face_recognition_service_unavailable() -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("service", "face_recognition");
        ctx.add_context("operation", "face_recognition");

        CoreError::ServiceUnavailable {
            service: "face_recognition".into(),
            retry_after: Some(Duration::from_secs(30)),
            code: ErrorCode::ServiceUnavailable,
            ctx: Box::new(ctx),
        }
    }

    /// 访客权限过期
    pub fn visitor_permission_expired(
        visitor_id: impl Into<String>,
        visit_type: impl Into<String>,
    ) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("visitor_id", visitor_id.into());
        ctx.add_context("visit_type", visit_type.into());
        ctx.add_context("operation", "visitor_access");

        business_error("访客权限已过期，请重新申请")
    }

    /// 访客身份验证失败
    pub fn visitor_authentication_failed(
        visitor_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("visitor_id", visitor_id.into());
        ctx.add_context("auth_reason", reason.into());
        ctx.add_context("operation", "visitor_authentication");

        authentication_error("访客身份验证失败")
    }

    /// 合规检查失败
    pub fn compliance_check_failed(
        compliance_type: impl Into<String>,
        reason: impl Into<String>,
        resource_id: Option<impl Into<String>>,
    ) -> SecurityError {
        let compliance_type_str = compliance_type.into();
        let reason_str = reason.into();
        let mut ctx = ErrorContext::new();
        ctx.add_context("compliance_type", compliance_type_str);
        ctx.add_context("violation_reason", reason_str.clone());
        if let Some(id) = resource_id {
            ctx.add_context("resource_id", id.into());
        }
        ctx.add_context("operation", "compliance_check");

        CoreError::Business {
            message: format!("合规检查失败: {reason_str}"),
            code: ErrorCode::BusinessError,
            ctx: Box::new(ctx),
        }
    }

    /// 审计日志写入失败
    pub fn audit_log_failed(
        log_type: impl Into<String>,
        reason: impl Into<String>,
    ) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("log_type", log_type.into());
        ctx.add_context("failure_reason", reason.into());
        ctx.add_context("operation", "audit_logging");

        CoreError::Internal {
            code: ErrorCode::InternalError,
            message: "审计日志写入失败".to_string(),
            source: None,
            ctx: Box::new(ctx),
        }
    }

    /// 配置错误
    pub fn security_config_invalid(
        config_key: impl Into<String>,
        reason: impl Into<String>,
    ) -> SecurityError {
        let config_key_str = config_key.into();
        let reason_str = reason.into();
        let mut ctx = ErrorContext::new();
        ctx.add_context("config_key", config_key_str.clone());
        ctx.add_context("error_reason", reason_str.clone());
        ctx.add_context("operation", "security_config");

        configuration_error(format!("安全配置参数 {config_key_str} 无效: {reason_str}"))
    }

    /// 时间同步错误
    pub fn time_sync_failed(service: impl Into<String>, deviation_ms: i64) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("sync_service", service.into());
        ctx.add_context("time_deviation_ms", deviation_ms.to_string());
        ctx.add_context("operation", "time_sync");

        business_error("时间同步失败，安全验证需要精确的时间同步")
    }

    /// 加密操作失败
    pub fn crypto_operation_failed(
        operation: impl Into<String>,
        algorithm: impl Into<String>,
        reason: impl Into<String>,
    ) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("crypto_operation", operation.into());
        ctx.add_context("algorithm", algorithm.into());
        ctx.add_context("failure_reason", reason.into());
        ctx.add_context("operation", "cryptography");

        CoreError::Internal {
            code: ErrorCode::InternalError,
            message: "加密操作失败".to_string(),
            source: None,
            ctx: Box::new(ctx),
        }
    }

    /// 安全检查超时
    pub fn security_check_timeout(
        check_type: impl Into<String>,
        timeout_duration: Duration,
    ) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("check_type", check_type.into());
        ctx.add_context(
            "timeout_duration_ms",
            timeout_duration.as_millis().to_string(),
        );
        ctx.add_context("operation", "security_check");

        CoreError::Timeout {
            duration: timeout_duration,
            operation: Some(format!(
                "security_check:{}",
                ctx.get_context("check_type").unwrap_or_default()
            )),
            ctx: Box::new(ctx),
        }
    }

    /// 安全API调用限流
    pub fn security_api_rate_limited(
        endpoint: impl Into<String>,
        limit: u32,
        window_seconds: u64,
    ) -> SecurityError {
        let mut ctx = ErrorContext::new();
        ctx.add_context("api_endpoint", endpoint.into());
        ctx.add_context("rate_limit", limit.to_string());
        ctx.add_context("window_seconds", window_seconds.to_string());

        rate_limit_error(limit, Duration::from_secs(window_seconds), None)
    }
}

/// 飞书安全服务错误码智能映射
pub fn map_feishu_security_error(
    feishu_code: i32,
    message: &str,
    request_id: Option<&str>,
) -> SecurityError {
    let mut ctx = ErrorContext::new();
    if let Some(req_id) = request_id {
        ctx.set_request_id(req_id);
    }
    ctx.add_context("feishu_code", feishu_code.to_string());
    ctx.add_context("service", "security");

    // 优先使用飞书通用错误码映射
    match ErrorCode::from_feishu_code(feishu_code) {
        // 权限相关错误
        Some(ErrorCode::PermissionMissing) => CoreError::Authentication {
            message: format!("安全权限不足: {message}"),
            code: ErrorCode::PermissionMissing,
            ctx: Box::new(ctx),
        },
        // 令牌相关错误
        Some(ErrorCode::AccessTokenExpiredV2) => {
            token_expired_error(format!("安全访问令牌已过期: {message}"))
        }
        // 参数验证错误
        Some(ErrorCode::ValidationError) => validation_error("security_parameter", message),
        // 业务逻辑错误
        Some(ErrorCode::BusinessError) => {
            SecurityErrorBuilder::compliance_check_failed("business_rule", message, None::<String>)
        }
        // 其他映射
        _ => {
            // 回退到HTTP状态码或内部业务码
            CoreError::Api(Box::new(ApiError {
                status: feishu_code as u16,
                endpoint: "security".into(),
                message: message.to_string(),
                source: None,
                code: ErrorCode::from_feishu_code(feishu_code).unwrap_or(ErrorCode::InternalError),
                ctx: Box::new(ctx),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::error::ErrorTrait;

    #[test]
    fn test_security_error_creation() {
        let error = SecurityErrorBuilder::device_not_found("device_123");
        assert!(matches!(error, CoreError::Validation { .. }));
        assert!(error.context().get_context("device_id").is_some());
    }

    #[test]
    fn test_permission_error() {
        let error = SecurityErrorBuilder::access_denied("admin_panel", "insufficient_role");
        assert!(matches!(
            error,
            CoreError::Authentication {
                code: ErrorCode::PermissionMissing,
                ..
            }
        ));
    }

    #[test]
    fn test_compliance_error() {
        let error = SecurityErrorBuilder::compliance_check_failed(
            "gdpr",
            "data_retention_violation",
            Some("data_set_456"),
        );
        assert!(matches!(error, CoreError::Business { .. }));
        assert!(error.context().get_context("compliance_type").is_some());
    }

    #[test]
    fn test_feishu_error_mapping() {
        let error = map_feishu_security_error(99991672, "权限不足", Some("req_123"));
        assert!(matches!(
            error,
            CoreError::Authentication {
                code: ErrorCode::PermissionMissing,
                ..
            }
        ));
    }
}
