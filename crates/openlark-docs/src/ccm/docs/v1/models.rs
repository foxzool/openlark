use openlark_core::api::{ApiResponseTrait, ResponseFormat};
/// Docs V1 API 数据模型
use serde::{Deserialize, Serialize};

/// 云文档内容信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsContent {
    /// 文档Token
    pub document_token: String,
    /// 文档标题
    pub title: String,
    /// 文档内容
    pub content: String,
    /// 文档类型
    pub document_type: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 创建者ID
    pub creator_id: String,
    /// 文档URL
    pub url: String,
}

impl ApiResponseTrait for DocsContent {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
