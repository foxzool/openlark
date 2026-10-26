/// 文件元数据管理模块
///
/// 提供文件元数据的批量查询功能。
pub mod batch_query;

// 显式导出 - 避免使用 glob reexport
pub use batch_query::{BatchQueryMetaRequest, BatchQueryMetaRequestBody, BatchQueryMetaResponse};
