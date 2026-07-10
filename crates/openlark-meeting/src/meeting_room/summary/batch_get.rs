//! 查询会议室日程主题和会议详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_endpoints::MeetingRoomApi;
use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::meeting_room::responses::BatchGetSummaryResponse;

/// 查询会议室日程主题和会议详情请求
pub struct BatchGetSummaryRequest {
    config: Config,
}

impl BatchGetSummaryRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<BatchGetSummaryResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<BatchGetSummaryResponse> {
        let api_endpoint = MeetingRoomApi::RoomBatchGetSummary;
        let req: ApiRequest<BatchGetSummaryResponse> = ApiRequest::post(api_endpoint.to_url())
            .body(serialize_params(&body, "查询会议室日程主题和会议详情")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询会议室日程主题和会议详情")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../meeting_room/rooms/batch_get_summary → BatchGetSummaryResponse。
    #[tokio::test]
    async fn test_batch_get_summary_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/meeting_room/rooms/batch_get_summary"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "ErrorEventUids": [],
                    "EventInfos": [{
                        "original_time": 0,
                        "summary": "周会",
                        "uid": "a04dbea1-86b9-4372-aa8d-64ebe801be2a",
                        "vchat": {
                            "meeting_url": "https://vc.feishu.cn/j/935314044",
                            "vc_type": "vc"
                        }
                    }]
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

        let resp = BatchGetSummaryRequest::new(config)
            .execute(json!({ "room_ids": ["room_001"] }))
            .await
            .expect("查询会议室日程主题和会议详情应成功");
        assert!(resp.error_event_uids.is_empty());
        assert_eq!(resp.event_infos[0].summary.as_deref(), Some("周会"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/meeting_room/rooms/batch_get_summary"
        );
        assert_eq!(received[0].method, "POST");
    }
}
