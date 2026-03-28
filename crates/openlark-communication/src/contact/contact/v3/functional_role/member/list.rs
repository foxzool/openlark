//! 查询角色下的所有成员信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/list>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::{
        functional_role::member::models::ListMembersResponse, user::models::UserIdType,
    },
    endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
};

/// 查询角色下的所有成员信息请求
///
/// 用于分页查询指定角色下的所有成员信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `role_id`: 角色 ID，必填
/// - `page_size`: 分页大小（可选）
/// - `page_token`: 分页标记（可选）
/// - `user_id_type`: 用户 ID 类型（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let request = ListRoleMembersRequest::new(config)
///     .role_id("role_xxx")
///     .page_size(50);
/// ```
pub struct ListRoleMembersRequest {
    config: Config,
    role_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<UserIdType>,
}

impl ListRoleMembersRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            role_id: String::new(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 角色 ID（路径参数）
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 分页大小（查询参数，可选）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/list>
    pub async fn execute(self) -> SDKResult<ListMembersResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListMembersResponse> {
        let mut req: ApiRequest<ListMembersResponse> = ApiRequest::get(format!(
            "{}/{}/members",
            CONTACT_V3_FUNCTIONAL_ROLES, self.role_id
        ));

        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询角色下的所有成员信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_list_role_members_request_builder() {
        let config = Config::default();
        let request = ListRoleMembersRequest::new(config).role_id("role_xxx");
        assert_eq!(request.role_id, "role_xxx");
    }

    #[test]
    fn test_list_role_members_request_with_page_size() {
        let config = Config::default();
        let request = ListRoleMembersRequest::new(config)
            .role_id("role_xxx")
            .page_size(50);
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_list_role_members_request_default_values() {
        let config = Config::default();
        let request = ListRoleMembersRequest::new(config);
        assert_eq!(request.role_id, "");
        assert_eq!(request.page_size, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_list_role_members_request_with_user_id_type() {
        let config = Config::default();
        let request = ListRoleMembersRequest::new(config)
            .role_id("role_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_list_role_members_request_with_all_options() {
        let config = Config::default();
        let request = ListRoleMembersRequest::new(config)
            .role_id("role_123")
            .page_size(100)
            .page_token("token789")
            .user_id_type(UserIdType::UnionId);
        assert_eq!(request.role_id, "role_123");
        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.page_token, Some("token789".to_string()));
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }
}
