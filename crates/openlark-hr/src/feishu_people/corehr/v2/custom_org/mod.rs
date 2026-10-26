pub mod active;
pub mod create;
pub mod delete_org;
pub mod patch;
pub mod query;
pub mod query_recent_change;
pub mod update_rule;

use serde::{Deserialize, Serialize};

/// 组织角色授权信息（官方 `org_role_update` 请求结构）。
///
/// 用于 [`create`] / [`patch`] 的 `org_roles` 字段。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct OrgRoleUpdate {
    /// 角色 key（api_name、security_group_id 必须填一个）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    /// 角色 ID（api_name、security_group_id 必须填一个）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 被授权的员工 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_ids: Option<Vec<String>>,
}
