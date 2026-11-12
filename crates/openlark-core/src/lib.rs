//! OpenLark Core Infrastructure
//!
//! This crate provides the core infrastructure for the OpenLark SDK including
//! HTTP client configuration, error handling, authentication, and common utilities.

// 对外稳定导出：尽量保持"少而清晰"的公共 API（KISS）
pub mod api;
pub mod auth;
/// 客户端配置模块（Config、ConfigBuilder 等）
pub mod config;
/// 全局常量定义（URL、错误码前缀、超时配置等）
pub mod constants;
/// 统一错误处理模块（CoreError、错误码、错误上下文等）
pub mod error;
/// HTTP 客户端模块（Transport、请求构建等）
pub mod http;
pub(crate) mod observability;
/// 请求选项模块（RequestOption、自定义头部、租户键等）
pub mod req_option;
pub(crate) mod request_execution;
/// Security utilities for handling sensitive data
pub mod security;
#[cfg(feature = "testing")]
pub mod testing;
pub mod validation;

// crate 内部实现细节：不对外暴露（避免把 core 变成"全家桶"）
// 已移动到 auth::app_ticket
mod content_disposition;
mod performance;
mod utils;

// Re-export commonly used types from crate root
pub use error::{CoreError, SDKResult, validation_error};
pub use validation::Validatable;

// Re-export validate_required macro for docs module
/// 验证必填字段（检查非空白字符串）
///
/// # 参数
/// - `$field`: 要验证的字段
/// - `$error_msg`: 错误消息
///
/// # 使用示例
/// ```rust,ignore
/// validate_required!(self.user_id, "用户 ID 不能为空");
/// ```
#[macro_export]
macro_rules! validate_required {
    ($field:expr, $error_msg:expr) => {
        if $crate::Validatable::is_empty_trimmed(&$field) {
            return Err($crate::error::CoreError::validation_msg($error_msg));
        }
    };
}

/// 验证必填列表字段（检查非空和最大长度）
///
/// # 参数
/// - `$field`: 要验证的列表字段
/// - `$max_len`: 最大长度限制
/// - `$error_msg`: 错误消息
///
/// # 使用示例
/// ```rust,ignore
/// validate_required_list!(self.user_ids, 50, "用户 ID 列表不能为空且不能超过 50 个");
/// ```
#[macro_export]
macro_rules! validate_required_list {
    ($field:expr, $max_len:expr, $error_msg:expr) => {
        if $field.is_empty() {
            return Err($crate::error::CoreError::validation_msg($error_msg));
        }
        if $field.len() > $max_len {
            return Err($crate::error::CoreError::validation_msg($error_msg));
        }
    };
}

/// 验证 Empty-ok List：空列表合法。
///
/// 无上限的形式不拒绝任何长度，调用本身表示该列表允许为空。
/// 带上限的形式只在长度超过 `$max_len` 时失败。
///
/// # 使用示例
/// ```rust,ignore
/// validate_allow_empty_list!(self.wk_calendar_ids);
/// validate_allow_empty_list!(self.user_ids, 10, "user_ids 不能超过 10 个");
/// ```
#[macro_export]
macro_rules! validate_allow_empty_list {
    ($field:expr) => {{
        let _len = $field.len();
    }};
    ($field:expr, $max_len:expr, $error_msg:expr) => {
        if $field.len() > $max_len {
            return Err($crate::error::CoreError::validation_msg($error_msg));
        }
    };
}

/// Prelude module for convenient imports.
pub mod prelude {
    // Re-export new API module（请求/响应基础类型）
    pub use crate::api::prelude::*;

    // Re-export commonly used core modules directly（最小集合）
    pub use crate::config::Config;
    pub use crate::constants::*;
    pub use crate::error::{CoreError, SDKResult, validation_error};
    pub use crate::http::Transport;
    pub use crate::req_option::*;
    pub use crate::validate_allow_empty_list;
    pub use crate::validate_required;
    pub use crate::validate_required_list;
}

#[cfg(test)]
mod allow_empty_list_tests {
    use super::CoreError;

    fn allow_empty_with_max(values: &[String]) -> Result<(), CoreError> {
        crate::validate_allow_empty_list!(values, 10, "user_ids 不能超过 10 个");
        Ok(())
    }

    fn allow_empty_without_max(values: &[String]) -> Result<(), CoreError> {
        crate::validate_allow_empty_list!(values);
        Ok(())
    }

    #[test]
    fn empty_list_within_max_is_allowed() {
        assert!(allow_empty_with_max(&[]).is_ok());
    }

    #[test]
    fn eleven_items_over_max_is_rejected() {
        let values: Vec<String> = (0..11).map(|index| index.to_string()).collect();
        let err = allow_empty_with_max(&values).expect_err("超过 10 个应失败");
        assert!(err.to_string().contains("不能超过 10 个"), "{err}");
    }

    #[test]
    fn no_max_allows_a_long_list() {
        let values: Vec<String> = (0..50).map(|index| index.to_string()).collect();
        assert!(allow_empty_without_max(&values).is_ok());
    }
}
