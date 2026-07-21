//! 更新会议室
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/update-meeting-room>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::meeting_room::responses::UpdateRoomResponse;
use crate::{common::api_endpoints::MeetingRoomApi, common::api_utils::serialize_params};

/// 更新会议室请求
pub struct UpdateRoomRequest {
    config: Config,
    room_id: String,
}

impl UpdateRoomRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            room_id: String::new(),
        }
    }

    /// 会议室 ID
    pub fn room_id(mut self, room_id: impl Into<String>) -> Self {
        self.room_id = room_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/update-meeting-room>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<UpdateRoomResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<UpdateRoomResponse> {
        validate_required!(self.room_id, "room_id 不能为空");

        let api_endpoint = MeetingRoomApi::RoomUpdate;
        let req: ApiRequest<UpdateRoomResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "更新会议室")?);

        Transport::request_typed(req, &self.config, Some(option), "更新会议室").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../meeting_room/room/update → UpdateRoomResponse。
    #[tokio::test]
    async fn test_update_room_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/meeting_room/room/update"))
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

        let resp = UpdateRoomRequest::new(config)
            .room_id("room_001")
            .execute(json!({ "name": "更新大会议室" }))
            .await
            .expect("更新会议室应成功");
        assert_eq!(resp, UpdateRoomResponse {});

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/meeting_room/room/update"
        );
        assert_eq!(received[0].method, "POST");
    }
}
