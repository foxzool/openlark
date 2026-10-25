//! 更新兼职
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employees.additional_job/patch>

use super::models::AdditionalJob;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `PatchRequest` 请求。
#[derive(Debug, Clone)]
pub struct PatchRequest {
    config: Config,
    additional_job_id: Option<String>,
    request_body: Option<Value>,
}

impl PatchRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            additional_job_id: None,
            request_body: None,
        }
    }

    /// 设置 `additional_job_id`。
    pub fn additional_job_id(mut self, additional_job_id: String) -> Self {
        self.additional_job_id = Some(additional_job_id);
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let additional_job_id = self.additional_job_id.unwrap_or_default();
        validate_required!(additional_job_id.trim(), "additional_job_id 不能为空");
        let api_endpoint = FeishuPeopleApiV2::EmployeesAdditionalJobPatch(additional_job_id);
        let mut request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(request, &self.config, Some(option), "更新兼职响应数据为空").await
    }
}

/// `PatchResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 兼职信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_job: Option<AdditionalJob>,
}

impl ApiResponseTrait for PatchResponse {
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

    /// 端到端：PATCH /open-apis/corehr/v2/employees/additional_jobs/test001
    /// （官方 data 内层键为 additional_job）
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/corehr/v2/employees/additional_jobs/test001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "additional_job": {
                        "id": "6890452208593372679",
                        "employee_type_id": "6890452208593372679",
                        "work_location_id": "6890452208593372679",
                        "department_id": "6890452208593372679",
                        "employment_id": "6893014062142064135",
                        "start_date": "2020-05-01",
                        "dotted_line_manager_id": "6890452208593372680",
                        "work_shift": {
                            "enum_name": "phone_type",
                            "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                        },
                        "service_company": "6890452208593372680",
                        "weekly_working_hours": "5"
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

        let resp = PatchRequest::new(config)
            .additional_job_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let job = resp.additional_job.expect("additional_job 应解码");
        assert_eq!(job.id.as_deref(), Some("6890452208593372679"));
        assert_eq!(job.work_location_id.as_deref(), Some("6890452208593372679"));
        assert_eq!(
            job.work_shift.as_ref().and_then(|v| v.enum_name.clone()),
            Some("phone_type".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
