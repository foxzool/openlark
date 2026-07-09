//! 批量发送消息（旧版）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/batch_message/send-messages-in-batches>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport};

use crate::common::api_utils::{extract_response_data, serialize_params};

const IM_MESSAGE_V4_BATCH_SEND: &str = "/open-apis/message/v4/batch_send/";

/// 批量发送消息请求
pub struct BatchSendMessagesRequest {
    config: Config,
}

impl BatchSendMessagesRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口为旧版批量发送接口，请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/batch_message/send-messages-in-batches>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/message/v4/batch_send/
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(IM_MESSAGE_V4_BATCH_SEND)
            .body(serialize_params(&body, "批量发送消息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "批量发送消息")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/message/v4/batch_send/
    #[tokio::test]
    async fn test_batch_send_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/message/v4/batch_send/"))
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

        BatchSendMessagesRequest::new(config)
            .execute(json!({}))
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
