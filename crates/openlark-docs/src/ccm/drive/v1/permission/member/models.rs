use serde::{Deserialize, Serialize};

/// 协作者信息（基础字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMember {
    /// 协作者 ID 类型，与 member_id 对应
    ///
    /// 例如：`openid`、`email`、`unionid`、`openchat`、`opendepartmentid`、`groupid`、`userid`、`wikispaceid` 等。
    pub member_type: String,
    /// 协作者 ID
    pub member_id: String,
    /// 权限角色
    pub perm: String,
    /// 权限角色类型（知识库文档有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perm_type: Option<String>,
    /// 协作者类型
    ///
    /// **注意**：当 `member_type` 为 `wikispaceid` 时必填，且必须在
    /// `wiki_space_member`、`wiki_space_viewer`、`wiki_space_editor` 中选择。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl PermissionMember {
    /// 创建协作者信息
    pub fn new(
        member_type: impl Into<String>,
        member_id: impl Into<String>,
        perm: impl Into<String>,
    ) -> Self {
        Self {
            member_type: member_type.into(),
            member_id: member_id.into(),
            perm: perm.into(),
            perm_type: None,
            r#type: None,
        }
    }
}

/// 协作者信息（列表返回扩展字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMemberItem {
    /// 协作者 ID 类型
    pub member_type: String,
    /// 协作者 ID
    pub member_id: String,
    /// 权限角色
    pub perm: String,
    /// 权限角色类型（知识库文档有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perm_type: Option<String>,
    /// 协作者类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 协作者名称（字段权限不足时不返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 协作者头像（字段权限不足时不返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 协作者是否为外部成员（字段权限不足时不返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_label: Option<bool>,
}
