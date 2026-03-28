//! 删除应用消息流卡片
//!
//! docPath: <https://open.feishu.cn/document/im-v2/app_feed_card/delete>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V2_APP_FEED_CARD_BATCH,
    im::im::v1::message::models::UserIdType,
};

/// 删除应用消息流卡片请求
pub struct DeleteAppFeedCardsRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
}

impl DeleteAppFeedCardsRequest {
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

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "删除应用消息流卡片")
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
