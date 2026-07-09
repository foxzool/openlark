//! 删除仅特定人可见的消息卡片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-card/delete-message-cards-that-are-only-visible-to-certain-people>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::EPHEMERAL_V1_DELETE,
};

/// 删除仅特定人可见的消息卡片请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEphemeralBody {
    /// 消息卡片 ID
    pub message_id: String,
}

/// 删除仅特定人可见的消息卡片请求
pub struct DeleteEphemeralRequest {
    config: Config,
}

impl DeleteEphemeralRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-card/delete-message-cards-that-are-only-visible-to-certain-people>
    pub async fn execute(self, body: DeleteEphemeralBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: DeleteEphemeralBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(&body.message_id, "message_id 不能为空");

        // url: POST:/open-apis/ephemeral/v1/delete
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(EPHEMERAL_V1_DELETE)
            .body(serialize_params(&body, "删除仅特定人可见的消息卡片")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "删除仅特定人可见的消息卡片")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/ephemeral/v1/delete
    #[tokio::test]
    async fn test_delete_ephemeral_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/ephemeral/v1/delete"))
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

        let body: DeleteEphemeralBody =
            serde_json::from_value(json!({ "message_id": "test001" })).expect("body 构造");
        DeleteEphemeralRequest::new(config)
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
