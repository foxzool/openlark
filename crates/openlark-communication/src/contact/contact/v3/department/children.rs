//! 获取子部门列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/department/children>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::{
        department::models::DepartmentListResponse,
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_DEPARTMENTS,
};

/// 获取子部门列表请求
///
/// 用于获取指定部门的直接子部门列表。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `department_id`: 部门 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
/// - `page_size`: 分页大小（可选，默认 10，最大 50）
/// - `page_token`: 分页标记（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let request = ListDepartmentChildrenRequest::new(config)
///     .department_id("dept_xxx")
///     .page_size(20)
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct ListDepartmentChildrenRequest {
    config: Config,
    department_id: String,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListDepartmentChildrenRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id: String::new(),
            user_id_type: None,
            department_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 部门 ID（路径参数）
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = department_id.into();
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
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/department/children>
    pub async fn execute(self) -> SDKResult<DepartmentListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DepartmentListResponse> {
        // === 必填字段验证 ===
        validate_required!(self.department_id, "department_id 不能为空");

        let mut req: ApiRequest<DepartmentListResponse> = ApiRequest::get(format!(
            "{}/{}/children",
            CONTACT_V3_DEPARTMENTS, self.department_id
        ));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取子部门列表")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_list_department_children_request_builder() {
        let config = Config::default();
        let request = ListDepartmentChildrenRequest::new(config).department_id("dept_xxx");
        assert_eq!(request.department_id, "dept_xxx");
    }

    #[test]
    fn test_list_department_children_request_with_user_id_type() {
        let config = Config::default();
        let request = ListDepartmentChildrenRequest::new(config)
            .department_id("dept_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_list_department_children_request_with_page_size() {
        let config = Config::default();
        let request = ListDepartmentChildrenRequest::new(config)
            .department_id("dept_xxx")
            .page_size(20);
        assert_eq!(request.page_size, Some(20));
    }

    #[test]
    fn test_list_department_children_request_default_values() {
        let config = Config::default();
        let request = ListDepartmentChildrenRequest::new(config);
        assert_eq!(request.department_id, "");
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.page_size, None);
    }

    #[test]
    fn test_list_department_children_request_with_all_options() {
        let config = Config::default();
        let request = ListDepartmentChildrenRequest::new(config)
            .department_id("dept_456")
            .user_id_type(UserIdType::UnionId)
            .department_id_type(DepartmentIdType::OpenDepartmentId)
            .page_size(50)
            .page_token("token789");
        assert_eq!(request.department_id, "dept_456");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::OpenDepartmentId)
        );
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token789".to_string()));
    }
}
