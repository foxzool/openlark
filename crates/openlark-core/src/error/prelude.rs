//! 现代化错误处理系统预置模块
//!
//! 提供新thiserror架构的完整错误处理组件导入

// 核心类型重新导出（主要推荐 CoreError）
pub use super::codes::ErrorCode;
pub use super::context::ErrorContext;
pub use super::core::{BuilderKind, ErrorBuilder, ErrorRecord};
pub use super::core::{RecoveryStrategy, RetryPolicy};
pub use super::traits::{ErrorSeverity, ErrorType};
pub use super::{CoreError, ErrorId, SDKResult};

// 特征系统重新导出
pub use super::traits::{ErrorContextTrait, ErrorFormatTrait, ErrorTrait, FullErrorTrait};

pub use super::core::{
    api_error, authentication_error, business_error, configuration_error, network_error,
    network_error_with_details, rate_limit_error, serialization_error, service_unavailable_error,
    timeout_error, validation_error,
};

// 常用的导入组合
/// 常用导入组合模块
///
/// 包含错误处理核心类型、集合类型和时间类型，方便快速导入
pub mod common_imports {
    pub use super::*;
    pub use std::collections::HashMap;
    pub use std::time::Duration;
}

/// 创建网络错误的宏
#[macro_export]
macro_rules! network_err {
    ($msg:expr) => {
        $crate::error::network_error($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::network_error(format!($fmt, $($arg)*))
    };
}

/// 创建认证错误的宏
#[macro_export]
macro_rules! auth_err {
    ($msg:expr) => {
        $crate::error::authentication_error($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::authentication_error(format!($fmt, $($arg)*))
    };
}

/// 创建验证错误的宏
#[macro_export]
macro_rules! validation_err {
    ($field:expr, $msg:expr) => {
        $crate::error::validation_error($field, $msg)
    };
    ($field:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::error::validation_error($field, format!($fmt, $($arg)*))
    };
}

/// 创建业务错误的宏
#[macro_export]
macro_rules! business_err {
    ($msg:expr) => {
        $crate::error::business_error($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::business_error(format!($fmt, $($arg)*))
    };
}

/// 创建API错误的宏
#[macro_export]
macro_rules! api_err {
    ($raw_code:expr, $endpoint:expr, $msg:expr) => {
        $crate::error::api_error($raw_code, $endpoint, $msg, None::<String>)
    };
    ($raw_code:expr, $endpoint:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::error::api_error($raw_code, $endpoint, format!($fmt, $($arg)*), None::<String>)
    };
}

/// 条件错误返回宏（现代化版本）
#[macro_export]
macro_rules! ensure {
    ($condition:expr, $error:expr) => {
        if !$condition {
            return Err(Into::<$crate::error::CoreError>::into($error));
        }
    };
    ($condition:expr, $fmt:expr, $($arg:tt)*) => {
        if !$condition {
            return Err(Into::<$crate::error::CoreError>::into(
                $crate::error::validation_error("condition", format!($fmt, $($arg)*))
            ));
        }
    };
}

/// 快速验证宏
#[macro_export]
macro_rules! validate {
    ($field:expr, $value:expr, $error_msg:expr) => {
        if !$value {
            return Err(Into::<$crate::error::CoreError>::into(
                $crate::error::validation_error($field, $error_msg)
            ));
        }
    };
    ($field:expr, $value:expr, $error_msg:expr, $($arg:tt)*) => {
        if !$value {
            return Err(Into::<$crate::error::CoreError>::into(
                $crate::error::validation_error($field, format!($error_msg, $($arg)*))
            ));
        }
    };
}

/// 错误匹配和处理宏
#[macro_export]
macro_rules! handle_error {
    ($result:expr, {
        $pattern:pat => $handler:expr,
        * => $default_handler:expr,
    }) => {
        match $result {
            Ok(value) => value,
            Err($pattern) => $handler,
            Err(error) => $default_handler(error),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modern_prelude_imports() {
        // 确保所有重新导出的类型都可以使用
        let _error: CoreError = network_error("测试");
        let _result: SDKResult<()> = Ok(());
        let _severity: ErrorSeverity = ErrorSeverity::Warning;
        let _code: ErrorCode = ErrorCode::BadRequest;
        let _lark_error: CoreError = network_error("测试");
    }

    #[test]
    fn test_modern_convenience_macros() {
        let error = network_err!("连接失败");
        assert!(error.is_network_error());

        let error = auth_err!("认证失败");
        assert!(error.is_auth_error());

        let error = validation_err!("email", "格式不正确");
        assert!(error.is_validation_error());

        let error = api_err!(404, "/api/users", "用户不存在");
        assert!(error.is_api_error());
    }

    #[test]
    fn test_builder_macro() {
        let error = validation_error("email", "邮箱格式不正确");

        assert!(error.is_validation_error());
        assert_eq!(error.context().get_context("field"), Some("email"));
    }

    #[test]
    fn test_ensure_macro() {
        let result = || -> SDKResult<()> {
            ensure!(true, validation_error("test", "不应该失败"));
            ensure!(false, validation_error("test", "应该失败"));
            Ok(())
        }();

        assert!(result.is_err());
        assert!(result.unwrap_err().is_validation_error());
    }

    #[test]
    fn test_validate_macro() {
        let result = || -> SDKResult<()> {
            validate!("email", "test@example".contains("@"), "邮箱格式不正确");
            Ok(())
        }();

        assert!(result.is_ok());

        let result = || -> SDKResult<()> {
            validate!("email", "invalid_email".contains("@"), "邮箱格式不正确");
            Ok(())
        }();

        assert!(result.is_err());
        assert!(result.unwrap_err().is_validation_error());
    }

    #[test]
    fn test_modern_error_creation() {
        let error = api_error(
            404,
            "/api/v1/users/123",
            "用户不存在",
            Some("req-123".to_string()),
        );

        assert!(error.is_api_error());
        assert_eq!(error.severity(), ErrorSeverity::Warning);
        assert!(!error.is_retryable());
        assert!(!error.is_user_error());
        assert_eq!(error.context().request_id(), Some("req-123"));
    }
}
