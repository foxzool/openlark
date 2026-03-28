//! 批量添加角色成员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/batch_create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::{
        functional_role::member::models::BatchCreateMembersResponse, user::models::UserIdType,
    },
    endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
};

/// 批量添加角色成员请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCreateMembersBody {
    pub members: Vec<String>,
}

impl BatchCreateMembersBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn member(mut self, member_id: impl Into<String>) -> Self {
        self.members.push(member_id.into());
        self
    }
}

/// 批量添加角色成员请求
///
/// 用于向指定角色批量添加成员。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `role_id`: 角色 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选）
///
/// # 请求体字段
///
/// - `members`: 用户 ID 列表，至少包含 1 个用户 ID
///
/// # 示例
///
/// ```rust,ignore
/// let body = BatchCreateMembersBody::new()
///     .member("user_1")
///     .member("user_2");
/// let request = BatchCreateRoleMembersRequest::new(config)
///     .role_id("role_xxx")
///     .user_id_type(UserIdType::OpenId)
///     .execute(body).await?;
/// ```
pub struct BatchCreateRoleMembersRequest {
    config: Config,
    role_id: String,
    user_id_type: Option<UserIdType>,
}

impl BatchCreateRoleMembersRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/batch_create>
    pub async fn execute(
        self,
        body: BatchCreateMembersBody,
    ) -> SDKResult<BatchCreateMembersResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: BatchCreateMembersBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchCreateMembersResponse> {
        // === 必填字段验证 ===
        validate_required!(self.role_id, "role_id 不能为空");
        if body.members.is_empty() {
            return Err(openlark_core::error::validation_error(
                "members 不能为空".to_string(),
                "members 至少需要包含 1 个用户 ID".to_string(),
            ));
        }

        // url: POST:/open-apis/contact/v3/functional_roles/:role_id/members/batch_create
        let mut req: ApiRequest<BatchCreateMembersResponse> = ApiRequest::post(format!(
            "{}/{}/members/batch_create",
            CONTACT_V3_FUNCTIONAL_ROLES, self.role_id
        ))
        .body(serialize_params(&body, "批量添加角色成员")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "批量添加角色成员")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_create_role_members_request_builder() {
        let config = Config::default();
        let request = BatchCreateRoleMembersRequest::new(config).role_id("role_xxx");
        assert_eq!(request.role_id, "role_xxx");
    }

    #[test]
    fn test_batch_create_members_body_builder() {
        let body = BatchCreateMembersBody::new()
            .member("user_1")
            .member("user_2");
        assert_eq!(body.members.len(), 2);
        assert_eq!(body.members[0], "user_1");
    }

    #[test]
    fn test_batch_create_members_body_default() {
        let body = BatchCreateMembersBody::new();
        assert_eq!(body.members.len(), 0);
    }

    #[test]
    fn test_batch_create_role_members_request_with_user_id_type() {
        let config = Config::default();
        let request = BatchCreateRoleMembersRequest::new(config)
            .role_id("role_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_batch_create_role_members_request_default_values() {
        let config = Config::default();
        let request = BatchCreateRoleMembersRequest::new(config);
        assert_eq!(request.role_id, "");
        assert_eq!(request.user_id_type, None);
    }
}
