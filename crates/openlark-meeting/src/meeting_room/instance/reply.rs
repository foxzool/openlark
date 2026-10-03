//! 回复会议室日程实例
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/reply-meeting-room-event-instance>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_endpoints::MeetingRoomApi;
use crate::common::api_utils::serialize_params;
use crate::meeting_room::responses::ReplyInstanceResponse;

/// 回复会议室日程实例请求
pub struct ReplyInstanceRequest {
    config: Config,
}

impl ReplyInstanceRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/reply-meeting-room-event-instance>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<ReplyInstanceResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<ReplyInstanceResponse> {
        let api_endpoint = MeetingRoomApi::InstanceReplyOld;
        let req: ApiRequest<ReplyInstanceResponse> = ApiRequest::post(api_endpoint.to_url())
            .body(serialize_params(&body, "回复会议室日程实例")?);

        Transport::request_typed(req, &self.config, Some(option), "回复会议室日程实例").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../meeting_room/instance/reply → ReplyInstanceResponse。
    #[tokio::test]
    async fn test_reply_instance_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/meeting_room/instance/reply"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success"
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ReplyInstanceRequest::new(config)
            .execute(json!({ "instance_id": "inst_001", "reply": "accept" }))
            .await
            .expect("回复会议室日程实例应成功");
        assert_eq!(resp, ReplyInstanceResponse {});

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/meeting_room/instance/reply"
        );
        assert_eq!(received[0].method, "POST");
    }
}
