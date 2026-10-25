//! 查询背调信息列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/background_check_order/batch_query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::BackgroundCheckOrderSummary;

/// 查询背调信息列表请求
#[derive(Debug, Clone)]
pub struct BatchQueryRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl BatchQueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchQueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchQueryResponse> {
        use crate::common::api_endpoints::HireApiV1;

        let api_endpoint = HireApiV1::BackgroundCheckOrderBatchQuery;
        let mut request = ApiRequest::<BatchQueryResponse>::post(api_endpoint.to_url());
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询背调信息列表响应数据为空",
        )
        .await
    }
}

/// 查询背调信息列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchQueryResponse {
    #[serde(default, alias = "orders")]
    /// 结果项列表。
    pub items: Vec<BackgroundCheckOrderSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for BatchQueryResponse {
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

    /// 端到端：POST /open-apis/hire/v1/background_check_orders/batch_query
    #[tokio::test]
    async fn test_batch_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/hire/v1/background_check_orders/batch_query",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "order_id": "7037986982531778860",
                            "application_id": "6985833807195212076",
                            "order_status": 2,
                            "account_third_type": 1,
                            "package": "基础版",
                            "feedback_info_list": [
                                {
                                    "id": "6930815272790114324",
                                    "attachment_url": "https://feishucdn.com/staource/v1/2de04c10",
                                    "report_preview_url": "https://feishucdn.com/preview/file/6930815272790114324/",
                                    "result": "红灯",
                                    "report_type": 1,
                                    "create_time": "1686645425868",
                                    "report_name": "张三的背调报告"
                                }
                            ]
                        }
                    ],
                    "has_more": false
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

        let resp = BatchQueryRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");
        let item = resp.items.first().expect("应返回 items 列表");
        assert_eq!(item.order_id.as_deref(), Some("7037986982531778860"));
        // 官方响应体含 order_status / account_third_type / feedback_info_list（.md 字段表）
        assert_eq!(item.order_status, Some(2));
        assert_eq!(item.account_third_type, Some(1));
        let feedback = item
            .feedback_info_list
            .as_ref()
            .and_then(|l| l.first())
            .expect("feedback_info_list 应反序列化");
        assert_eq!(feedback.id.as_deref(), Some("6930815272790114324"));
        assert_eq!(feedback.result.as_deref(), Some("红灯"));
        assert_eq!(feedback.report_type, Some(1));
        assert_eq!(
            feedback.report_preview_url.as_deref(),
            Some("https://feishucdn.com/preview/file/6930815272790114324/")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
