//! 分组 API v2 的数据模型

use serde::{Deserialize, Serialize};

/// 创建分组请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateSectionBody {
    /// 分组标题
    pub summary: String,

    /// 分组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 更新分组请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateSectionBody {
    /// 分组标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// 分组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 创建分组响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSectionResponse {
    /// 分组 GUID
    pub section_guid: String,

    /// 分组标题
    pub summary: String,

    /// 分组描述
    #[serde(default)]
    pub description: Option<String>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

/// 获取分组响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetSectionResponse {
    /// 分组 GUID
    pub section_guid: String,

    /// 分组标题
    pub summary: String,

    /// 分组描述
    #[serde(default)]
    pub description: Option<String>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

/// 更新分组响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSectionResponse {
    /// 分组 GUID
    pub section_guid: String,

    /// 分组标题
    pub summary: String,

    /// 分组描述
    #[serde(default)]
    pub description: Option<String>,

    /// 更新时间
    pub updated_at: String,
}

/// 删除分组响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteSectionResponse {
    /// 是否删除成功
    pub success: bool,

    /// 分组 GUID
    pub section_guid: String,
}

/// 分组列表项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SectionItem {
    /// 分组 GUID
    pub section_guid: String,

    /// 分组标题
    pub summary: String,

    /// 分组描述
    #[serde(default)]
    pub description: Option<String>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

/// 获取分组列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListSectionsResponse {
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,

    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,

    /// 列表项
    #[serde(default)]
    pub items: Vec<SectionItem>,
}
