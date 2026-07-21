//! 延时更新消息卡片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-card/delay-update-message-card>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{common::api_utils::serialize_params, endpoints::INTERACTIVE_V1_CARD_UPDATE};

/// 延时更新消息卡片请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardBody {
    /// 消息卡片 token
    pub token: String,

    /// 消息卡片内容（JSON 格式）
    pub card: Value,

    /// 消息卡片更新的触发关键词（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_keyword: Option<String>,
}

/// 延时更新消息卡片请求
pub struct UpdateCardRequest {
    config: Config,
}

impl UpdateCardRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-card/delay-update-message-card>
    pub async fn execute(self, body: UpdateCardBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: UpdateCardBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(&body.token, "token 不能为空");

        // url: POST:/open-apis/interactive/v1/card/update
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(INTERACTIVE_V1_CARD_UPDATE)
            .body(serialize_params(&body, "延时更新消息卡片")?);

        Transport::request_typed(req, &self.config, Some(option), "延时更新消息卡片").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/interactive/v1/card/update
    #[tokio::test]
    async fn test_update_card_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/interactive/v1/card/update"))
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

        let body: UpdateCardBody =
            serde_json::from_value(json!({ "token": "test001", "card": {} })).expect("body 构造");
        UpdateCardRequest::new(config)
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
