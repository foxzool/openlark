pub mod create;
pub mod delete;
pub mod get;
/// 视图管理模块
///
/// 提供多维表格视图的 CRUD 操作。
pub mod list;
pub mod patch;

// 显式导出 - 避免使用 glob reexport
pub use create::{CreateViewData, CreateViewRequest, CreateViewRequestBody, CreateViewResponse};

pub use delete::{DeleteViewRequest, DeleteViewResponse};

pub use get::{GetViewRequest, GetViewResponse};

pub use list::{ListViewsRequest, ListViewsResponse};

pub use patch::{PatchViewData, PatchViewRequest, PatchViewResponse, View};
