//! CoreHR 地点 API
//!
//! 提供地点管理相关的 API，包括创建、删除、查询、更新等功能。

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod patch;

// Re-export 公共类型
pub use create::CreateRequest;
pub use delete::DeleteRequest;
pub use get::GetRequest;
pub use list::ListRequest;
pub use models::{
    CreateRequestBody, CreateResponse, DeleteResponse, GetResponse, HiberarchyCommon, I18n,
    ListRequestBody, ListResponse, Location, PatchRequestBody, PatchResponse,
};
pub use patch::PatchRequest;
