//! 获取会议详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, validate_required_field};
use serde::{Deserialize, Serialize};

/// 获取会议详情请求

#[derive(Debug, Clone)]
pub struct GetMeetingRequest {
    /// 配置信息
    config: Config,
    /// 会议 ID（路径参数）
    meeting_id: String,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 获取会议详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetMeetingResponse {
    /// 会议 ID
    pub meeting_id: String,
    /// 会议主题
    pub topic: String,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
}

impl ApiResponseTrait for GetMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMeetingRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            meeting_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 设置会议 ID（路径参数）
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/get>
    pub async fn execute(self) -> SDKResult<GetMeetingResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMeetingResponse> {
        validate_required_field("meeting_id", Some(&self.meeting_id), "会议 ID 不能为空")?;

        let api_endpoint = VcApiV1::MeetingGet(self.meeting_id.clone());
        let mut api_request: ApiRequest<GetMeetingResponse> =
            ApiRequest::get(api_endpoint.to_url());

        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取会议详情")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../meetings/{id} → 强类型 GetMeetingResponse 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_get_meeting_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/meetings/mtg_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "meeting_id": "mtg_001",
                    "topic": "周会",
                    "start_time": "2026-07-08 10:00:00",
                    "end_time": "2026-07-08 11:00:00"
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

        let resp = GetMeetingRequest::new(config)
            .meeting_id("mtg_001")
            .query_param("user_id_type", "open_id")
            .execute()
            .await
            .expect("获取会议详情应成功");
        assert_eq!(resp.meeting_id, "mtg_001");
        assert_eq!(resp.topic, "周会");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/meetings/mtg_001");
        assert_eq!(received[0].url.query(), Some("user_id_type=open_id"));
    }
}
