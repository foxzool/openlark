/// batch_create 子模块。
pub mod batch_create;
/// batch_delete 子模块。
pub mod batch_delete;
/// 记录管理模块
///
/// 提供多维表格记录的 CRUD 操作，包括批量操作。
pub mod batch_get;
/// batch_update 子模块。
pub mod batch_update;
/// create 子模块。
pub mod create;
/// delete 子模块。
pub mod delete;
/// get 子模块。
pub mod get;
/// list 子模块。
pub mod list;
/// models 子模块。
pub mod models;
/// search 子模块。
pub mod search;
/// update 子模块。
pub mod update;

// 显式导出 - 避免使用 glob reexport
/// 重新导出相关类型。
pub use batch_create::{
    BatchCreateRecordRequest, BatchCreateRecordRequestBody, BatchCreateRecordResponse,
    CreateRecordItem,
};

/// 重新导出相关类型。
pub use batch_delete::{
    BatchDeleteRecordRequest, BatchDeleteRecordRequestBody, BatchDeleteRecordResponse,
    DeletedRecord,
};

/// 重新导出相关类型。
pub use batch_get::{BatchGetRecordRequest, BatchGetRecordResponse};

/// 重新导出相关类型。
pub use batch_update::{
    BatchUpdateRecordRequest, BatchUpdateRecordRequestBody, BatchUpdateRecordResponse,
    UpdateRecordItem,
};

/// 重新导出相关类型。
pub use create::{CreateRecordRequest, CreateRecordRequestBody, CreateRecordResponse};

/// 重新导出相关类型。
pub use delete::{DeleteRecordRequest, DeleteRecordResponse};

/// 重新导出相关类型。
pub use get::{GetRecordRequest, GetRecordResponse};

/// 重新导出相关类型。
pub use list::{ListRecordRequest, ListRecordResponse};

/// 重新导出相关类型。
pub use models::{Person, Record};

/// 重新导出相关类型。
pub use search::{
    FilterCondition, FilterInfo, SearchRecordRequest, SearchRecordRequestBody,
    SearchRecordResponse, SortCondition,
};

/// 重新导出相关类型。
pub use update::{UpdateRecordRequest, UpdateRecordRequestBody, UpdateRecordResponse};
