//! 获取活跃会议
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/get_active_meeting>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

/// 获取活跃会议请求
pub struct GetActiveMeetingRequest {
    config: Config,
    reserve_id: String,
    query_params: Vec<(String, String)>,
}

impl GetActiveMeetingRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 预约 ID（路径参数）
    pub fn reserve_id(mut self, reserve_id: impl Into<String>) -> Self {
        self.reserve_id = reserve_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/get_active_meeting>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.reserve_id, "reserve_id 不能为空");

        // url: GET:/open-apis/vc/v1/reserves/:reserve_id/get_active_meeting
        let api_endpoint = format!(
            "/open-apis/vc/v1/reserves/{}/get_active_meeting",
            self.reserve_id
        );
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(&api_endpoint);
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        Transport::request_typed(req, &self.config, Some(option), "获取活跃会议").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../vc/v1/reserves/{reserve_id}/get_active_meeting → serde_json::Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_get_active_meeting_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/reserves/r_001/get_active_meeting"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "meeting_id": "m_001", "status": "ongoing" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetActiveMeetingRequest::new(config)
            .reserve_id("r_001")
            .execute()
            .await
            .expect("获取活跃会议应成功");
        assert_eq!(resp["meeting_id"], "m_001");
        assert_eq!(resp["status"], "ongoing");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/reserves/r_001/get_active_meeting"
        );
    }
}
