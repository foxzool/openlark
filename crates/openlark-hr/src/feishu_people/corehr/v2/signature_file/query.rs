//! 查询电子签文件详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/signature_file/query>

use super::models::SignatureFile;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询电子签文件详情请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        let mut request =
            ApiRequest::<QueryResponse>::post("/open-apis/corehr/v2/signature_files/query");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 查询电子签文件详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 电子签文件信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SignatureFile>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for QueryResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/signature_files/query
    /// （官方 data 内层键为 items / page_token / has_more）
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/signature_files/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "signature_file_id": "7147527056140813828",
                            "names": [
                                { "lang": "zh-CN", "value": "中文示例" }
                            ],
                            "create_time": "2023-01-01 00:00:00",
                            "update_time": "2023-01-01 00:00:00",
                            "employment_id": "7123132668099919891",
                            "pre_hire_id": "7123132668099919891",
                            "signature_file_state": {
                                "enum_name": "sign_finished",
                                "display": [ { "lang": "zh-CN", "value": "签署已完成" } ]
                            },
                            "contract_code": "48793beea04f4e2583d50732e481d507",
                            "effective_date": "2024-12-01",
                            "template_id": "7147527056140813828",
                            "sign_url": "123213"
                        }
                    ],
                    "page_token": "{\"cn\":\"[\\\"7371387495822411308\\\"]\"}",
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

        let resp = QueryRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应解码");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].signature_file_id.as_deref(),
            Some("7147527056140813828")
        );
        assert_eq!(items[0].pre_hire_id.as_deref(), Some("7123132668099919891"));
        assert_eq!(
            items[0]
                .signature_file_state
                .as_ref()
                .and_then(|s| s.enum_name.clone()),
            Some("sign_finished".to_string())
        );
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
