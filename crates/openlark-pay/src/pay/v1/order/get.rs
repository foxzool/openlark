//! 查询订单详情
//!
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uITNwUjLyUDM14iM1ATN>
//!
//! Authorization 仅支持 `tenant_access_token`。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption, validate_required,
};

use crate::common::api_endpoints::PayApiV1;
use crate::pay::v1::order::models::GetOrderResponse;

/// 查询订单详情请求。
#[derive(Debug, Clone)]
pub struct GetOrderRequest {
    config: Config,
    order_id: Option<String>,
}

impl GetOrderRequest {
    /// 创建请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            order_id: None,
        }
    }

    /// 设置订单 ID（必填）。
    pub fn order_id(mut self, order_id: impl Into<String>) -> Self {
        self.order_id = Some(order_id.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetOrderResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<GetOrderResponse> {
        let order_id = self.order_id.unwrap_or_default();
        validate_required!(order_id, "order_id 不能为空");

        let req: ApiRequest<GetOrderResponse> = ApiRequest::get(PayApiV1::OrderGet.to_url())
            .query("order_id", order_id)
            .with_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request_typed(req, &self.config, Some(option), "查询订单详情").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pay::v1::order::models::{OrderBuyType, OrderStatus, PricePlanType};
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
    async fn get_order_rejects_empty_order_id_before_sending_request() {
        let server = MockServer::start().await;
        let result = GetOrderRequest::new(test_config(server.uri()))
            .order_id("   ")
            .execute()
            .await;
        let error = result.expect_err("空 order_id 应在发起网络请求前被拒绝");
        assert!(error.to_string().contains("order_id"));
        assert_no_http(&server).await;
    }

    #[tokio::test]
    async fn get_order_rejects_missing_order_id_before_sending_request() {
        let server = MockServer::start().await;
        let result = GetOrderRequest::new(test_config(server.uri()))
            .execute()
            .await;
        let error = result.expect_err("缺 order_id 应在发起网络请求前被拒绝");
        assert!(error.to_string().contains("order_id"));
        assert_no_http(&server).await;
    }

    #[tokio::test]
    async fn get_order_gets_official_contract_and_parses_typed_data() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/pay/v1/order/get"))
            .and(query_param("order_id", "order_001"))
            .and(header("authorization", "Bearer tenant-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "order": {
                        "order_id": "order_001",
                        "price_plan_id": "plan_seat",
                        "price_plan_type": "per_seat_per_year",
                        "seats": 10,
                        "buy_count": 1,
                        "create_time": "1600000000",
                        "pay_time": "1600000100",
                        "status": "normal",
                        "buy_type": "buy",
                        "src_order_id": null,
                        "dst_order_id": null,
                        "order_pay_price": 19900,
                        "tenant_key": "tenant_abc"
                    }
                }
            })))
            .mount(&server)
            .await;

        let resp = GetOrderRequest::new(test_config(server.uri()))
            .order_id("order_001")
            .execute_with_options(tenant_option())
            .await
            .expect("查询订单详情应成功");
        let order = resp.order.expect("响应应包含 order");
        assert_eq!(order.order_id.as_deref(), Some("order_001"));
        assert_eq!(order.price_plan_type, Some(PricePlanType::PerSeatPerYear));
        assert_eq!(order.seats, Some(10));
        assert_eq!(order.buy_count, Some(1));
        assert_eq!(order.status, Some(OrderStatus::Normal));
        assert_eq!(order.buy_type, Some(OrderBuyType::Buy));
        assert_eq!(order.order_pay_price, Some(19900));
        assert_eq!(order.tenant_key.as_deref(), Some("tenant_abc"));
        assert_eq!(order.create_time.as_deref(), Some("1600000000"));
    }
}
