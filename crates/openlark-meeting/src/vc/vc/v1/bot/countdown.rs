//! 会中倒计时
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/countdown>
//!
//! Authorization 为 tenant 或 user（默认即可）。权限点 `vc:meeting.interaction:write`。
//! 24h 上限、reminder 范围、错误码 121001~121011、频控均由服务端校验。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, error::CoreError, http::Transport,
    req_option::RequestOption, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::serialize_params;
use crate::common::models::EmptyData;

/// 会中倒计时动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BotCountdownAction {
    /// 设置倒计时。
    Set,
    /// 延长倒计时。
    Prolong,
    /// 提前结束。
    EndInAdvance,
    /// 关闭倒计时窗口。
    CloseWindow,
}

impl BotCountdownAction {
    /// 序列化字面量。
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Set => "set",
            Self::Prolong => "prolong",
            Self::EndInAdvance => "end_in_advance",
            Self::CloseWindow => "close_window",
        }
    }
}

/// 会中倒计时请求体。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BotCountdownBody {
    /// 会议 ID。
    pub meeting_id: String,
    /// 动作：`set` / `prolong` / `end_in_advance` / `close_window`。
    pub action: String,
    /// 时长（分钟，字符串）。`set` / `prolong` 时必填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// 结束时是否播放提示音，仅 `set` 生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_play_audio_at_end: Option<bool>,
    /// 结束前提醒（分钟，字符串），仅 `set` 生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_before_end: Option<String>,
}

/// 会中倒计时请求。
#[derive(Debug, Clone)]
pub struct BotCountdownRequest {
    config: Config,
}

impl BotCountdownRequest {
    /// 创建请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: BotCountdownBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: BotCountdownBody,
        option: RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_countdown_body(&body)?;

        let req: ApiRequest<EmptyData> = ApiRequest::post(VcApiV1::BotCountdown.to_url())
            .body(serialize_params(&body, "会中倒计时")?);
        Transport::request_typed(req, &self.config, Some(option), "会中倒计时").await
    }
}

fn validate_countdown_body(body: &BotCountdownBody) -> SDKResult<()> {
    validate_required!(body.meeting_id, "meeting_id 不能为空");
    validate_required!(body.action, "action 不能为空");
    let action = body.action.trim();
    if action == BotCountdownAction::Set.as_str() || action == BotCountdownAction::Prolong.as_str()
    {
        match body.duration.as_deref() {
            Some(duration) => validate_required!(duration, "duration 不能为空"),
            None => return Err(CoreError::validation_msg("duration 不能为空")),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_countdown_url() {
        assert_eq!(
            VcApiV1::BotCountdown.to_url(),
            "/open-apis/vc/v1/bots/countdown"
        );
    }

    #[test]
    fn countdown_action_snake_case() {
        assert_eq!(
            serde_json::to_value(BotCountdownAction::EndInAdvance).unwrap(),
            serde_json::json!("end_in_advance")
        );
        assert_eq!(
            serde_json::to_value(BotCountdownAction::CloseWindow).unwrap(),
            serde_json::json!("close_window")
        );
    }
}
