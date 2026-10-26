//! 创建通道
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/pathway/create>

use crate::common::shared_models::I18nLangValue;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 创建通道请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBody {
    /// 通道名称（必填，i18n 多语，最多 2 条；名称不能包含「/」「；」「;」「\」「'」字符）
    pub names: Vec<I18nLangValue>,
    /// 通道编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 通道描述（i18n 多语，最多 2 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nLangValue>>,
}

/// 创建通道请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: CreateBody,
    /// 用于幂等控制的 client_token
    client_token: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: CreateBody::default(),
            client_token: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: CreateBody) -> Self {
        self.body = body;
        self
    }

    /// 设置通道名称（必填，i18n 多语）。
    pub fn names(mut self, names: Vec<I18nLangValue>) -> Self {
        self.body.names = names;
        self
    }

    /// 设置通道编码。
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.body.code = Some(code.into());
        self
    }

    /// 设置通道描述（i18n 多语）。
    pub fn descriptions(mut self, descriptions: Vec<I18nLangValue>) -> Self {
        self.body.descriptions = Some(descriptions);
        self
    }

    /// 设置用于幂等控制的 client_token。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
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
        validate_required!(self.body.names, "names 不能为空");

        let request = ApiRequest::<CreateResponse>::post("/open-apis/corehr/v2/pathways")
            .query_opt("client_token", self.client_token)
            .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 创建通道响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 通道ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
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
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/pathways
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/pathways"))
            .and(query_param("client_token", "12454646"))
            .and(body_json(json!({
                "code": "A01234",
                "names": [
                    { "lang": "zh-CN", "value": "中文示例" }
                ],
                "descriptions": [
                    { "lang": "zh-CN", "value": "中文示例" }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "pathway_id": "12345678" }
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
            .names(vec![I18nLangValue {
                lang: Some("zh-CN".to_string()),
                value: Some("中文示例".to_string()),
            }])
            .code("A01234")
            .descriptions(vec![I18nLangValue {
                lang: Some("zh-CN".to_string()),
                value: Some("中文示例".to_string()),
            }])
            .client_token("12454646")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.pathway_id, Some("12345678".to_string()));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
