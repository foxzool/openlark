//! 发送会中消息
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/message>
//!
//! `msg_type=reaction` 时，`content` 为会中表情 key；可用表情码见飞书 Emoji 参考。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::serialize_params;

/// 发送会中消息请求体。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendBotMessageBody {
    /// 会议 ID。使用加入会议返回的长 ID，不是 9 位会议号。
    pub meeting_id: String,
    /// 消息类型。可选值：`text`、`reaction`。
    pub msg_type: String,
    /// 消息内容。`text` 时为文本；`reaction` 时为表情 key。
    pub content: String,
    /// 幂等去重 ID。未传时由服务端生成并在响应中返回。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// 发送会中消息响应数据。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendBotMessageResponse {
    /// 幂等去重 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl ApiResponseTrait for SendBotMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 发送会中消息请求。
#[derive(Debug, Clone)]
pub struct SendBotMessageRequest {
    config: Config,
}

impl SendBotMessageRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: SendBotMessageBody) -> SDKResult<SendBotMessageResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: SendBotMessageBody,
        option: RequestOption,
    ) -> SDKResult<SendBotMessageResponse> {
        validate_required!(body.meeting_id, "meeting_id 不能为空");
        validate_required!(body.msg_type, "msg_type 不能为空");
        validate_required!(body.content, "content 不能为空");

        let req: ApiRequest<SendBotMessageResponse> =
            ApiRequest::post(VcApiV1::BotMessage.to_url())
                .body(serialize_params(&body, "发送会中消息")?);
        Transport::request_typed(req, &self.config, Some(option), "发送会中消息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_message_url() {
        assert_eq!(
            VcApiV1::BotMessage.to_url(),
            "/open-apis/vc/v1/bots/message"
        );
    }
}
