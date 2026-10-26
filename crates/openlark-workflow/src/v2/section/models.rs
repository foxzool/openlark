//! 分组 API v2 的数据模型

use serde::{Deserialize, Serialize};

/// 创建分组请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateSectionBody {
    /// 自定义分组名（必填，不允许为空，最大 100 个 UTF-8 字符）
    pub name: String,

    /// 自定义分组的资源类型（必填，支持 "tasklist"（清单）或 "my_tasks"（我负责的））
    pub resource_type: String,

    /// 归属资源 ID（resource_type 为 "tasklist" 时必填清单 GUID；"my_tasks" 无需填写）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,

    /// 插入到该分组之前的分组 GUID（与 insert_after 互斥）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,

    /// 插入到该分组之后的分组 GUID（与 insert_before 互斥）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
}

/// 待更新的自定义分组数据（input_section）
#[derive(Debug, Clone, Serialize, Default)]
pub struct InputSection {
    /// 自定义分组名（如更新，不允许设为空，最大 100 个 UTF-8 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 插入到该分组之前的分组 GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,

    /// 插入到该分组之后的分组 GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
}

/// 更新分组请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateSectionBody {
    /// 要更新的自定义分组数据
    pub section: InputSection,

    /// 要更新的字段名，支持 name / insert_before / insert_after（1-10 个）
    pub update_fields: Vec<String>,
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
