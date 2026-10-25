//! performance v1 共享模型
//!
//! v1 接口的多语言名称与用户 ID 容器。与 v2 不同，v1 响应的 i18n wire 键为
//! `zh-CN` / `en-US`（连字符加大写），不能复用 `common::shared_models::I18nText`
//! （`zh_cn` / `en_us` 键），故在本模块单独建模。

use serde::{Deserialize, Serialize};

/// v1 多语言名称（官方 i18n 结构，wire 键为 `zh-CN` / `en-US`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct I18nName {
    /// 中文名称
    #[serde(
        rename = "zh-CN",
        alias = "zh_cn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zh_cn: Option<String>,
    /// 英文名称
    #[serde(
        rename = "en-US",
        alias = "en_us",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub en_us: Option<String>,
}

/// 用户 ID 容器（官方 user 结构：`{open_id, user_id}`）
///
/// `user_id` 与入参 `user_id_type` 类型一致，且受字段权限
/// `contact:user.employee_id:readonly` 门控，未开启权限时不返回。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UserId {
    /// 用户的 open_id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 用户的 user_id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
