//! 离开会议
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/leave>

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
use crate::vc::vc::v1::bot::models::BotMeetingUser;

/// 离开会议请求体。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaveBotBody {
    /// 目标会议 ID。使用加入会议返回的长 ID，不是 9 位会议号。
    pub meeting_id: String,
}

/// 离开会议响应数据。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LeaveBotResponse {
    /// 离会的机器人对应用户。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_user: Option<BotMeetingUser>,
}

impl ApiResponseTrait for LeaveBotResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 离开会议请求。
#[derive(Debug, Clone)]
pub struct LeaveBotRequest {
    config: Config,
}

impl LeaveBotRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: LeaveBotBody) -> SDKResult<LeaveBotResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: LeaveBotBody,
        option: RequestOption,
    ) -> SDKResult<LeaveBotResponse> {
        validate_required!(body.meeting_id, "meeting_id 不能为空");

        let req: ApiRequest<LeaveBotResponse> =
            ApiRequest::post(VcApiV1::BotLeave.to_url()).body(serialize_params(&body, "离开会议")?);
        Transport::request_typed(req, &self.config, Some(option), "离开会议").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_leave_url() {
        assert_eq!(VcApiV1::BotLeave.to_url(), "/open-apis/vc/v1/bots/leave");
    }
}
