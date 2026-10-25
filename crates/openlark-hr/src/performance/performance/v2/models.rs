//! performance v2 共享模型
//!
//! v2 响应中的用户 ID 容器。多语言名称复用
//! `crate::common::shared_models::I18nText`（v2 的 i18n wire 键为 `zh_cn` / `en_us`）。

use serde::{Deserialize, Serialize};

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
