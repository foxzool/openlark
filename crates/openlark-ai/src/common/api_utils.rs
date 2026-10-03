//! API 工具函数（re-export core canonical）。
//!
//! serialize_params re-export core canonical（#330 下沉）。响应抽取走 Transport::request_typed
//! / Response::decode（#470 楔子），不再经本模块。

pub use openlark_core::api::serialize_params;
