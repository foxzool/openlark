//! 文件相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 上传文件响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFileResponse {
    /// 文件 key。
    pub file_key: String,
}

impl ApiResponseTrait for CreateFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
