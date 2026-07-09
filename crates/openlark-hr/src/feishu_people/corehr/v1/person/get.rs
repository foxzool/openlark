//! 查询单个个人信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/person/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询单个个人信息请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    config: Config,
    person_id: String,
    user_id_type: Option<String>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            person_id: String::new(),
            user_id_type: None,
        }
    }

    /// 设置 `person_id`。
    pub fn person_id(mut self, person_id: String) -> Self {
        self.person_id = person_id;
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

        validate_required!(self.person_id.trim(), "个人信息ID不能为空");

        let api_endpoint = FeishuPeopleApiV1::PersonGet(self.person_id);
        let mut request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询单个个人信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询单个个人信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 原始响应数据。
    pub data: Value,
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

    /// 端到端：GET /open-apis/corehr/v1/persons/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/persons/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        GetRequest::new(config)
            .person_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
