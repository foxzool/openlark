//! 机器人单聊即时提醒
//!
//! docPath: <https://open.feishu.cn/document/im-v2/groups-bots/bot_time_sentive>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, error, http::Transport};

use crate::{
    common::api_utils::serialize_params,
    endpoints::IM_V2_FEED_CARDS,
    im::v1::message::models::UserIdType,
    im::v2::feed_card::models::{FeedCardActionResponse, FeedCardTimeSensitiveBody},
};

/// 机器人单聊即时提醒请求
pub struct BotTimeSentiveRequest {
    /// 配置信息。
    config: Config,
    /// 用户 ID 类型。
    user_id_type: Option<UserIdType>,
}

impl BotTimeSentiveRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
        }
    }

    /// 用户 ID 类型（查询参数，必填）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/im-v2/groups-bots/bot_time_sentive>
    pub async fn execute(
        self,
        body: FeedCardTimeSensitiveBody,
    ) -> SDKResult<FeedCardActionResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: FeedCardTimeSensitiveBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<FeedCardActionResponse> {
        if body.user_ids.is_empty() {
            return Err(error::validation_error(
                "user_ids 不能为空".to_string(),
                "user_ids 至少需要 1 个".to_string(),
            ));
        }
        let user_id_type = self.user_id_type.ok_or_else(|| {
            error::validation_error(
                "user_id_type 不能为空".to_string(),
                "机器人单聊即时提醒需要指定 user_id_type".to_string(),
            )
        })?;

        // url: PATCH:/open-apis/im/v2/feed_cards/bot_time_sentive
        let req: ApiRequest<FeedCardActionResponse> =
            ApiRequest::patch(format!("{IM_V2_FEED_CARDS}/bot_time_sentive"))
                .query("user_id_type", user_id_type.as_str())
                .body(serialize_params(&body, "机器人单聊即时提醒")?);

        Transport::request_typed(req, &self.config, Some(option), "机器人单聊即时提醒").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH /open-apis/im/v2/feed_cards/bot_time_sentive
    #[tokio::test]
    async fn test_bot_time_sentive_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/im/v2/feed_cards/bot_time_sentive"))
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
        let body: FeedCardTimeSensitiveBody = serde_json::from_value(json!({
            "time_sensitive": true,
            "user_ids": ["test001"]
        }))
        .expect("body 构造");
        BotTimeSentiveRequest::new(config)
            .user_id_type(UserIdType::OpenId)
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
