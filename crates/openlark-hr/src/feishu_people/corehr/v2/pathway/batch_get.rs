//! 获取通道信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/pathway/batch_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取通道信息请求
#[derive(Debug, Clone)]
pub struct BatchGetRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl BatchGetRequest {
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
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        let mut request =
            ApiRequest::<BatchGetResponse>::post("/open-apis/corehr/v2/pathways/batch_get");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 获取通道信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 查询的通道信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Pathway>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 通道信息（pathway）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pathway {
    /// 通道 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nText>>,
    /// 描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nText>>,
    /// 停启用状态；true：启用，false：停用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（中文用 zh-CN，英文用 en-US）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for BatchGetResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/pathways/batch_get
    #[tokio::test]
    async fn test_batch_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/pathways/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "pathway_id": "4692446793125560154",
                            "code": "A01234",
                            "names": [ { "lang": "zh-CN", "value": "中文示例" } ],
                            "descriptions": [ { "lang": "zh-CN", "value": "中文示例" } ],
                            "active": true
                        }
                    ],
                    "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
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

        let resp = BatchGetRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].pathway_id, Some("4692446793125560154".to_string()));
        assert_eq!(items[0].code, Some("A01234".to_string()));
        let names = items[0].names.as_ref().expect("names 应存在");
        assert_eq!(names[0].value, Some("中文示例".to_string()));
        let descriptions = items[0].descriptions.as_ref().expect("descriptions 应存在");
        assert_eq!(descriptions[0].lang, Some("zh-CN".to_string()));
        assert_eq!(items[0].active, Some(true));
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
