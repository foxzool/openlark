//! 加入会议
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::serialize_params;
use crate::vc::vc::v1::bot::models::BotMeetingUser;

/// 加入会议时的识别信息。
///
/// 当前文档仅支持 `join_type = 1`（按会议号入会），此时需提供 `meeting_no`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinIdentify {
    /// 会议号。当 `join_type` 为 1 时必填。
    pub meeting_no: String,
}

/// 加入会议请求体。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct JoinBotBody {
    /// 入会方式。有效范围 1～100；当前仅支持 `1`（按会议号）。
    pub join_type: i32,
    /// 会议识别信息。
    pub join_identify: JoinIdentify,
    /// 会议密码。目标会议无密码时可省略。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 邀请入会关联 ID，仅在响应邀请时原样回传。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
}

/// 加入会议后返回的会议信息。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotJoinedMeeting {
    /// 会议 ID（后续离会、发消息使用的长 ID）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 会议号。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_no: Option<String>,
    /// 开始时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 会议主题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

/// 加入会议响应数据。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct JoinBotResponse {
    /// 会议信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting: Option<BotJoinedMeeting>,
    /// 入会的机器人对应用户。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_user: Option<BotMeetingUser>,
}

impl ApiResponseTrait for JoinBotResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 加入会议请求。
#[derive(Debug, Clone)]
pub struct JoinBotRequest {
    config: Config,
}

impl JoinBotRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: JoinBotBody) -> SDKResult<JoinBotResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: JoinBotBody,
        option: RequestOption,
    ) -> SDKResult<JoinBotResponse> {
        validate_join_body(&body)?;

        let req: ApiRequest<JoinBotResponse> =
            ApiRequest::post(VcApiV1::BotJoin.to_url()).body(serialize_params(&body, "加入会议")?);
        Transport::request_typed(req, &self.config, Some(option), "加入会议").await
    }
}

fn validate_join_body(body: &JoinBotBody) -> SDKResult<()> {
    if body.join_type < 1 {
        return Err(CoreError::validation_msg("join_type 不能为空"));
    }
    if body.join_type > 100 {
        return Err(CoreError::validation_msg("join_type 取值范围为 1～100"));
    }
    validate_required!(body.join_identify.meeting_no, "join_identify 不能为空");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_join_url() {
        assert_eq!(VcApiV1::BotJoin.to_url(), "/open-apis/vc/v1/bots/join");
    }
}
