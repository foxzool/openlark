//! CoreHR 部门 API
//!
//! 提供部门管理相关的 API，包括创建、删除、查询、搜索等功能。

pub mod batch_get;
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod multi_timeline;
pub mod operation_logs;
pub mod parents;
pub mod patch;
pub mod search;
pub mod timeline;
pub mod tree;

// Re-export 公共类型
pub use batch_get::BatchGetRequest;
pub use create::CreateRequest;
pub use delete::DeleteRequest;
pub use get::GetRequest;
pub use list::ListRequest;
pub use models::{
    BatchGetRequestBody, BatchGetResponse, CreateRequestBody, CreateResponse, DeleteRequestBody,
    DeleteResponse, Department, DepartmentChange, DepartmentOperationLog, DepartmentTimeline,
    DepartmentTreeNode, GetRequestBody, GetResponse, HiberarchyCommon, I18n, ListRequestBody,
    ListResponse, MultiTimelineRequestBody, MultiTimelineResponse, OperationLogsRequestBody,
    OperationLogsResponse, ParentsResponse, PatchRequestBody, PatchResponse, SearchRequestBody,
    SearchResponse, TimelineRequestBody, TimelineResponse, TreeRequestBody, TreeResponse,
};
pub use multi_timeline::MultiTimelineRequest;
pub use operation_logs::OperationLogsRequest;
pub use parents::ParentsRequest;
pub use patch::PatchRequest;
pub use search::SearchRequest;
pub use timeline::TimelineRequest;
pub use tree::TreeRequest;
