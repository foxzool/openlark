//! 预约会议
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/apply>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 预约会议请求

#[derive(Debug, Clone)]
pub struct ApplyReserveRequest {
    /// 配置信息
    config: Config,
}

/// 预约会议响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApplyReserveResponse {
    /// 会议 ID
    pub meeting_id: String,
    /// 预约 ID
    pub reserve_id: String,
}

impl ApiResponseTrait for ApplyReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApplyReserveRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/apply>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<ApplyReserveResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<ApplyReserveResponse> {
        let api_request: ApiRequest<ApplyReserveResponse> =
            ApiRequest::post("/open-apis/vc/v1/reserves/apply").body(serde_json::to_vec(&body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "预约会议").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../vc/v1/reserves/apply → 强类型 ApplyReserveResponse 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_apply_reserve_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/vc/v1/reserves/apply"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "meeting_id": "m_001", "reserve_id": "r_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ApplyReserveRequest::new(config)
            .execute(json!({ "topic": "测试预约" }))
            .await
            .expect("预约会议应成功");
        assert_eq!(resp.meeting_id, "m_001");
        assert_eq!(resp.reserve_id, "r_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/reserves/apply");
    }
}
