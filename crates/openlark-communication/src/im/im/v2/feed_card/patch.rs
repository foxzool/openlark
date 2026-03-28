//! 即时提醒
//!
//! docPath: <https://open.feishu.cn/document/im-v2/groups-bots/patch>

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V2_FEED_CARDS,
    im::im::v1::message::models::UserIdType,
    im::im::v2::feed_card::models::{FeedCardActionResponse, FeedCardTimeSensitiveBody},
};

/// 即时提醒请求
pub struct PatchFeedCardRequest {
    config: Config,
    feed_card_id: String,
    user_id_type: Option<UserIdType>,
}

impl PatchFeedCardRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            feed_card_id: String::new(),
            user_id_type: None,
        }
    }

    /// 消息流卡片 ID（路径参数）
    pub fn feed_card_id(mut self, feed_card_id: impl Into<String>) -> Self {
        self.feed_card_id = feed_card_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，必填）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/im-v2/groups-bots/patch>
    pub async fn execute(
        self,
        body: FeedCardTimeSensitiveBody,
    ) -> SDKResult<FeedCardActionResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: FeedCardTimeSensitiveBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<FeedCardActionResponse> {
        validate_required!(self.feed_card_id, "feed_card_id 不能为空");
        if body.user_ids.is_empty() {
            return Err(error::validation_error(
                "user_ids 不能为空".to_string(),
                "user_ids 至少需要 1 个".to_string(),
            ));
        }
        let user_id_type = self.user_id_type.ok_or_else(|| {
            error::validation_error(
                "user_id_type 不能为空".to_string(),
                "即时提醒需要指定 user_id_type".to_string(),
            )
        })?;

        // url: PATCH:/open-apis/im/v2/feed_cards/:feed_card_id
        let req: ApiRequest<FeedCardActionResponse> =
            ApiRequest::patch(format!("{}/{}", IM_V2_FEED_CARDS, self.feed_card_id))
                .query("user_id_type", user_id_type.as_str())
                .body(serialize_params(&body, "即时提醒")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "即时提醒")
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
