//! 任务 API v2 模块
//!
//! 遵循飞书开放平台 API 规范的任务管理接口。

pub mod attachment;
pub mod custom_field;
pub mod task;
pub mod task_v2;
pub mod tasklist;
pub mod section;
pub mod comment;

// 重新导出所有子模块
// custom_field 模块显式导出
pub use custom_field::{
    AddCustomFieldRequest,
    CreateCustomFieldRequest,
    CreateCustomFieldOptionRequest, GetCustomFieldRequest, ListCustomFieldsRequest,
    PatchCustomFieldBody,
    PatchCustomFieldRequest,
    PatchCustomFieldResponse,
    RemoveCustomFieldRequest,
    UpdateCustomFieldOptionRequest,
};
// attachment 模块显式导出
pub use attachment::{DeleteAttachmentRequest, GetAttachmentRequest, ListAttachmentsRequest};
// task 模块显式导出
pub use task::{};
pub use task::{GetTaskRequest, GetTaskTasklistsRequest, ListSubtasksRequest, ListTasksRequest, SearchTaskRequest};
pub use task_v2::{ListRelatedTaskRequest, TaskSubscriptionRequest};
// tasklist 模块显式导出
pub use tasklist::{GetTasklistRequest, GetTasklistTasksRequest, ListTasklistsRequest, SearchTasklistRequest};
// section 模块显式导出
pub use section::{};
pub use section::{GetSectionRequest, GetSectionTasksRequest, ListSectionsRequest};
// comment 模块显式导出
pub use comment::{
    CreateCommentRequest,
    DeleteCommentRequest,
    GetCommentRequest,
    GetCommentResponse,
    ListCommentsRequest,
    ListCommentsResponse,
    PatchCommentRequest,
};
