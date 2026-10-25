//! 查询货币信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/basic_info.currency/search>

use crate::feishu_people::corehr::v2::basic_info::I18nText;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询货币信息请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    /// 请求体
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

    /// 设置请求体
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

        let api_endpoint = CorehrApiV2::BasicInfoCurrencySearch;
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");

        let request = ApiRequest::<SearchResponse>::post(endpoint_url).body(self.request_body);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询货币信息响应数据为空",
        )
        .await
    }
}

/// 查询货币信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 响应数据
    /// 数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CurrencyItem>>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 货币条目信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CurrencyItem {
    /// 货币 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// 货币所属国家/地区 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_region_id_list: Option<Vec<String>>,
    /// 货币名称（i18n 多语数组）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<Vec<I18nText>>,
    /// 数字代码（ISO 4217）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_code: Option<i32>,
    /// 三位字母代码（ISO 4217）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_alpha_3_code: Option<String>,
    /// 状态（1 生效 / 0 失效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
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

    /// 端到端：POST /open-apis/corehr/v2/basic_info/currencies/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/basic_info/currencies/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "currency_id": "6863329932261459464",
                            "country_region_id_list": ["6862995757234914824"],
                            "currency_name": [{ "lang": "zh-CN", "value": "中文示例" }],
                            "numeric_code": 156,
                            "currency_alpha_3_code": "CNY",
                            "status": 1
                        }
                    ],
                    "page_token": "6863329958782043655",
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

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("应返回货币条目");
        assert_eq!(item.currency_id.as_deref(), Some("6863329932261459464"));
        assert_eq!(
            item.country_region_id_list.as_deref(),
            Some(&["6862995757234914824".to_string()][..])
        );
        assert_eq!(item.numeric_code, Some(156));
        assert_eq!(item.currency_alpha_3_code.as_deref(), Some("CNY"));
        assert_eq!(
            item.currency_name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("中文示例")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
