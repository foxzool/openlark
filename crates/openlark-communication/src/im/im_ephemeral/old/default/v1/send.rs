//! 发送仅特定人可见的消息卡片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-card/send-message-cards-that-are-only-visible-to-certain-people>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{common::api_utils::serialize_params, endpoints::EPHEMERAL_V1_SEND};

/// 发送仅特定人可见的消息卡片请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEphemeralBody {
    /// 接收用户的 ID 列表
    pub user_id_list: Vec<String>,

    /// 消息卡片内容（JSON 格式）
    pub card: Value,
}

/// 发送仅特定人可见的消息卡片请求
pub struct SendEphemeralRequest {
    config: Config,
}

impl SendEphemeralRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-card/send-message-cards-that-are-only-visible-to-certain-people>
    pub async fn execute(self, body: SendEphemeralBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: SendEphemeralBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(&body.user_id_list, "user_id_list 不能为空");

        // url: POST:/open-apis/ephemeral/v1/send
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(EPHEMERAL_V1_SEND)
            .body(serialize_params(&body, "发送仅特定人可见的消息卡片")?);

        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "发送仅特定人可见的消息卡片",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/ephemeral/v1/send
    #[tokio::test]
    async fn test_send_ephemeral_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/ephemeral/v1/send"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body: SendEphemeralBody =
            serde_json::from_value(json!({ "user_id_list": ["test001"], "card": {} }))
                .expect("body 构造");
        SendEphemeralRequest::new(config)
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
