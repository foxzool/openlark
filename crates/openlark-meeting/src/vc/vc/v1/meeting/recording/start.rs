//! 开始录制
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-recording/start>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 开始录制请求
pub struct StartRecordingRequest {
    config: Config,
    meeting_id: String,
}

impl StartRecordingRequest {
    /// 创建新的请求构建器。
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-recording/start>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.meeting_id, "meeting_id 不能为空");

        // url: PATCH:/open-apis/vc/v1/meetings/:meeting_id/recording/start
        let api_endpoint = VcApiV1::MeetingRecordingStart(self.meeting_id);
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::patch(api_endpoint.to_url()).body(serialize_params(&body, "开始录制")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "开始录制")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../meetings/{id}/recording/start → 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_start_recording_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/vc/v1/meetings/mtg_001/recording/start"))
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

        let resp = StartRecordingRequest::new(config)
            .meeting_id("mtg_001")
            .execute(json!({}))
            .await
            .expect("开始录制应成功");
        assert_eq!(resp["success"], json!(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/meetings/mtg_001/recording/start"
        );
    }
}
