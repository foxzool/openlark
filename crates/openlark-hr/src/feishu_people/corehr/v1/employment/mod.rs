//! CoreHR 雇佣信息 API
//!
//! 提供雇佣信息管理相关的 API，包括创建、删除、更新等功能。

pub mod create;
pub mod delete;
pub mod models;
pub mod patch;

// Re-export 公共类型
pub use create::CreateRequest;
pub use delete::DeleteRequest;
pub use models::{
    CreateRequestBody, CreateResponse, CustomField, DeleteRequestBody, DeleteResponse, Employment,
    EnumValue, PatchRequestBody, PatchResponse,
};
pub use patch::PatchRequest;
