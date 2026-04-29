//! 现代化错误处理特征系统
//!
//! 定义了符合Rust最佳实践的错误处理特征和接口

use crate::error::ErrorCode;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::time::Duration;

/// 核心错误特征
///
/// 所有错误类型都应该实现此特征，提供统一的错误处理接口
pub trait ErrorTrait: std::error::Error + Send + Sync + 'static {
    /// 获取错误严重程度
    fn severity(&self) -> ErrorSeverity;

    /// 判断是否可重试
    fn is_retryable(&self) -> bool;

    /// 获取建议的重试延迟
    fn retry_delay(&self, attempt: u32) -> Option<Duration>;

    /// 获取用户友好的错误消息
    fn user_message(&self) -> Option<&str>;

    /// 获取错误上下文信息
    fn context(&self) -> &ErrorContext;

    /// 判断错误类型
    fn error_type(&self) -> ErrorType;

    /// 获取错误代码（如果有）
    fn error_code(&self) -> Option<&str> {
        None
    }

    /// 判断是否为用户错误
    fn is_user_error(&self) -> bool {
        matches!(
            self.error_type(),
            ErrorType::Validation | ErrorType::Authentication
        )
    }

    /// 判断是否为系统错误
    fn is_system_error(&self) -> bool {
        matches!(
            self.error_type(),
            ErrorType::Network | ErrorType::ServiceUnavailable | ErrorType::Internal
        )
    }

    /// 判断是否为网络错误
    fn is_network_error(&self) -> bool {
        matches!(self.error_type(), ErrorType::Network)
    }

    /// 判断是否为认证错误
    fn is_auth_error(&self) -> bool {
        matches!(self.error_type(), ErrorType::Authentication)
    }

    /// 判断是否为验证错误
    fn is_validation_error(&self) -> bool {
        matches!(self.error_type(), ErrorType::Validation)
    }

    /// 判断是否为超时错误
    fn is_timeout_error(&self) -> bool {
        matches!(self.error_type(), ErrorType::Timeout)
    }

    /// 判断是否为限流错误
    fn is_rate_limited(&self) -> bool {
        matches!(self.error_type(), ErrorType::RateLimit)
    }

    /// 判断是否为配置错误
    fn is_config_error(&self) -> bool {
        matches!(self.error_type(), ErrorType::Configuration)
    }

    /// 判断是否为序列化错误
    fn is_serialization_error(&self) -> bool {
        matches!(self.error_type(), ErrorType::Serialization)
    }

    /// 判断是否为业务错误
    fn is_business_error(&self) -> bool {
        matches!(self.error_type(), ErrorType::Business)
    }

    /// 判断是否为API错误
    fn is_api_error(&self) -> bool {
        matches!(self.error_type(), ErrorType::Api)
    }

    /// 判断是否为服务不可用错误
    fn is_service_unavailable_error(&self) -> bool {
        matches!(self.error_type(), ErrorType::ServiceUnavailable)
    }
}

/// 错误严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// 信息级别 - 成功或提示信息
    Info,
    /// 警告级别 - 客户端错误或可恢复问题
    Warning,
    /// 错误级别 - 认证权限错误或业务逻辑错误
    Error,
    /// 严重错误级别 - 系统故障或不可恢复问题
    Critical,
}

impl ErrorSeverity {
    /// 获取严重程度的数值表示
    pub fn as_level(&self) -> u8 {
        match self {
            Self::Info => 0,
            Self::Warning => 1,
            Self::Error => 2,
            Self::Critical => 3,
        }
    }

    /// 获取严重程度的描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::Info => "信息",
            Self::Warning => "警告",
            Self::Error => "错误",
            Self::Critical => "严重错误",
        }
    }

    /// 从数值创建严重程度（兼容旧接口）
    pub fn from_level(level: u8) -> Self {
        match level {
            0 => Self::Info,
            1 => Self::Warning,
            2 => Self::Error,
            3 => Self::Critical,
            _ => Self::Error,
        }
    }

    /// 判断是否需要立即处理
    pub fn requires_immediate_action(&self) -> bool {
        matches!(self, Self::Critical | Self::Error)
    }

    /// 判断是否需要用户干预
    pub fn requires_user_intervention(&self) -> bool {
        matches!(self, Self::Error | Self::Critical)
    }

    /// 判断是否可以自动恢复
    pub fn is_auto_recoverable(&self) -> bool {
        matches!(self, Self::Info | Self::Warning)
    }
}

impl Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// 错误类型分类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    /// 网络相关错误
    Network,
    /// 认证相关错误
    Authentication,
    /// 验证相关错误
    Validation,
    /// API相关错误
    Api,
    /// 配置相关错误
    Configuration,
    /// 序列化相关错误
    Serialization,
    /// 业务逻辑错误
    Business,
    /// 超时错误
    Timeout,
    /// 限流错误
    RateLimit,
    /// 服务不可用错误
    ServiceUnavailable,
    /// 内部系统错误
    Internal,
    /// 响应体过大错误
    ResponseTooLarge,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network => write!(f, "Network"),
            Self::Authentication => write!(f, "Authentication"),
            Self::Validation => write!(f, "Validation"),
            Self::Api => write!(f, "Api"),
            Self::Configuration => write!(f, "Configuration"),
            Self::Serialization => write!(f, "Serialization"),
            Self::Business => write!(f, "Business"),
            Self::Timeout => write!(f, "Timeout"),
            Self::RateLimit => write!(f, "RateLimit"),
            Self::ServiceUnavailable => write!(f, "ServiceUnavailable"),
            Self::Internal => write!(f, "Internal"),
            Self::ResponseTooLarge => write!(f, "ResponseTooLarge"),
        }
    }
}

/// 从 ErrorCode 转换到 ErrorType
impl From<ErrorCode> for ErrorType {
    fn from(code: ErrorCode) -> Self {
        match code {
            ErrorCode::NetworkTimeout
            | ErrorCode::NetworkConnectionFailed
            | ErrorCode::DnsResolutionFailed
            | ErrorCode::SslCertificateError
            | ErrorCode::ConnectionRefused => ErrorType::Network,
            ErrorCode::Unauthorized
            | ErrorCode::AccessTokenInvalid
            | ErrorCode::AppAccessTokenInvalid
            | ErrorCode::TenantAccessTokenInvalid
            | ErrorCode::AuthenticationFailed
            | ErrorCode::TokenExpired
            | ErrorCode::PermissionDenied
            | ErrorCode::AccessDenied => ErrorType::Authentication,
            ErrorCode::BadRequest
            | ErrorCode::ValidationError
            | ErrorCode::MissingRequiredParameter
            | ErrorCode::InvalidParameterFormat
            | ErrorCode::ParameterOutOfRange => ErrorType::Validation,
            ErrorCode::NotFound
            | ErrorCode::MethodNotAllowed
            | ErrorCode::Conflict
            | ErrorCode::Forbidden => ErrorType::Api,
            ErrorCode::TooManyRequests | ErrorCode::RateLimitExceeded => ErrorType::RateLimit,
            ErrorCode::GatewayTimeout => ErrorType::Timeout,
            ErrorCode::ServiceUnavailable
            | ErrorCode::BadGateway
            | ErrorCode::CacheServiceUnavailable => ErrorType::ServiceUnavailable,
            ErrorCode::ConfigurationError => ErrorType::Configuration,
            ErrorCode::ResponseTooLarge => ErrorType::ResponseTooLarge,
            ErrorCode::Success => ErrorType::Api,
            _ => ErrorType::Internal,
        }
    }
}

/// 错误上下文特征
pub trait ErrorContextTrait {
    /// 获取用户友好消息
    fn user_message(&self) -> Option<&str>;

    /// 获取请求ID
    fn request_id(&self) -> Option<&str>;

    /// 获取操作名称
    fn operation(&self) -> Option<&str>;

    /// 获取组件名称
    fn component(&self) -> Option<&str>;

    /// 获取时间戳
    fn timestamp(&self) -> Option<chrono::DateTime<chrono::Utc>>;

    /// 获取上下文信息
    fn get_context(&self, key: &str) -> Option<&str>;

    /// 获取所有上下文信息
    fn all_context(&self) -> &std::collections::HashMap<String, String>;

    /// 检查是否有指定的上下文键
    fn has_context(&self, key: &str) -> bool {
        self.get_context(key).is_some()
    }

    /// 判断是否为空
    fn is_empty(&self) -> bool {
        self.user_message().is_none()
            && self.request_id().is_none()
            && self.operation().is_none()
            && self.component().is_none()
            && self.all_context().is_empty()
    }
}

/// 使用导入的ErrorContext类型
use super::context::ErrorContext;

impl ErrorContextTrait for ErrorContext {
    fn user_message(&self) -> Option<&str> {
        ErrorContext::user_message(self)
    }

    fn request_id(&self) -> Option<&str> {
        ErrorContext::request_id(self)
    }

    fn operation(&self) -> Option<&str> {
        ErrorContext::operation(self)
    }

    fn component(&self) -> Option<&str> {
        ErrorContext::component(self)
    }

    fn timestamp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        ErrorContext::timestamp(self).map(|st| {
            let duration = st.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
            chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                .unwrap_or_else(chrono::Utc::now)
        })
    }

    fn get_context(&self, key: &str) -> Option<&str> {
        ErrorContext::get_context(self, key)
    }

    fn all_context(&self) -> &std::collections::HashMap<String, String> {
        ErrorContext::all_context(self)
    }
}

/// 错误构建器特征
pub trait ErrorBuilderTrait: Sized {
    /// 设置用户消息
    fn message(self, message: impl Into<String>) -> Self;

    /// 设置请求ID
    fn request_id(self, request_id: impl Into<String>) -> Self;

    /// 设置操作名称
    fn operation(self, operation: impl Into<String>) -> Self;

    /// 设置组件名称
    fn component(self, component: impl Into<String>) -> Self;

    /// 添加上下文信息
    fn context(self, key: impl Into<String>, value: impl Into<String>) -> Self;

    /// 构建错误（V3）
    fn build(self) -> super::CoreError;
}

/// 错误格式化特征
pub trait ErrorFormatTrait {
    /// 格式化为用户友好的消息
    fn format_user_message(&self) -> String;

    /// 格式化为开发者友好的消息
    fn format_developer_message(&self) -> String;

    /// 格式化为日志消息
    fn format_log_message(&self) -> String;

    /// 格式化为JSON
    fn format_json(&self) -> serde_json::Result<serde_json::Value>;
}

/// 完整错误特征
///
/// 组合所有错误相关特征的超级特征
pub trait FullErrorTrait:
    ErrorTrait + ErrorContextTrait + ErrorFormatTrait + Debug + Display + Clone + Send + Sync + 'static
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_severity() {
        assert!(ErrorSeverity::Info < ErrorSeverity::Warning);
        assert!(ErrorSeverity::Warning < ErrorSeverity::Error);
        assert!(ErrorSeverity::Error < ErrorSeverity::Critical);

        assert!(ErrorSeverity::Critical.requires_immediate_action());
        assert!(ErrorSeverity::Info.is_auto_recoverable());
    }

    #[test]
    fn test_error_type_display() {
        assert_eq!(ErrorType::Network.to_string(), "Network");
        assert_eq!(ErrorType::Authentication.to_string(), "Authentication");
        assert_eq!(ErrorType::Validation.to_string(), "Validation");
    }

    #[test]
    fn test_error_severity_display() {
        assert_eq!(ErrorSeverity::Info.to_string(), "信息");
        assert_eq!(ErrorSeverity::Warning.to_string(), "警告");
        assert_eq!(ErrorSeverity::Error.to_string(), "错误");
        assert_eq!(ErrorSeverity::Critical.to_string(), "严重错误");
    }
}
