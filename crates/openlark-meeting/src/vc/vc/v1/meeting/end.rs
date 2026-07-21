//! 结束会议
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/end>

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

/// 结束会议请求
#[derive(Debug, Clone)]
pub struct EndMeetingRequest {
    config: Config,
    meeting_id: String,
}

/// 结束会议响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EndMeetingResponse {
    /// 操作结果
    pub success: bool,
}

impl ApiResponseTrait for EndMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EndMeetingRequest {
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
    /// 说明：如文档不要求请求体，可传入空对象 `{}`。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/end>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<EndMeetingResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<EndMeetingResponse> {
        validate_required!(self.meeting_id, "meeting_id 不能为空");

        let api_endpoint = VcApiV1::MeetingEnd(self.meeting_id);
        let req: ApiRequest<EndMeetingResponse> =
            ApiRequest::patch(api_endpoint.to_url()).body(serialize_params(&body, "结束会议")?);

        Transport::request_typed(req, &self.config, Some(option), "结束会议").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../meetings/{id}/end → 强类型 EndMeetingResponse 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_end_meeting_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/vc/v1/meetings/mtg_001/end"))
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

        let resp = EndMeetingRequest::new(config)
            .meeting_id("mtg_001")
            .execute(json!({}))
            .await
            .expect("结束会议应成功");
        assert!(resp.success);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/meetings/mtg_001/end"
        );
    }
}
