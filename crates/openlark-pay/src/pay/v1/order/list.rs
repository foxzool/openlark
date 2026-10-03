//! 查询租户购买的付费方案
//!
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uETNwUjLxUDM14SM1ATN>
//!
//! Authorization 仅支持 `tenant_access_token`。`page_size` 按文档必填，不设默认值。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, error::CoreError,
    http::Transport, req_option::RequestOption,
};

use crate::common::api_endpoints::PayApiV1;
use crate::pay::v1::order::models::{ListOrdersResponse, OrderListStatus};

/// 查询租户购买的付费方案请求。
#[derive(Debug, Clone)]
pub struct ListOrdersRequest {
    config: Config,
    page_size: Option<i32>,
    status: Option<OrderListStatus>,
    page_token: Option<String>,
    tenant_key: Option<String>,
}

impl ListOrdersRequest {
    /// 创建请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            status: None,
            page_token: None,
            tenant_key: None,
        }
    }

    /// 设置分页大小（必填，无默认值）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置订单状态过滤：`normal` / `refunded` / `all`。
    pub fn status(mut self, status: OrderListStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置租户 key。
    pub fn tenant_key(mut self, tenant_key: impl Into<String>) -> Self {
        self.tenant_key = Some(tenant_key.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListOrdersResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListOrdersResponse> {
        let page_size = self
            .page_size
            .ok_or_else(|| CoreError::validation_msg("page_size 不能为空"))?;

        let mut req: ApiRequest<ListOrdersResponse> = ApiRequest::get(PayApiV1::OrderList.to_url())
            .query("page_size", page_size.to_string())
            .with_supported_access_token_types(vec![AccessTokenType::Tenant]);
        if let Some(status) = self.status {
            req = req.query("status", status.as_str());
        }
        req = req.query_opt("page_token", self.page_token);
        req = req.query_opt("tenant_key", self.tenant_key);

        Transport::request_typed(req, &self.config, Some(option), "查询租户购买的付费方案").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pay::v1::order::models::OrderStatus;
    use openlark_core::config::Config;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path, query_param},
    };

    fn test_config(base_url: impl Into<String>) -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(base_url)
            .enable_token_cache(false)
            .build()
    }

    fn tenant_option() -> RequestOption {
        RequestOption::builder()
            .tenant_access_token("tenant-token")
            .build()
    }

    async fn assert_no_http(server: &MockServer) {
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn list_orders_rejects_missing_page_size_before_sending_request() {
        let server = MockServer::start().await;
        let result = ListOrdersRequest::new(test_config(server.uri()))
            .execute()
            .await;
        let error = result.expect_err("缺 page_size 应在发起网络请求前被拒绝");
        assert!(error.to_string().contains("page_size"));
        assert_no_http(&server).await;
    }

    #[tokio::test]
    async fn list_orders_gets_official_contract_and_parses_typed_data() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/pay/v1/order/list"))
            .and(query_param("page_size", "10"))
            .and(query_param("status", "refunded"))
            .and(header("authorization", "Bearer tenant-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "total": 1,
                    "has_more": false,
                    "page_token": "",
                    "order_list": [{
                        "order_id": "order_002",
                        "status": "refund",
                        "buy_count": 1
                    }]
                }
            })))
            .mount(&server)
            .await;

        let resp = ListOrdersRequest::new(test_config(server.uri()))
            .page_size(10)
            .status(OrderListStatus::Refunded)
            .execute_with_options(tenant_option())
            .await
            .expect("查询订单列表应成功");
        assert_eq!(resp.total, Some(1));
        assert_eq!(resp.has_more, Some(false));
        assert_eq!(resp.page_token.as_deref(), Some(""));
        let orders = resp.order_list.expect("响应应包含 order_list");
        assert_eq!(orders[0].status, Some(OrderStatus::Refund));
    }
}
