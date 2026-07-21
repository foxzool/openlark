//! 删除应用消息流卡片
//!
//! docPath: <https://open.feishu.cn/document/im-v2/app_feed_card/delete>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport};

use crate::{
    common::api_utils::serialize_params, endpoints::IM_V2_APP_FEED_CARD_BATCH,
    im::v1::message::models::UserIdType,
};

/// 删除应用消息流卡片请求
pub struct DeleteAppFeedCardsRequest {
    /// 配置信息。
    config: Config,
    /// 用户 ID 类型。
    user_id_type: Option<UserIdType>,
}

impl DeleteAppFeedCardsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
        }
    }

    /// 用户 ID 类型（查询参数，可选，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/im-v2/app_feed_card/delete>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: DELETE:/open-apis/im/v2/app_feed_card/batch
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::delete(IM_V2_APP_FEED_CARD_BATCH)
            .body(serialize_params(&body, "删除应用消息流卡片")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        Transport::request_typed(req, &self.config, Some(option), "删除应用消息流卡片").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE /open-apis/im/v2/app_feed_card/batch
    #[tokio::test]
    async fn test_delete_app_feed_cards_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/im/v2/app_feed_card/batch"))
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

        let body = json!({});
        DeleteAppFeedCardsRequest::new(config)
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
