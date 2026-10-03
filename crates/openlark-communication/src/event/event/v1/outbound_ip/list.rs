//! 获取事件出口 IP
//!
//! docPath: <https://open.feishu.cn/document/server-docs/event-subscription-guide/list>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport};

use crate::endpoints::EVENT_V1_OUTBOUND_IP;

/// 获取事件出口 IP 请求
pub struct ListOutboundIpRequest {
    config: Config,
}

impl ListOutboundIpRequest {
    /// 创建获取事件出口 IP 请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/event-subscription-guide/list>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/event/v1/outbound_ip
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(EVENT_V1_OUTBOUND_IP);
        Transport::request_typed(req, &self.config, Some(option), "获取事件出口 IP").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/event/v1/outbound_ip
    #[tokio::test]
    async fn test_list_outbound_ip_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/event/v1/outbound_ip"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        ListOutboundIpRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
