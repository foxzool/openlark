/// Bitable V1 自定义角色协作者（member）数据模型
///
/// 注意：该文件仅存放模型结构，不计入 API 文件数量。
use serde::{Deserialize, Serialize};

/// 协作者 ID 类型（用于请求参数与批量成员列表）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RoleMemberIdType {
    #[default]
    /// open_id。
    OpenId,
    /// union_id。
    UnionId,
    /// user_id。
    UserId,
    /// chat_id。
    ChatId,
    /// department_id。
    DepartmentId,
    /// open_department_id。
    OpenDepartmentId,
}

/// 协作者类型（列表响应）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RoleMemberType {
    /// 用户。
    User,
    /// 群聊。
    Chat,
    /// 部门。
    Department,
    /// 未知类型。
    #[serde(other)]
    Unknown,
}

/// 协作者 ID（批量新增/批量删除请求体内的成员项）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoleMemberId {
    /// 协作者 ID 类型
    #[serde(rename = "type", default)]
    pub id_type: RoleMemberIdType,
    /// 协作者 ID
    pub id: String,
}

/// 协作者信息（列出协作者响应 items）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleMemberInfo {
    /// 协作者类型：user/chat/department
    pub member_type: RoleMemberType,
    /// 协作者名称
    pub member_name: String,
    /// 协作者英文名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_en_name: Option<String>,

    /// 协作者为用户时，用户的 open_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 协作者为用户时，用户的 union_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    /// 协作者为用户时，用户的 user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// 协作者为群聊时，群聊的 chat_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,

    /// 协作者为部门时，部门的 department_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 协作者为部门时，部门的 open_department_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
}
