//! 查询参会人会议质量数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get-3>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_endpoints::VcApiV1;

/// 查询参会人会议质量数据请求
pub struct GetParticipantQualityListRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl GetParticipantQualityListRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get-3>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/vc/v1/participant_quality_list
        let api_endpoint = VcApiV1::ParticipantQualityListList;
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(api_endpoint.to_url());
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        Transport::request_typed(req, &self.config, Some(option), "查询参会人会议质量数据").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../participant_quality_list + query 拼装 + 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_get_participant_quality_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/participant_quality_list"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "items": [{ "user_id": "u_001", "score": 95 }], "has_more": false }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetParticipantQualityListRequest::new(config)
            .query_param("meeting_id", "m_001")
            .query_param("page_size", "20")
            .execute()
            .await
            .expect("查询参会人会议质量数据应成功");
        assert_eq!(resp["items"].as_array().unwrap().len(), 1);
        assert_eq!(resp["has_more"], false);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/participant_quality_list"
        );
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("meeting_id=m_001"));
        assert!(query.contains("page_size=20"));
    }
}
