//! 获取用户列表
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::user::models::{DepartmentIdType, User, UserIdType},
    endpoints::CONTACT_V3_USERS,
};

/// 获取用户列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersResponse {
    #[serde(default)]
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub items: Vec<User>,
}

impl ApiResponseTrait for ListUsersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取用户列表请求
///
/// 用于分页获取通讯录中的用户列表。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
/// - `department_id`: 部门 ID（可选）
/// - `page_token`: 分页标记（可选）
/// - `page_size`: 分页大小（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let request = ListUsersRequest::new(config)
///     .department_id("dept_xxx")
///     .page_size(50)
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct ListUsersRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
    department_id: Option<String>,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl ListUsersRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            department_id_type: None,
            department_id: None,
            page_token: None,
            page_size: None,
        }
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

    /// 部门 ID（查询参数，可选）
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = Some(department_id.into());
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 分页大小（查询参数，可选）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/list>
    pub async fn execute(self) -> SDKResult<ListUsersResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListUsersResponse> {
        let mut req: ApiRequest<ListUsersResponse> = ApiRequest::get(CONTACT_V3_USERS);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(department_id) = self.department_id {
            req = req.query("department_id", department_id);
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取用户列表")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_list_users_request_builder() {
        let config = Config::default();
        let request = ListUsersRequest::new(config);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_list_users_request_with_user_id_type() {
        let config = Config::default();
        let request = ListUsersRequest::new(config).user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_list_users_request_with_department_id() {
        let config = Config::default();
        let request = ListUsersRequest::new(config).department_id("dept_xxx");
        assert_eq!(request.department_id, Some("dept_xxx".to_string()));
    }

    #[test]
    fn test_list_users_request_with_page_size() {
        let config = Config::default();
        let request = ListUsersRequest::new(config).page_size(50);
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_list_users_request_with_page_token() {
        let config = Config::default();
        let request = ListUsersRequest::new(config).page_token("token123");
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_list_users_request_with_all_options() {
        let config = Config::default();
        let request = ListUsersRequest::new(config)
            .user_id_type(UserIdType::UnionId)
            .department_id_type(DepartmentIdType::OpenDepartmentId)
            .department_id("dept_456")
            .page_size(100)
            .page_token("token789");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::OpenDepartmentId)
        );
        assert_eq!(request.department_id, Some("dept_456".to_string()));
        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.page_token, Some("token789".to_string()));
    }
}
