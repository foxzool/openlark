//! API 工具函数（re-export core canonical + validate_required_field）。
//!
//! serialize_params / ensure_success 已下沉到
//! `openlark_core::api`（#330）；保留 meeting 域的 validate_required_field（会议叶子复用）。
use openlark_core::{SDKResult, error};

pub use openlark_core::api::{ensure_success, serialize_params};

/// 标准化必填字段校验。
pub fn validate_required_field<T: AsRef<str>>(
    field_name: &str,
    field_value: Option<T>,
    error_message: &str,
) -> SDKResult<()> {
    match field_value {
        Some(value) if !value.as_ref().trim().is_empty() => Ok(()),
        _ => Err(error::validation_error(field_name, error_message)),
    }
}
