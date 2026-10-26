/// 媒体文件操作模块
///
/// 提供媒体文件的上传和下载功能，包括分片上传、批量获取下载链接等。
pub mod batch_get_tmp_download_url;
pub mod download;
pub mod upload_all;
pub mod upload_finish;
pub mod upload_part;
pub mod upload_prepare;

// 显式导出 - 避免使用 glob reexport
pub use batch_get_tmp_download_url::{
    BatchGetTmpDownloadUrlRequest, BatchGetTmpDownloadUrlResponse, TmpDownloadUrlInfo,
};

pub use download::DownloadMediaRequest;

pub use upload_all::{UploadAllMediaRequest, UploadAllMediaResponse};

pub use upload_finish::{
    UploadFinishMediaBody, UploadFinishMediaRequest, UploadFinishMediaResponse,
};

pub use upload_part::{UploadPartMediaRequest, UploadPartMediaResponse};

pub use upload_prepare::{
    UploadPrepareMediaBody, UploadPrepareMediaRequest, UploadPrepareMediaResponse,
};
