//! API 工具函数（re-export core canonical）。
//!
//! serialize_params 已下沉到 `openlark::api`（#330）。extract_response_data 原 re-export
//! 在 #485 迁移到 Transport::request_typed 后零 caller，移除（core helper 删除留给 #486）。

pub use openlark_core::api::serialize_params;
