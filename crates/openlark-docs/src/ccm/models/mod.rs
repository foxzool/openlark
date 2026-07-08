//! 云文档通用模型定义。

use serde::{Deserialize, Serialize};

/// 文件类型
#[derive(Debug, Deserialize, Serialize)]
pub enum FileType {
    /// 文档
    Document,
    /// 电子表格
    Spreadsheet,
    /// 演示文稿
    Presentation,
    /// 图片
    Image,
    /// 视频
    Video,
    /// 其他类型
    Other,
}

/// 云文档响应
#[derive(Debug, Deserialize, Serialize)]
pub struct CcmResponse<T> {
    /// 错误码，0 表示成功
    pub code: i32,
    /// 错误描述
    pub msg: String,
    /// 业务数据
    pub data: Option<T>,
}

/// 通用分页响应
#[derive(Debug, Deserialize, Serialize)]
pub struct PagedResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: bool,
}
