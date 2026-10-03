//! system_status 共享模型（对齐飞书 personal_settings-v1）

use serde::{Deserialize, Serialize};

/// 系统状态多语言名称（`system_status_i18n_name` / sync i18n）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemStatusI18nName {
    /// 中文名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    /// 日文名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

/// 系统状态同步设置（`system_status_sync_setting`）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SystemStatusSyncSetting {
    /// 是否默认开启。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open_by_default: Option<bool>,
    /// 同步设置名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 同步设置国际化名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<SystemStatusI18nName>,
    /// 同步设置解释文案。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explain: Option<String>,
    /// 同步设置国际化解释文案。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_explain: Option<SystemStatusI18nName>,
}

/// 系统状态实体（请求/响应共用字段子集）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SystemStatus {
    /// 系统状态 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_status_id: Option<String>,
    /// 系统状态标题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 系统状态国际化标题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<SystemStatusI18nName>,
    /// 图标 key。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_key: Option<String>,
    /// 颜色。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// 优先级（越小越优先）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 同步设置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_setting: Option<SystemStatusSyncSetting>,
}

/// 批量开启时的用户参数（`system_status_user_open_param`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemStatusUserOpenParam {
    /// 用户 ID（类型由查询参数 `user_id_type` 决定）。
    pub user_id: String,
    /// 结束时间（Unix 秒级时间戳字符串）。
    pub end_time: String,
}

/// 批量开启结果项。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemStatusUserOpenResult {
    /// 用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 结束时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开启结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

/// 批量关闭结果项。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemStatusUserCloseResult {
    /// 用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 关闭结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}
