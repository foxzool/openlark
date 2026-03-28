//! 获取部门信息列表
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/list>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::{
        department::models::DepartmentListResponse,
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_DEPARTMENTS,
};

/// 获取部门信息列表请求
///
/// 用于分页获取通讯录中的部门列表。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
/// - `parent_department_id`: 父部门 ID（可选）
/// - `fetch_child`: 是否递归获取子部门（可选）
/// - `page_size`: 分页大小（可选）
/// - `page_token`: 分页标记（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let request = ListDepartmentsRequest::new(config)
///     .parent_department_id("dept_xxx")
///     .page_size(50)
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct ListDepartmentsRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
    parent_department_id: Option<String>,
    fetch_child: Option<bool>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListDepartmentsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            department_id_type: None,
            parent_department_id: None,
            fetch_child: None,
            page_size: None,
            page_token: None,
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

    /// 父部门 ID（查询参数，可选）
    pub fn parent_department_id(mut self, parent_department_id: impl Into<String>) -> Self {
        self.parent_department_id = Some(parent_department_id.into());
        self
    }

    /// 是否递归获取子部门（查询参数，可选）
    pub fn fetch_child(mut self, fetch_child: bool) -> Self {
        self.fetch_child = Some(fetch_child);
        self
    }

    /// 分页大小（查询参数，可选，默认 10，最大 50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/list>
    pub async fn execute(self) -> SDKResult<DepartmentListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DepartmentListResponse> {
        let mut req: ApiRequest<DepartmentListResponse> = ApiRequest::get(CONTACT_V3_DEPARTMENTS);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(parent_department_id) = self.parent_department_id {
            req = req.query("parent_department_id", parent_department_id);
        }
        if let Some(fetch_child) = self.fetch_child {
            req = req.query("fetch_child", fetch_child.to_string());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取部门信息列表")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_list_departments_request_builder() {
        let config = Config::default();
        let request = ListDepartmentsRequest::new(config);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_list_departments_request_with_user_id_type() {
        let config = Config::default();
        let request = ListDepartmentsRequest::new(config).user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_list_departments_request_with_parent_department_id() {
        let config = Config::default();
        let request = ListDepartmentsRequest::new(config).parent_department_id("dept_xxx");
        assert_eq!(request.parent_department_id, Some("dept_xxx".to_string()));
    }

    #[test]
    fn test_list_departments_request_with_fetch_child() {
        let config = Config::default();
        let request = ListDepartmentsRequest::new(config).fetch_child(true);
        assert_eq!(request.fetch_child, Some(true));
    }

    #[test]
    fn test_list_departments_request_with_page_size() {
        let config = Config::default();
        let request = ListDepartmentsRequest::new(config).page_size(50);
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_list_departments_request_with_all_options() {
        let config = Config::default();
        let request = ListDepartmentsRequest::new(config)
            .user_id_type(UserIdType::UnionId)
            .department_id_type(DepartmentIdType::OpenDepartmentId)
            .parent_department_id("dept_456")
            .fetch_child(false)
            .page_size(100)
            .page_token("token789");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::OpenDepartmentId)
        );
        assert_eq!(request.parent_department_id, Some("dept_456".to_string()));
        assert_eq!(request.fetch_child, Some(false));
        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.page_token, Some("token789".to_string()));
    }
}
