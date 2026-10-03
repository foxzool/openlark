//! 移除参会人
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/kickout>

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

/// 移除参会人请求
#[derive(Debug, Clone)]
pub struct KickoutMeetingRequest {
    config: Config,
    meeting_id: String,
}

/// 移除参会人响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KickoutMeetingResponse {
    /// 操作结果
    pub success: bool,
}

impl ApiResponseTrait for KickoutMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl KickoutMeetingRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            meeting_id: String::new(),
        }
    }

    /// 会议 ID（路径参数）
    pub fn meeting_id(mut self, meeting_id: impl Into<String>) -> Self {
        self.meeting_id = meeting_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/kickout>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<KickoutMeetingResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<KickoutMeetingResponse> {
        validate_required!(self.meeting_id, "meeting_id 不能为空");

        let api_endpoint = VcApiV1::MeetingKickout(self.meeting_id);
        let req: ApiRequest<KickoutMeetingResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "移除参会人")?);

        Transport::request_typed(req, &self.config, Some(option), "移除参会人").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../meetings/{id}/kickout → 强类型 KickoutMeetingResponse 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_kickout_meeting_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/vc/v1/meetings/mtg_001/kickout"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "success": true }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = KickoutMeetingRequest::new(config)
            .meeting_id("mtg_001")
            .execute(json!({"kickout_users": ["user_001"]}))
            .await
            .expect("移除参会人应成功");
        assert!(resp.success);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/meetings/mtg_001/kickout"
        );
    }
}
