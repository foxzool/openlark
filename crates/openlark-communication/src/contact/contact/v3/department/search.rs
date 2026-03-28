//! 搜索部门
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/department/search>

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::{
        department::models::DepartmentListResponse,
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_DEPARTMENTS_SEARCH,
};

/// 搜索部门请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDepartmentsBody {
    pub query: String,
}

impl SearchDepartmentsBody {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
        }
    }
}

/// 搜索部门请求
///
/// 用于根据关键词搜索部门。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
/// - `page_size`: 分页大小（可选，默认 20，最大 50）
/// - `page_token`: 分页标记（可选）
///
/// # 请求体字段
///
/// - `query`: 搜索关键词，必填
///
/// # 示例
///
/// ```rust,ignore
/// let body = SearchDepartmentsBody::new("研发");
/// let request = SearchDepartmentsRequest::new(config)
///     .page_size(20)
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct SearchDepartmentsRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl SearchDepartmentsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            department_id_type: None,
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

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 分页大小（查询参数，可选，默认 20，最大 50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/department/search>
    pub async fn execute(self, body: SearchDepartmentsBody) -> SDKResult<DepartmentListResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: SearchDepartmentsBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DepartmentListResponse> {
        // === 必填字段验证 ===
        validate_required!(body.query, "query 不能为空");
        if body.query.trim().is_empty() {
            return Err(error::validation_error(
                "query 不能为空".to_string(),
                "搜索关键词不可为空字符串".to_string(),
            ));
        }
        if let Some(page_size) = self.page_size {
            if !(1..=50).contains(&page_size) {
                return Err(error::validation_error(
                    "page_size 不合法".to_string(),
                    "page_size 取值范围为 1~50".to_string(),
                ));
            }
        }

        // url: POST:/open-apis/contact/v3/departments/search
        let mut req: ApiRequest<DepartmentListResponse> =
            ApiRequest::post(CONTACT_V3_DEPARTMENTS_SEARCH)
                .body(serialize_params(&body, "搜索部门")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "搜索部门")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_search_departments_request_builder() {
        let config = Config::default();
        let request = SearchDepartmentsRequest::new(config);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_search_departments_request_with_user_id_type() {
        let config = Config::default();
        let request = SearchDepartmentsRequest::new(config).user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_search_departments_body_builder() {
        let body = SearchDepartmentsBody::new("研发");
        assert_eq!(body.query, "研发");
    }

    #[test]
    fn test_search_departments_request_with_page_size() {
        let config = Config::default();
        let request = SearchDepartmentsRequest::new(config).page_size(30);
        assert_eq!(request.page_size, Some(30));
    }

    #[test]
    fn test_search_departments_request_with_all_options() {
        let config = Config::default();
        let request = SearchDepartmentsRequest::new(config)
            .user_id_type(UserIdType::UnionId)
            .department_id_type(DepartmentIdType::OpenDepartmentId)
            .page_size(50)
            .page_token("token789");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::OpenDepartmentId)
        );
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token789".to_string()));
    }
}
