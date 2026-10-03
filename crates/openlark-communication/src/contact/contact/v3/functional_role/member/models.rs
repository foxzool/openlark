//! 角色成员相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 角色成员信息（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionalRoleMember {
    /// 用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 管理范围类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// 部门 ID 列表。
    #[serde(default)]
    pub department_ids: Vec<String>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 查询角色下某个成员的管理范围响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetMemberResponse {
    /// 成员详情。
    pub member: FunctionalRoleMember,
}

impl ApiResponseTrait for GetMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询角色下的所有成员信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListMembersResponse {
    /// 成员列表。
    #[serde(default)]
    pub members: Vec<FunctionalRoleMember>,
    /// 分页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for ListMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量操作结果项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemberOperationResult {
    /// 用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 失败原因码。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<i32>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 批量添加角色成员响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateMembersResponse {
    /// 批量创建结果列表。
    #[serde(default)]
    pub results: Vec<MemberOperationResult>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for BatchCreateMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量设置角色成员管理范围响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchMembersScopesResponse {
    /// 批量设置范围结果列表。
    #[serde(default)]
    pub results: Vec<MemberOperationResult>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for PatchMembersScopesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除角色下的成员响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteMembersResponse {
    /// 批量删除结果列表。
    #[serde(default)]
    pub result: Vec<MemberOperationResult>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for BatchDeleteMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
