//! 通过员工 ID 获取入职信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/employee/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::EmployeeSummary;

/// 通过员工 ID 获取入职信息请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    employee_id: String,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            employee_id: String::new(),
        }
    }

    /// 设置 `employee_id`。
    pub fn employee_id(mut self, employee_id: String) -> Self {
        self.employee_id = employee_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.employee_id.trim(), "员工 ID 不能为空");

        let api_endpoint = HireApiV1::EmployeeGet(self.employee_id);
        let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "通过员工 ID 获取入职信息响应数据为空",
        )
        .await
    }
}

/// 通过员工 ID 获取入职信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `employee` 字段。
    pub employee: Option<EmployeeSummary>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for GetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/hire/v1/employees/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/employees/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "employee": {
                        "id": "7095600054216542508",
                        "application_id": "7073372582620416300",
                        "onboard_status": 1,
                        "conversion_status": 1,
                        "onboard_time": 1637596800000i64,
                        "expected_conversion_time": 1637596800000i64,
                        "actual_conversion_time": 1637596800000i64,
                        "overboard_time": 1637596800000i64,
                        "overboard_note": "职业发展考虑",
                        "onboard_city_code": "CT_2",
                        "department": "6966123381141866028",
                        "leader": "ou-xxx",
                        "sequence": "6937934036379650311",
                        "level": "7006234385490345986",
                        "employee_type": "1",
                        "job_requirement_id": "123123123213"
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetRequest::new(config)
            .employee_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let employee = resp.employee.as_ref().expect("employee 应存在");
        assert_eq!(employee.id.as_deref(), Some("7095600054216542508"));
        assert_eq!(employee.conversion_status, Some(1));
        assert_eq!(employee.onboard_time, Some(1637596800000));
        assert_eq!(employee.overboard_note.as_deref(), Some("职业发展考虑"));
        assert_eq!(employee.leader.as_deref(), Some("ou-xxx"));
        assert_eq!(employee.job_requirement_id.as_deref(), Some("123123123213"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
