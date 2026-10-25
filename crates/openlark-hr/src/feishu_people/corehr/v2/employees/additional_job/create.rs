//! 创建兼职
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employees.additional_job/create>

use super::models::AdditionalJob;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `CreateRequest` 请求。
#[derive(Debug, Clone)]
pub struct CreateRequest {
    config: Config,
    request_body: Option<Value>,
}

impl CreateRequest {
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
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let api_endpoint = FeishuPeopleApiV2::EmployeesAdditionalJobCreate;
        let mut request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(request, &self.config, Some(option), "创建兼职响应数据为空").await
    }
}

/// `CreateResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 兼职信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_job: Option<AdditionalJob>,
}

impl ApiResponseTrait for CreateResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/employees/additional_jobs
    /// （官方 data 内层键为 additional_job，含 work_location_id / work_shift 等）
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/employees/additional_jobs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "additional_job": {
                        "id": "6890452208593372679",
                        "employee_type_id": "6890452208593372679",
                        "working_hours_type_id": "6890452208593372679",
                        "work_location_id": "6890452208593372679",
                        "department_id": "6890452208593372679",
                        "job_id": "6890452208593372679",
                        "job_level_id": "6890452208593372679",
                        "job_family_id": "1245678",
                        "employment_id": "6893014062142064135",
                        "start_date": "2020-05-01",
                        "end_date": "2020-05-02",
                        "direct_manager_id": "6890452208593372680",
                        "dotted_line_manager_id": "6890452208593372680",
                        "work_shift": {
                            "enum_name": "phone_type",
                            "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                        },
                        "compensation_type": {
                            "enum_name": "phone_type",
                            "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                        },
                        "service_company": "6890452208593372680",
                        "weekly_working_hours": "5",
                        "work_calendar_id": "6890452208593372680",
                        "position_id": "6890452208593372680",
                        "employee_subtype_id": "6890452208593372680"
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

        let resp = CreateRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let job = resp.additional_job.expect("additional_job 应解码");
        assert_eq!(job.id.as_deref(), Some("6890452208593372679"));
        assert_eq!(job.work_location_id.as_deref(), Some("6890452208593372679"));
        assert_eq!(
            job.dotted_line_manager_id.as_deref(),
            Some("6890452208593372680")
        );
        assert_eq!(
            job.work_shift.as_ref().and_then(|v| v.enum_name.clone()),
            Some("phone_type".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
