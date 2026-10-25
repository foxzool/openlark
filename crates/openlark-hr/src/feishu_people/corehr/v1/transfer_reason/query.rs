//! 获取异动原因列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/transfer_reason/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取异动原因列表请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    config: Config,
}

impl QueryRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::CorehrApiV1;

        let api_endpoint = CorehrApiV1::TransferReasonQuery;
        let request = ApiRequest::<QueryResponse>::get(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(_option),
            "获取异动原因列表响应为空",
        )
        .await
    }
}

/// 获取异动原因列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 异动原因列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TransferReasonItem>>,
}

/// 异动原因
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TransferReasonItem {
    /// 异动原因唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_reason_unique_identifier: Option<String>,
    /// 异动原因的名称信息（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nItem>>,
    /// 异动原因状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 上级异动原因唯一标识（顶级原因可能为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_transfer_reason_unique_identifier: Option<String>,
    /// 创建时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 更新时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

/// 官方 i18n 多语结构：`{lang, value}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct I18nItem {
    /// 语言编码，如 `zh-CN`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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

    /// 端到端：GET /open-apis/corehr/v1/transfer_reasons/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/transfer_reasons/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "transfer_reason_unique_identifier": "voluntary_transfer",
                            "name": [ { "lang": "zh-CN", "value": "张三" } ],
                            "active": true,
                            "parent_transfer_reason_unique_identifier": "reason_for_job_change_option25",
                            "created_time": "2021-07-04 20:30:47",
                            "updated_time": "2022-02-28 16:27:56"
                        }
                    ]
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

        let items = resp.items.expect("items 不能为空");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].transfer_reason_unique_identifier,
            Some("voluntary_transfer".to_string())
        );
        assert_eq!(
            items[0].parent_transfer_reason_unique_identifier,
            Some("reason_for_job_change_option25".to_string())
        );
        assert_eq!(items[0].active, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
