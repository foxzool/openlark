//! 任务清单 API v2 的数据模型

use serde::{Deserialize, Serialize};

/// 创建任务清单请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateTasklistBody {
    /// 任务清单标题
    pub summary: String,

    /// 任务清单描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 任务清单图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<TasklistIcon>,
}

/// 任务清单图标
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum TasklistIcon {
    /// 默认图标。
    #[serde(rename = "default")]
    Default {
        /// 默认图标索引。
        index: i32,
    },
    /// Emoji 图标。
    #[serde(rename = "emoji")]
    Emoji {
        /// Emoji 文本。
        emoji: String,
    },
    /// 图片图标。
    #[serde(rename = "image")]
    Image {
        /// 图片资源 key。
        image_key: String,
    },
}

/// 更新任务清单请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateTasklistBody {
    /// 任务清单标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// 任务清单描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 任务清单图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<TasklistIcon>,
}

/// 创建任务清单响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTasklistResponse {
    /// 任务清单 GUID
    pub tasklist_guid: String,

    /// 任务清单标题
    pub summary: String,

    /// 任务清单描述
    #[serde(default)]
    pub description: Option<String>,

    /// 任务清单图标
    #[serde(default)]
    pub icon: Option<TasklistIcon>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

/// 获取任务清单响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetTasklistResponse {
    /// 任务清单 GUID
    pub tasklist_guid: String,

    /// 任务清单标题
    pub summary: String,

    /// 任务清单描述
    #[serde(default)]
    pub description: Option<String>,

    /// 任务清单图标
    #[serde(default)]
    pub icon: Option<TasklistIcon>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

/// 更新任务清单响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTasklistResponse {
    /// 任务清单 GUID
    pub tasklist_guid: String,

    /// 任务清单标题
    pub summary: String,

    /// 任务清单描述
    #[serde(default)]
    pub description: Option<String>,

    /// 任务清单图标
    #[serde(default)]
    pub icon: Option<TasklistIcon>,

    /// 更新时间
    pub updated_at: String,
}

/// 删除任务清单响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTasklistResponse {
    /// 是否删除成功
    pub success: bool,

    /// 任务清单 GUID
    pub tasklist_guid: String,
}

/// 任务清单列表项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TasklistItem {
    /// 任务清单 GUID
    pub tasklist_guid: String,

    /// 任务清单标题
    pub summary: String,

    /// 任务清单描述
    #[serde(default)]
    pub description: Option<String>,

    /// 任务清单图标
    #[serde(default)]
    pub icon: Option<TasklistIcon>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

/// 获取任务清单列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListTasklistsResponse {
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
    pub items: Vec<TasklistItem>,
}
