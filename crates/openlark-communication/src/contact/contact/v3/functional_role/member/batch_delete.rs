//! 删除角色下的成员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/batch_delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::{
        functional_role::member::models::BatchDeleteMembersResponse, user::models::UserIdType,
    },
    endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
};

/// 删除角色下的成员请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteMembersBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

impl BatchDeleteMembersBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn member(mut self, member_id: impl Into<String>) -> Self {
        self.members
            .get_or_insert_with(Vec::new)
            .push(member_id.into());
        self
    }
}

/// 删除角色下的成员请求
///
/// 用于从指定角色批量删除成员。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `role_id`: 角色 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选）
///
/// # 请求体字段
///
/// - `members`: 用户 ID 列表（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = BatchDeleteMembersBody::new()
///     .member("user_1")
///     .member("user_2");
/// let request = BatchDeleteRoleMembersRequest::new(config)
///     .role_id("role_xxx")
///     .user_id_type(UserIdType::OpenId)
///     .execute(body).await?;
/// ```
pub struct BatchDeleteRoleMembersRequest {
    config: Config,
    role_id: String,
    user_id_type: Option<UserIdType>,
}

impl BatchDeleteRoleMembersRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            role_id: String::new(),
            user_id_type: None,
        }
    }

    /// 角色 ID（路径参数）
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/batch_delete>
    pub async fn execute(
        self,
        body: BatchDeleteMembersBody,
    ) -> SDKResult<BatchDeleteMembersResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: BatchDeleteMembersBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchDeleteMembersResponse> {
        // === 必填字段验证 ===
        validate_required!(self.role_id, "role_id 不能为空");

        // url: PATCH:/open-apis/contact/v3/functional_roles/:role_id/members/batch_delete
        let mut req: ApiRequest<BatchDeleteMembersResponse> = ApiRequest::patch(format!(
            "{}/{}/members/batch_delete",
            CONTACT_V3_FUNCTIONAL_ROLES, self.role_id
        ))
        .body(serialize_params(&body, "删除角色下的成员")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "删除角色下的成员")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_delete_role_members_request_builder() {
        let config = Config::default();
        let request = BatchDeleteRoleMembersRequest::new(config).role_id("role_xxx");
        assert_eq!(request.role_id, "role_xxx");
    }

    #[test]
    fn test_batch_delete_members_body_builder() {
        let body = BatchDeleteMembersBody::new()
            .member("user_1")
            .member("user_2");
        assert_eq!(body.members.as_ref().map(|v| v.len()), Some(2));
        assert_eq!(body.members.as_ref().map(|v| v[0].as_str()), Some("user_1"));
    }

    #[test]
    fn test_batch_delete_members_body_default() {
        let body = BatchDeleteMembersBody::new();
        assert_eq!(body.members, None);
    }

    #[test]
    fn test_batch_delete_role_members_request_with_user_id_type() {
        let config = Config::default();
        let request = BatchDeleteRoleMembersRequest::new(config)
            .role_id("role_xxx")
            .user_id_type(UserIdType::UnionId);
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }

    #[test]
    fn test_batch_delete_role_members_request_default_values() {
        let config = Config::default();
        let request = BatchDeleteRoleMembersRequest::new(config);
        assert_eq!(request.role_id, "");
        assert_eq!(request.user_id_type, None);
    }
}
