//! 获取单位绑定的部门列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/list_department>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    contact::contact::v3::unit::models::ListUnitDepartmentsResponse,
    contact::contact::v3::user::models::DepartmentIdType,
    endpoints::CONTACT_V3_UNIT_LIST_DEPARTMENT,
};

/// 获取单位绑定的部门列表请求
///
/// 用于分页查询指定单位已经绑定的部门。
pub struct ListUnitDepartmentsRequest {
    config: Config,
    unit_id: String,
    department_id_type: Option<DepartmentIdType>,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl ListUnitDepartmentsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            unit_id: String::new(),
            department_id_type: None,
            page_token: None,
            page_size: None,
        }
    }

    /// 单位 ID（查询参数，必填）
    pub fn unit_id(mut self, unit_id: impl Into<String>) -> Self {
        self.unit_id = unit_id.into();
        self
    }

    /// 部门 ID 类型（查询参数，可选，默认 open_department_id）
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 分页大小（查询参数，可选，默认 50，范围 1~100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/list_department>
    pub async fn execute(self) -> SDKResult<ListUnitDepartmentsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListUnitDepartmentsResponse> {
        validate_required!(self.unit_id, "unit_id 不能为空");

        // url: GET:/open-apis/contact/v3/unit/list_department
        let mut req: ApiRequest<ListUnitDepartmentsResponse> =
            ApiRequest::get(CONTACT_V3_UNIT_LIST_DEPARTMENT).query("unit_id", self.unit_id);

        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        Transport::request_typed(req, &self.config, Some(option), "获取单位绑定的部门列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/contact/v3/unit/list_department
    #[tokio::test]
    async fn test_list_unit_departments_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/contact/v3/unit/list_department"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "departmentlist": [], "has_more": false }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        ListUnitDepartmentsRequest::new(config)
            .unit_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
