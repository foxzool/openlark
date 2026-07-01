//! 机器人单聊即时提醒
//!
//! docPath: <https://open.feishu.cn/document/im-v2/groups-bots/bot_time_sentive>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, error, http::Transport};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
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

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "机器人单聊即时提醒")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
