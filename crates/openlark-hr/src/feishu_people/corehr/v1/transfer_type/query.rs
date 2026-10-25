//! 获取异动类型列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/transfer_type/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取异动类型列表请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
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

        let api_endpoint = CorehrApiV1::TransferTypeQuery;
        let request = ApiRequest::<QueryResponse>::get(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(_option),
            "获取异动类型列表响应数据为空",
        )
        .await
    }
}

/// 获取异动类型列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 异动类型列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TransferTypeItem>>,
}

/// 异动类型
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TransferTypeItem {
    /// 异动类型唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_type_unique_identifier: Option<String>,
    /// 异动类型名称（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nItem>>,
    /// 异动类型状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 关联流程唯一标识符
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// 关联流程名称（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<Vec<I18nItem>>,
    /// 创建时间（官方响应示例中为 `created_at`）
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "created_at")]
    pub created_time: Option<String>,
    /// 更新时间（官方响应示例中为 `updated_at`）
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "updated_at")]
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

    /// 端到端：GET /open-apis/corehr/v1/transfer_types/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/transfer_types/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "active": true,
                            "created_at": "2022-04-08 15:17:10",
                            "flow_id": "people_6963913041981490725_6982474967023882020",
                            "flow_name": [ { "lang": "zh-CN", "value": "xmz异动流程" } ],
                            "name": [ { "lang": "zh-CN", "value": "上级变更" } ],
                            "transfer_type_unique_identifier": "reporting_line_change",
                            "updated_at": "2022-04-08 15:17:10"
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
            items[0].transfer_type_unique_identifier,
            Some("reporting_line_change".to_string())
        );
        assert_eq!(
            items[0].flow_id,
            Some("people_6963913041981490725_6982474967023882020".to_string())
        );
        assert_eq!(items[0].flow_name.as_deref().map(|f| f.len()), Some(1));
        // 官方示例使用 created_at/updated_at，经 alias 归一到 created_time/updated_time
        assert_eq!(
            items[0].created_time,
            Some("2022-04-08 15:17:10".to_string())
        );
        assert_eq!(
            items[0].updated_time,
            Some("2022-04-08 15:17:10".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
