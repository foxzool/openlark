//! 批量设置角色成员管理范围
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/scopes>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::{
        functional_role::member::models::PatchMembersScopesResponse,
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
};

/// 批量设置角色成员管理范围请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchMembersScopesBody {
    pub members: Vec<String>,
    pub departments: Vec<String>,
}

impl PatchMembersScopesBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn member(mut self, member_id: impl Into<String>) -> Self {
        self.members.push(member_id.into());
        self
    }

    pub fn department(mut self, department_id: impl Into<String>) -> Self {
        self.departments.push(department_id.into());
        self
    }
}

/// 批量设置角色成员管理范围请求
///
/// 用于批量设置指定角色成员的管理范围。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `role_id`: 角色 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
///
/// # 请求体字段
///
/// - `members`: 用户 ID 列表，至少包含 1 个用户 ID
/// - `departments`: 部门 ID 列表，至少包含 1 个部门 ID
///
/// # 示例
///
/// ```rust,ignore
/// let body = PatchMembersScopesBody::new()
///     .member("user_1")
///     .member("user_2")
///     .department("dept_1")
///     .department("dept_2");
/// let request = PatchRoleMembersScopesRequest::new(config)
///     .role_id("role_xxx")
///     .execute(body).await?;
/// ```
pub struct PatchRoleMembersScopesRequest {
    config: Config,
    role_id: String,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
}

impl PatchRoleMembersScopesRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            role_id: String::new(),
            user_id_type: None,
            department_id_type: None,
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

    /// 部门 ID 类型（查询参数，可选）
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/scopes>
    pub async fn execute(
        self,
        body: PatchMembersScopesBody,
    ) -> SDKResult<PatchMembersScopesResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: PatchMembersScopesBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchMembersScopesResponse> {
        // === 必填字段验证 ===
        validate_required!(self.role_id, "role_id 不能为空");
        if body.members.is_empty() {
            return Err(openlark_core::error::validation_error(
                "members 不能为空".to_string(),
                "members 至少需要包含 1 个用户 ID".to_string(),
            ));
        }
        if body.departments.is_empty() {
            return Err(openlark_core::error::validation_error(
                "departments 不能为空".to_string(),
                "departments 至少需要包含 1 个部门 ID".to_string(),
            ));
        }

        // url: PATCH:/open-apis/contact/v3/functional_roles/:role_id/members/scopes
        let mut req: ApiRequest<PatchMembersScopesResponse> = ApiRequest::patch(format!(
            "{}/{}/members/scopes",
            CONTACT_V3_FUNCTIONAL_ROLES, self.role_id
        ))
        .body(serialize_params(&body, "批量设置角色成员管理范围")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "批量设置角色成员管理范围")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_role_members_scopes_request_builder() {
        let config = Config::default();
        let request = PatchRoleMembersScopesRequest::new(config).role_id("role_xxx");
        assert_eq!(request.role_id, "role_xxx");
    }

    #[test]
    fn test_patch_members_scopes_body_builder() {
        let body = PatchMembersScopesBody::new()
            .member("user_1")
            .member("user_2")
            .department("dept_1")
            .department("dept_2");
        assert_eq!(body.members.len(), 2);
        assert_eq!(body.departments.len(), 2);
        assert_eq!(body.members[0], "user_1");
        assert_eq!(body.departments[0], "dept_1");
    }

    #[test]
    fn test_patch_members_scopes_body_default() {
        let body = PatchMembersScopesBody::new();
        assert_eq!(body.members.len(), 0);
        assert_eq!(body.departments.len(), 0);
    }

    #[test]
    fn test_patch_role_members_scopes_request_with_user_id_type() {
        let config = Config::default();
        let request = PatchRoleMembersScopesRequest::new(config)
            .role_id("role_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_patch_role_members_scopes_request_with_department_id_type() {
        let config = Config::default();
        let request = PatchRoleMembersScopesRequest::new(config)
            .role_id("role_xxx")
            .department_id_type(DepartmentIdType::DepartmentId);
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::DepartmentId)
        );
    }

    #[test]
    fn test_patch_role_members_scopes_request_default_values() {
        let config = Config::default();
        let request = PatchRoleMembersScopesRequest::new(config);
        assert_eq!(request.role_id, "");
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_patch_role_members_scopes_request_with_all_options() {
        let config = Config::default();
        let request = PatchRoleMembersScopesRequest::new(config)
            .role_id("role_123")
            .user_id_type(UserIdType::UnionId)
            .department_id_type(DepartmentIdType::OpenDepartmentId);
        assert_eq!(request.role_id, "role_123");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::OpenDepartmentId)
        );
    }
}
