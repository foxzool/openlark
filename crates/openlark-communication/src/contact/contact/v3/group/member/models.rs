//! 用户组成员相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 用户组成员信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupMember {
    /// 成员 ID。
    pub member_id: String,
    /// 成员类型。
    pub member_type: String,
    /// 成员 ID 类型。
    pub member_id_type: String,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 查询用户组成员列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimpleListGroupMembersResponse {
    /// 成员列表。
    #[serde(default)]
    pub memberlist: Vec<GroupMember>,
    /// 分页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    #[serde(default)]
    pub has_more: bool,
}

impl ApiResponseTrait for SimpleListGroupMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量添加用户组成员结果项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchAddResult {
    /// 成员 ID。
    pub member_id: String,
    /// 结果码。
    pub code: i32,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 批量添加用户组成员响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchAddGroupMembersResponse {
    /// 批量添加结果列表。
    #[serde(default)]
    pub results: Vec<BatchAddResult>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for BatchAddGroupMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
