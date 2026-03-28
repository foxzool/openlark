//! 获取单个用户信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/get>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::user::create::UserResponse,
    contact::contact::v3::user::models::{DepartmentIdType, UserIdType},
    endpoints::CONTACT_V3_USERS,
};

/// 获取单个用户信息请求
///
/// 用于根据用户 ID 获取用户详细信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id`: 用户 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetUserRequest::new(config)
///     .user_id("user_xxx")
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct GetUserRequest {
    config: Config,
    user_id: String,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
}

impl GetUserRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id: String::new(),
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 用户 ID（路径参数）
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
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
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/get>
    pub async fn execute(self) -> SDKResult<UserResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserResponse> {
        // === 必填字段验证 ===
        validate_required!(self.user_id, "user_id 不能为空");

        // url: GET:/open-apis/contact/v3/users/:user_id
        let mut req: ApiRequest<UserResponse> =
            ApiRequest::get(format!("{}/{}", CONTACT_V3_USERS, self.user_id));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取单个用户信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_request_builder() {
        let config = Config::default();
        let request = GetUserRequest::new(config).user_id("user_xxx");
        assert_eq!(request.user_id, "user_xxx");
    }

    #[test]
    fn test_get_user_request_with_user_id_type() {
        let config = Config::default();
        let request = GetUserRequest::new(config)
            .user_id("user_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_get_user_request_with_department_id_type() {
        let config = Config::default();
        let request = GetUserRequest::new(config)
            .user_id("user_xxx")
            .department_id_type(DepartmentIdType::DepartmentId);
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::DepartmentId)
        );
    }

    #[test]
    fn test_get_user_request_default_values() {
        let config = Config::default();
        let request = GetUserRequest::new(config);
        assert_eq!(request.user_id, "");
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_get_user_request_with_all_options() {
        let config = Config::default();
        let request = GetUserRequest::new(config)
            .user_id("user_123")
            .user_id_type(UserIdType::UnionId)
            .department_id_type(DepartmentIdType::OpenDepartmentId);
        assert_eq!(request.user_id, "user_123");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::OpenDepartmentId)
        );
    }
}
