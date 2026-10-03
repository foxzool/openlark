//! 云盘 v1 接口模块，聚合导出任务、文件、媒体与权限等能力。

/// 云盘导出任务模块。
pub mod export_task;
/// 云盘文件模块。
pub mod file;
/// 云盘导入任务模块。
pub mod import_task;
/// 云盘媒体模块。
pub mod media;
/// 云盘元数据模块。
pub mod meta;
/// 云盘权限模块。
pub mod permission;

/// 重新导出导出任务相关类型。
pub use export_task::{
    CreateExportTaskRequest, CreateExportTaskResponse, DownloadExportRequest, ExportTaskResult,
    GetExportTaskRequest, GetExportTaskResponse,
};

/// 重新导出导入任务相关类型。
pub use import_task::{
    CreateImportTaskRequest, CreateImportTaskResponse, GetImportTaskRequest, GetImportTaskResponse,
    ImportTaskResult,
};

/// 重新导出媒体相关类型。
pub use media::{
    BatchGetTmpDownloadUrlRequest, BatchGetTmpDownloadUrlResponse, DownloadMediaRequest,
    TmpDownloadUrlInfo, UploadAllMediaRequest, UploadAllMediaResponse, UploadFinishMediaRequest,
    UploadFinishMediaResponse, UploadPartMediaRequest, UploadPartMediaResponse,
    UploadPrepareMediaRequest, UploadPrepareMediaResponse,
};

/// 重新导出批量元数据查询请求。
pub use meta::BatchQueryMetaRequest;
/// user 模块。
pub mod user;
