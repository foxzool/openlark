//! 解除 Exchange 账户绑定
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/exchange_binding/delete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use super::models::DeleteExchangeBindingResponse;
use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_EXCHANGE_BINDINGS};

/// 解除 Exchange 账户绑定请求
pub struct DeleteExchangeBindingRequest {
    config: Config,
    exchange_binding_id: String,
}

impl DeleteExchangeBindingRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/exchange_binding/delete>
    pub async fn execute(self) -> SDKResult<DeleteExchangeBindingResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用自定义请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteExchangeBindingResponse> {
        validate_required!(self.exchange_binding_id, "exchange_binding_id 不能为空");

        // url: DELETE:/open-apis/calendar/v4/exchange_bindings/:exchange_binding_id
        let req: ApiRequest<DeleteExchangeBindingResponse> = ApiRequest::delete(format!(
            "{}/{}",
            CALENDAR_V4_EXCHANGE_BINDINGS, self.exchange_binding_id
        ));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "解除 Exchange 账户绑定")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../calendar/v4/exchange_bindings/{id} → DeleteExchangeBindingResponse（data.deleted）。
    #[tokio::test]
    async fn test_delete_exchange_binding_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/calendar/v4/exchange_bindings/binding_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "deleted": true }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = DeleteExchangeBindingRequest::new(config)
            .exchange_binding_id("binding_001")
            .execute()
            .await
            .expect("解除 Exchange 绑定应成功");
        assert_eq!(resp.deleted, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/exchange_bindings/binding_001"
        );
        assert_eq!(received[0].method, "DELETE");
    }
}
