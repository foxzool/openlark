//! 查询 Exchange 账户的绑定状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/exchange_binding/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use super::models::GetExchangeBindingResponse;
use crate::common::api_utils::extract_response_data;
/// 查询 Exchange 账户的绑定状态请求
pub struct GetExchangeBindingRequest {
    config: Config,
    exchange_binding_id: String,
}

impl GetExchangeBindingRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            exchange_binding_id: String::new(),
        }
    }

    /// Exchange 绑定 ID（路径参数）
    pub fn exchange_binding_id(mut self, exchange_binding_id: impl Into<String>) -> Self {
        self.exchange_binding_id = exchange_binding_id.into();
        self
    }

    /// 执行请求
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/exchange_binding/get>
    pub async fn execute(self) -> SDKResult<GetExchangeBindingResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用自定义请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetExchangeBindingResponse> {
        validate_required!(self.exchange_binding_id, "exchange_binding_id 不能为空");

        // url: GET:/open-apis/calendar/v4/exchange_bindings/:exchange_binding_id
        let req: ApiRequest<GetExchangeBindingResponse> = ApiRequest::get(format!(
            "/open-apis/calendar/v4/exchange_bindings/{}",
            self.exchange_binding_id
        ));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询 Exchange 账户的绑定状态")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../calendar/v4/exchange_bindings/{id} → GetExchangeBindingResponse（data.exchange_binding_id）。
    #[tokio::test]
    async fn test_get_exchange_binding_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/calendar/v4/exchange_bindings/binding_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "exchange_binding_id": "binding_001",
                    "user_id": "user_001",
                    "exchange_account": "user@example.com",
                    "status": 1
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

        let resp = GetExchangeBindingRequest::new(config)
            .exchange_binding_id("binding_001")
            .execute()
            .await
            .expect("查询 Exchange 绑定应成功");
        assert_eq!(resp.exchange_binding_id, "binding_001");
        assert_eq!(resp.user_id.as_deref(), Some("user_001"));
        assert_eq!(resp.status, Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/exchange_bindings/binding_001"
        );
        assert_eq!(received[0].method, "GET");
    }
}
