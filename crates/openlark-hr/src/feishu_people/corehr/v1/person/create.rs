//! 创建个人信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/person/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::models::Person;

/// 创建个人信息请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    person: Option<Value>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            person: None,
        }
    }

    /// 设置 `person`。
    pub fn person(mut self, person: Value) -> Self {
        self.person = Some(person);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::PersonCreate;
        let mut request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        if let Some(person) = self.person {
            request = request.body(serde_json::json!({ "person": person }));
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

/// 创建个人信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 个人信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<Person>,
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

    /// 端到端：POST /open-apis/corehr/v1/persons
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/persons"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "person": {
                        "phone_number": "11111111111",
                        "legal_name": "张三",
                        "preferred_name": "张四",
                        "id": "646465654545",
                        "name_list": [{
                            "local_primary": "黄",
                            "local_first_name": "四",
                            "country_region_id": "6862995757234914824",
                            "name_type": {
                                "enum_name": "type_1",
                                "display": [{ "lang": "zh-CN", "value": "刘梓新" }]
                            }
                        }]
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

        let data = CreateRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let person = data.person.expect("person 应存在");
        assert_eq!(person.phone_number, Some("11111111111".to_string()));
        assert_eq!(person.legal_name, Some("张三".to_string()));
        assert_eq!(person.preferred_name, Some("张四".to_string()));
        assert_eq!(person.id, Some("646465654545".to_string()));
        let name_list = person.name_list.expect("name_list 应存在");
        assert_eq!(name_list.len(), 1);
        assert_eq!(name_list[0].local_first_name, Some("四".to_string()));
        assert_eq!(
            name_list[0].country_region_id,
            Some("6862995757234914824".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
