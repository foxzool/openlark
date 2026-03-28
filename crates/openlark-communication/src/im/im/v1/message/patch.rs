//! 更新已发送的消息卡片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-card/patch>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    common::models::EmptyData,
    endpoints::IM_V1_MESSAGES,
};

/// 更新已发送的消息卡片请求
///
/// 用于更新已发送的消息卡片内容。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 待更新的消息 ID，必填
///
/// # 请求体说明
///
/// 该接口支持两种请求体形态：
/// 1) `{"content": "..."}` - 传入卡片 JSON（需为 JSON 序列化后的字符串）
/// 2) `{"type":"template","data":{...}}` - 传入卡片模板（搭建工具）
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::message::PatchMessageCardRequest;
/// use serde_json::json;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let body = json!({"content": "{\"elements\":[{\"tag\":\"div\",\"text\":{\"content\":\"更新内容\"}}]}"});
/// let request = PatchMessageCardRequest::new(config)
///     .message_id("om_xxx");
/// let response = request.execute(body).await?;
/// ```
pub struct PatchMessageCardRequest {
    config: Config,
    message_id: String,
}

impl PatchMessageCardRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 待更新的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口支持两种请求体形态：
    /// 1) `{"content": "..."}` 传入卡片 JSON（需为 JSON 序列化后的字符串）
    /// 2) `{"type":"template","data":{...}}` 传入卡片模板（搭建工具）
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-card/patch>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.message_id, "message_id 不能为空");

        // url: PATCH:/open-apis/im/v1/messages/:message_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::patch(format!("{}/{}", IM_V1_MESSAGES, self.message_id))
                .body(serialize_params(&body, "更新已发送的消息卡片")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新已发送的消息卡片")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_patch_message_card_request_builder() {
        let config = Config::default();
        let request = PatchMessageCardRequest::new(config).message_id("om_xxx");
        assert_eq!(request.message_id, "om_xxx");
    }

    #[test]
    fn test_patch_body_with_content() {
        let body = json!({
            "content": "{\"elements\":[{\"tag\":\"div\",\"text\":{\"content\":\"测试内容\"}}]}"
        });
        assert!(body.get("content").is_some());
    }

    #[test]
    fn test_patch_body_with_template() {
        let body = json!({
            "type": "template",
            "data": {
                "template_id": "tpl_xxx"
            }
        });
        assert_eq!(body.get("type"), Some(&json!("template")));
        assert!(body.get("data").is_some());
    }

    #[test]
    fn test_empty_message_id() {
        let config = Config::default();
        let request = PatchMessageCardRequest::new(config).message_id("");
        assert_eq!(request.message_id, "");
    }

    #[test]
    fn test_patch_message_card_request_builder_chaining() {
        let config = Config::default();
        let request = PatchMessageCardRequest::new(config).message_id("om_test_123");
        assert_eq!(request.message_id, "om_test_123");
    }

    #[test]
    fn test_patch_body_empty() {
        let body = json!({});
        assert_eq!(body.as_object().unwrap().len(), 0);
    }
}
