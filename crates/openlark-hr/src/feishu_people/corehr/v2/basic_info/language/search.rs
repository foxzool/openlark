//! 查询语言信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/basic_info.language/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询语言信息请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    request_body: Value,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: Value::Object(serde_json::Map::new()),
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        use crate::common::api_endpoints::CorehrApiV2;

        let api_endpoint = CorehrApiV2::BasicInfoLanguageSearch;
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");
        let request = ApiRequest::<SearchResponse>::post(endpoint_url).body(self.request_body);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询语言信息响应数据为空",
        )
        .await
    }
}

/// 查询语言信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 响应数据
    /// 数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<LanguageItem>>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 语言条目信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LanguageItem {
    /// 语言 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_id: Option<String>,
    /// 语言名称（多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    /// IETF 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ietf_language_tag: Option<String>,
    /// 状态：1 生效 / 0 失效
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for SearchResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/basic_info/languages/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/basic_info/languages/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "language_id": "6863323445740963342",
                            "name": [
                                { "lang": "zh-CN", "value": "英文（英国）" }
                            ],
                            "ietf_language_tag": "en-UK",
                            "status": 1
                        }
                    ],
                    "page_token": "6863323445740963342",
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

        let resp = SearchRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("应解析语言列表");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].language_id,
            Some("6863323445740963342".to_string())
        );
        assert_eq!(
            items[0].name.as_deref(),
            Some(
                &[I18nText {
                    lang: Some("zh-CN".to_string()),
                    value: Some("英文（英国）".to_string()),
                }][..]
            )
        );
        assert_eq!(items[0].ietf_language_tag, Some("en-UK".to_string()));
        assert_eq!(items[0].status, Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
