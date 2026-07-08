//! 查询参会人明细
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get-2>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::extract_response_data;

/// 查询参会人明细请求
#[derive(Debug, Clone)]
pub struct GetParticipantListRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

/// 查询参会人明细响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetParticipantListResponse {
    /// 参会人列表数据
    pub data: serde_json::Value,
}

impl ApiResponseTrait for GetParticipantListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetParticipantListRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get-2>
    pub async fn execute(self) -> SDKResult<GetParticipantListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetParticipantListResponse> {
        let api_endpoint = VcApiV1::ParticipantListList;
        let mut req: ApiRequest<GetParticipantListResponse> =
            ApiRequest::get(api_endpoint.to_url());
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询参会人明细")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../participant_list + query 拼装 + 强类型 GetParticipantListResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_participant_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/participant_list"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "items": [{ "user_id": "u_001" }, { "user_id": "u_002" }] } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetParticipantListRequest::new(config)
            .query_param("meeting_id", "m_001")
            .query_param("page_size", "20")
            .execute()
            .await
            .expect("查询参会人明细应成功");
        assert_eq!(resp.data["items"].as_array().unwrap().len(), 2);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/participant_list");
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("meeting_id=m_001"));
        assert!(query.contains("page_size=20"));
    }
}
