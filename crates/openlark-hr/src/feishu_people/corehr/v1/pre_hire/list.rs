//! 批量查询待入职信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/pre_hire/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::PreHire;

/// 批量查询待入职信息请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::PreHireList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询待入职信息响应数据为空",
        )
        .await
    }
}

/// 批量查询待入职信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 查询的待入职信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PreHire>>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记（`has_more` 为 `true` 时返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
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

    /// 端到端：GET /open-apis/corehr/v1/pre_hires
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/pre_hires"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "ats_application_id": "4719168654814483759",
                        "id": "154545454",
                        "hire_date": "2020-01-01",
                        "employee_type": {
                            "enum_name": "type_1",
                            "display": [{ "lang": "zh-CN", "value": "正式" }]
                        },
                        "worker_id": "1245646",
                        "employee_type_id": "正式",
                        "person_id": "656464648662",
                        "onboarding_status": {
                            "enum_name": "preboarding",
                            "display": [{ "lang": "zh-CN", "value": "待入职" }]
                        },
                        "cost_center_rate": [{ "cost_center_id": "6890452208593356295", "rate": 100 }],
                        "work_email_list": [{
                            "email": "test@example.com",
                            "is_primary": true,
                            "is_public": false
                        }],
                        "department_id": "od-xxx"
                    }],
                    "has_more": false
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

        let data = ListRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let items = data.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].ats_application_id,
            Some("4719168654814483759".to_string())
        );
        assert_eq!(items[0].worker_id, Some("1245646".to_string()));
        assert_eq!(items[0].person_id, Some("656464648662".to_string()));
        assert_eq!(items[0].hire_date, Some("2020-01-01".to_string()));
        assert_eq!(data.has_more, Some(false));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
