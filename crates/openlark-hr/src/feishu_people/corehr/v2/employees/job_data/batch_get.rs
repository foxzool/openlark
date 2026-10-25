//! 批量查询员工任职信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employees.job_data/batch_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::models::EmployeeJobData;

/// `BatchGetRequest` 请求。
#[derive(Debug, Clone)]
pub struct BatchGetRequest {
    config: Config,
    request_body: Option<Value>,
}

impl BatchGetRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let api_endpoint = FeishuPeopleApiV2::EmployeesJobDataBatchGet;
        let mut request = ApiRequest::<BatchGetResponse>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询员工任职信息响应数据为空",
        )
        .await
    }
}

/// `BatchGetResponse` 响应。
///
/// 响应 `data` 子树形状：`{items: employee_job_data[]}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 查询的任职信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<EmployeeJobData>>,
}

impl ApiResponseTrait for BatchGetResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/employees/job_datas/batch_get
    #[tokio::test]
    async fn test_batch_get_returns_data_on_success() {
        let job_data = json!({
            "job_data_id": "6890452208593372698",
            "version_id": "6890452208593372697",
            "employee_type_id": "6971090097697521314",
            "working_hours_type_id": "6890452208593372680",
            "work_location_id": "6890452208593372679",
            "department_id": "6893014062142064135",
            "position_id": "6893014062142064135",
            "job_id": "6893014062142064135",
            "job_level_id": "6893014062142064135",
            "job_grade_id": "6893014062142064135",
            "job_family_id": "6893014062142064135",
            "probation_start_date": "2022-01-02",
            "probation_end_date": "2022-03-02",
            "primary_job_data": true,
            "employment_id": "6893014062142064135",
            "effective_time": "2020-01-01 00:00:00",
            "expiration_time": "2021-01-01 00:00:00",
            "assignment_start_reason": {
                "enum_name": "transfer",
                "display": [{ "lang": "zh-CN", "value": "转岗" }]
            },
            "probation_expected_end_date": "2022-03-02",
            "probation_outcome": {
                "enum_name": "positive",
                "display": [{ "lang": "zh-CN", "value": "转正" }]
            },
            "direct_manager": {
                "job_data_id": "6890452208593372698",
                "employment_id": "6893014062142064135"
            },
            "dotted_line_managers": [],
            "second_direct_manager": null,
            "cost_center_rates": [
                { "cost_center_id": "7142384817131652652", "rate": 100, "new_rate": 100.0 }
            ],
            "work_shift": {
                "enum_name": "day_shift",
                "display": [{ "lang": "zh-CN", "value": "白班" }]
            },
            "compensation_type": {
                "enum_name": "monthly",
                "display": [{ "lang": "zh-CN", "value": "月薪" }]
            },
            "service_company": "4692472714243080020",
            "created_at": "2020-05-02 00:00:00",
            "weekly_working_hours_v2": 40.0,
            "weekly_working_hours": 40,
            "employee_subtype_id": "6971090097697521317",
            "job_data_reason": {
                "enum_name": "transfer",
                "display": [{ "lang": "zh-CN", "value": "转岗" }]
            },
            "pathway_id": "6893014062142064135"
        });

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/employees/job_datas/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "employment_id": "6893014062142064135",
                            "job_datas": [job_data]
                        }
                    ]
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

        let resp = BatchGetRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含任职信息");
        assert_eq!(item.employment_id.as_deref(), Some("6893014062142064135"));
        let job_data = item
            .job_datas
            .as_ref()
            .and_then(|ds| ds.first())
            .expect("job_datas 应包含任职记录");
        assert_eq!(job_data.version_id.as_deref(), Some("6890452208593372697"));
        assert_eq!(
            job_data.work_location_id.as_deref(),
            Some("6890452208593372679")
        );
        assert_eq!(job_data.created_at.as_deref(), Some("2020-05-02 00:00:00"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
