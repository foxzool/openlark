//! stage_task 共享响应模型
//!
//! 官方 `semester_base_info` / `stage_task` / `stage_task_status_num` /
//! `stage_task_info` 结构，被 find_by_page 与 find_by_user_list 两个端点共享。
//!
//! 文档字段表写 `task_option_id`，而 find_by_user_list 响应示例写 `task_option`
//! （find_by_page 示例为 `task_option_id`），此处通过 `rename` + `alias` 兼容两种键名。

use crate::performance::performance::v1::models::I18nName;
use serde::{Deserialize, Serialize};

/// 周期基本信息（官方 semester_base_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SemesterBaseInfo {
    /// 周期 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    /// 周期名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_name: Option<I18nName>,
    /// 周期开始时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 周期结束时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

/// 周期任务（官方 stage_task 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StageTask {
    /// 用户 ID，与入参 user_id_type 类型一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 各分类的任务数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_num_lists: Option<Vec<StageTaskStatusNum>>,
    /// 环节任务信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_task_info_lists: Option<Vec<StageTaskInfo>>,
}

/// 各分类的任务数（官方 stage_task_status_num 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StageTaskStatusNum {
    /// 任务分类：1 待完成 / 2 已完成 / 3 已逾期
    #[serde(
        rename = "task_option_id",
        alias = "task_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_option_id: Option<i32>,
    /// 环节任务数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_num: Option<i32>,
}

/// 环节任务信息（官方 stage_task_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StageTaskInfo {
    /// 环节 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
    /// 环节名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nName>,
    /// 环节截止时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    /// 未完成的任务数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub need_todo_count: Option<i32>,
    /// 处理任务的系统页面链接
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump_url: Option<String>,
    /// 环节任务状态：need_todo / overdue / all_done / stage_pause
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_task_status: Option<String>,
    /// 任务分类：1 待完成 / 2 已完成 / 3 已逾期
    #[serde(
        rename = "task_option_id",
        alias = "task_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub task_option_id: Option<i32>,
}
