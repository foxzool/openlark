//! 任务清单 API v2 的数据模型

use serde::{Deserialize, Serialize};

/// 创建任务清单请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateTasklistBody {
    /// 清单名称（必填，最多 100 个字符）
    pub name: String,

    /// 清单的成员列表（可选，最多 500 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TasklistMemberDetail>>,
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

/// 清单成员信息（member 对象，v2）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TasklistMemberDetail {
    /// 成员 ID
    #[serde(default)]
    pub id: Option<String>,

    /// 成员类型（"user" 或 "app"；所有者类型不可为 "chat"）
    #[serde(default)]
    pub r#type: Option<String>,

    /// 成员角色（更新所有者时必须为 "owner"）
    #[serde(default)]
    pub role: Option<String>,

    /// 成员名称
    #[serde(default)]
    pub name: Option<String>,
}

/// 待更新的清单数据（input_tasklist，v2）
#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub struct InputTasklist {
    /// 清单名称（如要更新，不能设为空，最大 100 个字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 更新的清单所有者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<TasklistMemberDetail>,
}

/// 更新任务清单请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateTasklistBody {
    /// 要更新清单的数据
    pub tasklist: InputTasklist,

    /// 要更新的字段名，支持 name（更新清单名）/ owner（更新清单所有者），最小长度 1
    pub update_fields: Vec<String>,

    /// 更新所有者后原所有者的新角色（editor / viewer / none，默认 none；仅更新所有者时生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_owner_to_role: Option<String>,
}

/// 创建任务清单响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTasklistResponse {
    /// 创建的清单数据
    pub tasklist: TasklistDetail,
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

/// 任务清单详情（v2，响应内嵌 tasklist 对象）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct TasklistDetail {
    /// 清单的全局唯一 ID
    #[serde(default)]
    pub guid: Option<String>,

    /// 清单名
    #[serde(default)]
    pub name: Option<String>,

    /// 清单创建者
    #[serde(default)]
    pub creator: Option<TasklistMemberDetail>,

    /// 清单所有者
    #[serde(default)]
    pub owner: Option<TasklistMemberDetail>,

    /// 清单协作人
    #[serde(default)]
    pub members: Vec<TasklistMemberDetail>,

    /// 该清单分享的 applink
    #[serde(default)]
    pub url: Option<String>,

    /// 清单创建时间戳（毫秒）
    #[serde(default)]
    pub created_at: Option<String>,

    /// 清单最后一次更新时间戳（毫秒）
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// 更新任务清单响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTasklistResponse {
    /// 修改后的任务清单
    pub tasklist: TasklistDetail,
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
