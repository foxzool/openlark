//! 搜索成本中心信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/cost_center/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::CostCenterVersion;

/// SearchRequest
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    /// 请求体（可选）
    body: Option<Value>,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
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
        let mut request =
            ApiRequest::<SearchResponse>::post("/open-apis/corehr/v2/cost_centers/search");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// SearchResponse
///
/// 响应 `data` 子树形状：`{items, page_token, has_more}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 成本中心信息列表（元素为官方 `cost_center.version` 结构）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CostCenterVersion>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
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

    /// 端到端：POST /open-apis/corehr/v2/cost_centers/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/cost_centers/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "cost_center_id": "6969828847121885087",
                            "version_id": "6969828847121885087",
                            "name": [
                                { "lang": "zh-CN", "value": "技术部成本中心" }
                            ],
                            "code": "MDPD00000023",
                            "parent_cost_center_id": "6862995757234914824",
                            "managers": ["6862995757234914824"],
                            "description": [
                                { "lang": "zh-CN", "value": "基础架构相关业务" }
                            ],
                            "effective_time": "2020-01-01",
                            "expiration_time": "2020-01-01",
                            "active": true
                        }
                    ],
                    "page_token": "6969828847121885087",
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
            .expect("items 应包含成本中心版本");
        assert_eq!(item.cost_center_id.as_deref(), Some("6969828847121885087"));
        assert_eq!(item.version_id.as_deref(), Some("6969828847121885087"));
        assert_eq!(
            item.parent_cost_center_id.as_deref(),
            Some("6862995757234914824")
        );
        assert_eq!(
            item.managers.as_deref(),
            Some(&["6862995757234914824".to_string()][..])
        );
        assert_eq!(resp.has_more, Some(true));
        assert_eq!(resp.page_token.as_deref(), Some("6969828847121885087"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
