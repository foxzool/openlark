//! API 工具函数（re-export core canonical）。
//!
//! serialize_params / ensure_success 已下沉到
//! `openlark_core::api`（#330），本模块仅 re-export canonical copy，避免各 crate 各派生一份。

pub use openlark_core::api::{ensure_success, serialize_params};
