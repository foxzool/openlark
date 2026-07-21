//! 获取录制文件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-recording/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

/// 获取录制文件请求
pub struct GetRecordingRequest {
    config: Config,
    meeting_id: String,
    query_params: Vec<(String, String)>,
}

impl GetRecordingRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            meeting_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 会议 ID（路径参数）
    pub fn meeting_id(mut self, meeting_id: impl Into<String>) -> Self {
        self.meeting_id = meeting_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-recording/get>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.meeting_id, "meeting_id 不能为空");

        // url: GET:/open-apis/vc/v1/meetings/:meeting_id/recording
        let url = format!("/open-apis/vc/v1/meetings/{}/recording", self.meeting_id);
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        Transport::request_typed(req, &self.config, Some(option), "获取录制文件").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../meetings/{id}/recording → 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_get_recording_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/meetings/mtg_001/recording"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "recording": {
                        "recording_id": "rec_001",
                        "status": "completed"
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetRecordingRequest::new(config)
            .meeting_id("mtg_001")
            .query_param("user_id_type", "open_id")
            .execute()
            .await
            .expect("获取录制文件应成功");
        assert_eq!(resp["recording"]["recording_id"], "rec_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/meetings/mtg_001/recording"
        );
        assert_eq!(received[0].url.query(), Some("user_id_type=open_id"));
    }
}
