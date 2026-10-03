//! 任务 API v2 的数据模型

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 创建任务请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateTaskBody {
    /// 任务标题
    pub summary: String,

    /// 任务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 任务开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,

    /// 任务截止时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<String>,

    /// 任务所属的任务清单 GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasklist_guid: Option<String>,

    /// 任务所属的分组 GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_guid: Option<String>,

    /// 任务优先级（1-5）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Value>,

    /// 任务关注者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<String>>,

    /// 子任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<Value>>,

    /// 任务执行者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,

    /// 任务提醒时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remind_time: Option<String>,

    /// 重复规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_rule: Option<Value>,
}

/// 更新任务请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateTaskBody {
    /// 任务标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// 任务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 任务开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,

    /// 任务截止时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<String>,

    /// 任务优先级（1-5）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Value>,

    /// 任务关注者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<String>>,

    /// 子任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<Value>>,

    /// 任务执行者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,

    /// 任务提醒时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remind_time: Option<String>,

    /// 重复规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_rule: Option<Value>,

    /// 任务状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 创建任务响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTaskResponse {
    /// 任务 GUID
    pub task_guid: String,

    /// 任务标题
    pub summary: String,

    /// 任务描述
    #[serde(default)]
    pub description: Option<String>,

    /// 任务状态
    pub status: String,

    /// 任务清单 GUID
    #[serde(default)]
    pub tasklist_guid: Option<String>,

    /// 分组 GUID
    #[serde(default)]
    pub section_guid: Option<String>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

/// 获取任务响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetTaskResponse {
    /// 任务 GUID
    pub task_guid: String,

    /// 任务标题
    pub summary: String,

    /// 任务描述
    #[serde(default)]
    pub description: Option<String>,

    /// 任务状态
    pub status: String,

    /// 任务清单 GUID
    #[serde(default)]
    pub tasklist_guid: Option<String>,

    /// 分组 GUID
    #[serde(default)]
    pub section_guid: Option<String>,

    /// 任务优先级
    #[serde(default)]
    pub priority: Option<i32>,

    /// 任务开始时间
    #[serde(default)]
    pub start: Option<String>,

    /// 任务截止时间
    #[serde(default)]
    pub due: Option<String>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,

    /// 完成时间
    #[serde(default)]
    pub completed_at: Option<String>,
}

/// 更新任务响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTaskResponse {
    /// 任务 GUID
    pub task_guid: String,

    /// 任务标题
    pub summary: String,

    /// 任务描述
    #[serde(default)]
    pub description: Option<String>,

    /// 任务状态
    pub status: String,

    /// 更新时间
    pub updated_at: String,
}

/// 删除任务响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTaskResponse {
    /// 是否删除成功
    pub success: bool,

    /// 任务 GUID
    pub task_guid: String,
}

/// 完成任务响应
#[derive(Debug, Clone, Deserialize)]
pub struct CompleteTaskResponse {
    /// 任务 GUID
    pub task_guid: String,

    /// 任务状态
    pub status: String,

    /// 完成时间
    pub completed_at: String,
}

/// 取消完成任务响应
#[derive(Debug, Clone, Deserialize)]
pub struct UncompleteTaskResponse {
    /// 任务 GUID
    pub task_guid: String,

    /// 任务状态
    pub status: String,

    /// 更新时间
    pub updated_at: String,
}

/// 任务列表项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskItem {
    /// 任务 GUID
    pub task_guid: String,

    /// 任务标题
    pub summary: String,

    /// 任务描述
    #[serde(default)]
    pub description: Option<String>,

    /// 任务状态
    pub status: String,

    /// 任务清单 GUID
    #[serde(default)]
    pub tasklist_guid: Option<String>,

    /// 分组 GUID
    #[serde(default)]
    pub section_guid: Option<String>,

    /// 任务优先级
    #[serde(default)]
    pub priority: Option<i32>,

    /// 任务开始时间
    #[serde(default)]
    pub start: Option<String>,

    /// 任务截止时间
    #[serde(default)]
    pub due: Option<String>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,

    /// 完成时间
    #[serde(default)]
    pub completed_at: Option<String>,

    /// 任务执行者
    #[serde(default)]
    pub assignee: Option<String>,

    /// 任务创建者
    #[serde(default)]
    pub creator: Option<String>,
}

/// 获取任务列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListTasksResponse {
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
    pub items: Vec<TaskItem>,
}
