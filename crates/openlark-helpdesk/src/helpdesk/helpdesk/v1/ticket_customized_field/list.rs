//! 获取工单自定义字段列表
//!
//! 获取工单自定义字段列表。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket_customized_field/list-ticket-customized-fields>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;

use super::models::TicketCustomizedField;

/// 获取工单自定义字段列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTicketCustomizedFieldResponse {
    /// 是否还有更多项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一分页标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// 工单自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TicketCustomizedField>>,
}

impl ApiResponseTrait for ListTicketCustomizedFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取工单自定义字段列表请求体（官方 GET 接口携带 JSON 请求体）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTicketCustomizedFieldRequestBody {
    /// 是否可见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

/// 获取工单自定义字段列表请求
#[derive(Debug, Clone)]
pub struct ListTicketCustomizedFieldRequest {
    config: Arc<Config>,
    /// 分页标记（查询参数）
    page_token: Option<String>,
    /// 分页大小（查询参数，默认 20，最大值 100）
    page_size: Option<i32>,
    /// 是否可见（请求体）
    visible: Option<bool>,
}

impl ListTicketCustomizedFieldRequest {
    /// 创建新的获取工单自定义字段列表请求
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_token: None,
            page_size: None,
            visible: None,
        }
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置分页大小（默认 20，最大值 100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置是否可见（官方 GET 接口的 JSON 请求体字段）
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    /// 执行获取工单自定义字段列表请求
    pub async fn execute(self) -> SDKResult<ListTicketCustomizedFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTicketCustomizedFieldResponse> {
        let api_endpoint = HelpdeskApiV1::TicketCustomizedFieldList;
        let mut request =
            ApiRequest::<ListTicketCustomizedFieldResponse>::get(api_endpoint.to_url())
                .query_opt("page_token", self.page_token.as_ref())
                .query_opt("page_size", self.page_size.map(|v| v.to_string()));

        if self.visible.is_some() {
            request = request.body(serde_json::to_value(
                ListTicketCustomizedFieldRequestBody {
                    visible: self.visible,
                },
            )?);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取工单自定义字段列表",
        )
        .await
    }
}

/// 获取工单自定义字段列表请求构建器
#[derive(Debug, Clone)]
pub struct ListTicketCustomizedFieldRequestBuilder {
    config: Arc<Config>,
    page_token: Option<String>,
    page_size: Option<i32>,
    visible: Option<bool>,
}

impl ListTicketCustomizedFieldRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_token: None,
            page_size: None,
            visible: None,
        }
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置分页大小（默认 20，最大值 100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置是否可见（官方 GET 接口的 JSON 请求体字段）
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<ListTicketCustomizedFieldResponse> {
        ListTicketCustomizedFieldRequest {
            config: self.config.clone(),
            page_token: self.page_token.clone(),
            page_size: self.page_size,
            visible: self.visible,
        }
        .execute()
        .await
    }
}

/// 执行获取工单自定义字段列表（无分页参数的默认语义）
pub async fn list_ticket_customized_fields(
    config: &Config,
) -> SDKResult<ListTicketCustomizedFieldResponse> {
    ListTicketCustomizedFieldRequest::new(Arc::new(config.clone()))
        .execute()
        .await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let _builder = ListTicketCustomizedFieldRequestBuilder::new(Arc::new(config));
    }

    /// 端到端：GET .../ticket_customized_fields + 分页查询参数与 visible 请求体 →
    /// 强类型 ListTicketCustomizedFieldResponse 解析（官方字段名）。
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/ticket_customized_fields"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "has_more": true,
                    "next_page_token": "6948728206392295444",
                    "items": [
                        {
                            "ticket_customized_field_id": "tcf_001",
                            "helpdesk_id": "hd_001",
                            "key_name": "ticket_no",
                            "display_name": "工单编号",
                            "field_type": "text",
                            "required": true
                        }
                    ]
                }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = ListTicketCustomizedFieldRequest::new(config)
            .page_token("6948728206392295444")
            .page_size(50)
            .visible(true)
            .execute()
            .await
            .expect("获取工单自定义字段列表应成功");
        assert_eq!(resp.has_more, Some(true));
        assert_eq!(resp.next_page_token.as_deref(), Some("6948728206392295444"));
        let items = resp.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].ticket_customized_field_id.as_deref(),
            Some("tcf_001")
        );
        assert_eq!(items[0].key_name.as_deref(), Some("ticket_no"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/ticket_customized_fields"
        );
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("page_token=6948728206392295444"));
        assert!(query.contains("page_size=50"));
        let body = received[0].body.as_slice();
        let body: serde_json::Value = serde_json::from_slice(body).expect("请求体应为 JSON");
        assert_eq!(body["visible"], json!(true));
    }
}
