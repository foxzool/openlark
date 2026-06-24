//! 导出任务 API 模块。
//!
//! 提供文档导出任务相关的数据模型，包括：
//! - 创建导出任务（[`models::CreateExportTaskRequest`]）
//! - 查询导出任务状态（[`models::GetExportTaskRequest`]）
//! - 下载导出文件（[`models::DownloadExportFileRequest`]）
//!
//! 注意：服务实现（`services`）为占位模块，待后续补充。

/// 数据模型定义
pub mod models;

/// API 服务实现（占位）
pub mod services;

// 重新导出主要类型
pub use models::{
    CreateExportTaskRequest, CreateExportTaskResponse, DownloadExportFileRequest,
    DownloadExportFileResponse, ExportTaskResult, ExportTaskStatus, GetExportTaskRequest,
    GetExportTaskResponse,
};
