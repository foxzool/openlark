//! 创建个人信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/person/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::PersonInfo;

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

        let api_endpoint = FeishuPeopleApiV2::PersonCreate;
        let mut request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建个人信息响应数据为空",
        )
        .await
    }
}

/// `CreateResponse` 响应。
///
/// 响应 `data` 子树形状：`{person: {...}}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 个人信息（官方 `person_info` 结构）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonInfo>,
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

    /// 端到端：POST /open-apis/corehr/v2/persons
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/persons"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "person": {
                        "person_id": "6919733936050406926",
                        "phone_number": "13649211111",
                        "legal_name": "张三",
                        "preferred_name": "刘梓新(Henry)",
                        "preferred_local_full_name": "刘梓新",
                        "preferred_english_full_name": "Henry",
                        "name_list": [
                            {
                                "local_primary": "黄",
                                "local_first_name": "四",
                                "country_region_id": "6862995757234914824",
                                "name_type": {
                                    "enum_name": "legal_name",
                                    "display": [
                                        { "lang": "zh-CN", "value": "中文示例" }
                                    ]
                                },
                                "first_name": "帅",
                                "full_name": "王大帅"
                            }
                        ]
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

        let person = resp.person.as_ref().expect("person 应存在");
        assert_eq!(person.person_id.as_deref(), Some("6919733936050406926"));
        assert_eq!(person.legal_name.as_deref(), Some("张三"));
        assert_eq!(person.preferred_name.as_deref(), Some("刘梓新(Henry)"));
        assert_eq!(person.preferred_local_full_name.as_deref(), Some("刘梓新"));
        assert_eq!(person.preferred_english_full_name.as_deref(), Some("Henry"));
        let name = person
            .name_list
            .as_ref()
            .and_then(|list| list.first())
            .expect("name_list 应包含姓名");
        assert_eq!(name.local_primary.as_deref(), Some("黄"));
        assert_eq!(name.local_first_name.as_deref(), Some("四"));
        assert_eq!(
            name.country_region_id.as_deref(),
            Some("6862995757234914824")
        );
        assert_eq!(
            name.name_type.as_ref().and_then(|t| t.enum_name.as_deref()),
            Some("legal_name")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
