//! 查询单个待入职信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/pre_hire/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::PreHire;

/// 查询单个待入职信息请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    config: Config,
    pre_hire_id: String,
    user_id_type: Option<String>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            pre_hire_id: String::new(),
            user_id_type: None,
        }
    }

    /// 设置 `pre_hire_id`。
    pub fn pre_hire_id(mut self, pre_hire_id: String) -> Self {
        self.pre_hire_id = pre_hire_id;
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
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
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        validate_required!(self.pre_hire_id.trim(), "待入职ID不能为空");

        let api_endpoint = FeishuPeopleApiV1::PreHireGet(self.pre_hire_id);
        let mut request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询单个待入职信息响应数据为空",
        )
        .await
    }
}

/// 查询单个待入职信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 待入职信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire: Option<PreHire>,
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

    /// 端到端：GET /open-apis/corehr/v1/pre_hires/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/pre_hires/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "pre_hire": {
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
                        "custom_fields": [
                            { "field_name": "name", "value": "\"Sandy\"" }
                        ],
                        "cost_center_rate": [
                            { "cost_center_id": "6950635856373745165", "rate": 100 }
                        ],
                        "onboarding_status": {
                            "enum_name": "preboarding",
                            "display": [{ "lang": "zh-CN", "value": "待入职" }]
                        }
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

        let data = GetRequest::new(config)
            .pre_hire_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let pre_hire = data.pre_hire.expect("pre_hire 应存在");
        assert_eq!(
            pre_hire.ats_application_id,
            Some("4719168654814483759".to_string())
        );
        assert_eq!(pre_hire.id, Some("154545454".to_string()));
        assert_eq!(pre_hire.worker_id, Some("1245646".to_string()));
        assert_eq!(pre_hire.person_id, Some("656464648662".to_string()));
        assert_eq!(pre_hire.hire_date, Some("2020-01-01".to_string()));
        assert_eq!(pre_hire.employee_type_id, Some("正式".to_string()));
        assert_eq!(
            pre_hire
                .onboarding_status
                .as_ref()
                .and_then(|s| s.enum_name.as_deref()),
            Some("preboarding")
        );
        assert_eq!(
            pre_hire
                .cost_center_rate
                .as_ref()
                .and_then(|c| c.first())
                .and_then(|c| c.rate),
            Some(100)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
