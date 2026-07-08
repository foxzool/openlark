//! 查询会议明细
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get>

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

/// 查询会议明细请求
#[derive(Debug, Clone)]
pub struct GetMeetingListRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

/// 查询会议明细响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetMeetingListResponse {
    /// 会议列表数据
    pub data: serde_json::Value,
}

impl ApiResponseTrait for GetMeetingListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMeetingListRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get>
    pub async fn execute(self) -> SDKResult<GetMeetingListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMeetingListResponse> {
        let api_endpoint = VcApiV1::MeetingListList;
        let mut req: ApiRequest<GetMeetingListResponse> = ApiRequest::get(api_endpoint.to_url());
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询会议明细")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../meeting_list + query 拼装 + 强类型 GetMeetingListResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_meeting_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/meeting_list"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "total": 2, "items": [{ "meeting_id": "m_001" }, { "meeting_id": "m_002" }] } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetMeetingListRequest::new(config)
            .query_param("start_time", "1704067200")
            .query_param("end_time", "1706745599")
            .execute()
            .await
            .expect("查询会议明细应成功");
        assert_eq!(resp.data["total"], 2);
        assert_eq!(resp.data["items"].as_array().unwrap().len(), 2);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/meeting_list");
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("start_time=1704067200"));
        assert!(query.contains("end_time=1706745599"));
    }
}
