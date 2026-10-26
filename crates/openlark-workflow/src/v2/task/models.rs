//! 任务 API v2 的数据模型

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 任务成员（member）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskMember {
    /// 成员 ID
    #[serde(default)]
    pub id: String,
    /// 成员类型（如 "user"）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 成员角色（"assignee" 执行者 / "follower" 关注者）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 成员名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 任务时间点（start/due 对象）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskTimePoint {
    /// 时间戳（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 是否全天
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
}

/// 任务所在清单信息（task_in_tasklist_info）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskInTasklistInfo {
    /// 任务清单 GUID
    #[serde(default)]
    pub tasklist_guid: String,
    /// 分组 GUID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_guid: Option<String>,
}

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

/// 自定义字段值（input_custom_field_value，更新任务时传入）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InputCustomFieldValue {
    /// 自定义字段 GUID（必填）
    #[serde(default)]
    pub guid: String,

    /// 数字类型字段值（合法数字的字符串表示，空字符串表示设为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_value: Option<String>,

    /// 人员类型字段值（1-50 个 member，仅支持 user 类型；空数组表示设为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_value: Option<Vec<TaskMember>>,

    /// 日期类型字段值（毫秒时间戳字符串，空字符串表示设为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datetime_value: Option<String>,

    /// 单选类型字段值（字段选项的 option_guid，空字符串表示设为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_select_value: Option<String>,

    /// 多选类型字段值（option_guid 列表，空数组表示设为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_select_value: Option<Vec<String>>,

    /// 文本类型字段值（最长 3000 字符，空字符串表示设为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
}

/// 要更新的任务数据（input_task，只设置出现在 update_fields 中的字段）
#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub struct InputTask {
    /// 任务标题（如更新标题，不可设为空，最长 3000 个 UTF-8 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// 任务描述（最长 3000 个 UTF-8 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 任务开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<TaskTimePoint>,

    /// 任务截止时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskTimePoint>,

    /// 调用者附带的任意数据（获取任务详情时原样返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,

    /// 任务的完成时刻时间戳（毫秒字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,

    /// 重复任务规则（RRule 语法）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_rule: Option<String>,

    /// 任务自定义完成配置（pc/ios/android 各端 {href, tip{i18n}} 嵌套结构，按官方文档透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_complete: Option<Value>,

    /// 任务的完成模式（1-会签任务；2-或签任务）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,

    /// 是否是里程碑任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_milestone: Option<bool>,

    /// 自定义字段值列表（按字段类型填写对应的 *_value 字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<InputCustomFieldValue>>,
}

/// 更新任务请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateTaskBody {
    /// 要更新的任务数据（只设置出现在 update_fields 中的字段；
    /// update_fields 声明了某字段而 task 未设置新值表示将该字段清空）
    pub task: InputTask,

    /// 要更新的字段名列表，支持：summary / description / start / due / completed_at /
    /// extra / custom_complete / repeat_rule / mode / is_milestone / custom_fields
    pub update_fields: Vec<String>,
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
