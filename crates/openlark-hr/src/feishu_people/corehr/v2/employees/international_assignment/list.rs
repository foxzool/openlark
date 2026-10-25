//! 批量查询外派信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employees.international_assignment/list>

use super::create::EmployeesInternationalAssignmentResp;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `ListRequest` 请求。
#[derive(Debug, Clone)]
pub struct ListRequest {
    config: Config,
    request_body: Option<Value>,
}

impl ListRequest {
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
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let api_endpoint = FeishuPeopleApiV2::EmployeesInternationalAssignmentList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询外派信息响应数据为空",
        )
        .await
    }
}

/// `ListResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 外派信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<EmployeesInternationalAssignmentResp>>,
    /// 无权限的雇佣 ID（仅在指定 employment_ids 查询时生效）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_authority_ids: Option<Vec<String>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListResponse {
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

    /// 端到端：GET /open-apis/corehr/v2/employees/international_assignments
    /// （官方 data 内层键为 items / no_authority_ids / page_token / has_more）
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/corehr/v2/employees/international_assignments",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "work_location_id": "7127921432117937708",
                            "service_company": "7127921432117937708",
                            "work_shift": {
                                "enum_name": "phone_type",
                                "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                            },
                            "working_hours_type_id": "7127921432117937708",
                            "employee_type_id": "7127921432117937708",
                            "weekly_working_hours_v2": 8,
                            "department_id": "7127921432117937708",
                            "job_id": "7127921432117937708",
                            "job_family_id": "7127921432117937708",
                            "job_level_id": "7127921432117937708",
                            "job_grade_id": "7127921432117937708",
                            "compensation_type": {
                                "enum_name": "phone_type",
                                "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                            },
                            "direct_manager_id": "7127921432117937708",
                            "dotted_line_manager_id": "7127921432117937708",
                            "work_calendar_id": "7127921432117937708",
                            "position_id": "7127921432117937708",
                            "employment_id": "7127921432117937708",
                            "custom_fields": [
                                { "field_name": "name", "value": "Sandy" }
                            ],
                            "international_assignment_reason": "xxx 项目派遣",
                            "description": "xxx 项目",
                            "international_assignment_expected_end_date": "2024-01-02",
                            "international_assignment_status": {
                                "enum_name": "phone_type",
                                "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                            },
                            "international_assignment_type": {
                                "enum_name": "phone_type",
                                "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                            },
                            "effective_time": "2024-01-02",
                            "expiration_time": "2024-01-02",
                            "id": "7127921432117937708"
                        }
                    ],
                    "no_authority_ids": [
                        "7127921432117937708"
                    ],
                    "page_token": "123",
                    "has_more": true
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

        let resp = ListRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应解码");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].work_location_id.as_deref(),
            Some("7127921432117937708")
        );
        assert_eq!(
            items[0].service_company.as_deref(),
            Some("7127921432117937708")
        );
        assert_eq!(items[0].weekly_working_hours_v2, Some(8.0));
        assert_eq!(
            items[0]
                .work_shift
                .as_ref()
                .and_then(|v| v.enum_name.clone()),
            Some("phone_type".to_string())
        );
        assert_eq!(
            resp.no_authority_ids,
            Some(vec!["7127921432117937708".to_string()])
        );
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
