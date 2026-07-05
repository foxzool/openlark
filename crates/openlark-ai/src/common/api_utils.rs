//! API 工具函数（re-export core canonical）。
//!
//! serialize_params / extract_response_data 已下沉到 `openlark_core::api`（#330）。
//! ai 当前不用 ensure_success，故仅 re-export 用到的两项。

pub use openlark_core::api::{extract_response_data, serialize_params};
