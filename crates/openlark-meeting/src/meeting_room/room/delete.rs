//! 删除会议室
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/delete-meeting-room>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::common::api_endpoints::MeetingRoomApi;
use crate::meeting_room::responses::DeleteRoomResponse;

/// 删除会议室请求
pub struct DeleteRoomRequest {
    config: Config,
    room_id: String,
}

impl DeleteRoomRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/delete-meeting-room>
    pub async fn execute(self) -> SDKResult<DeleteRoomResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteRoomResponse> {
        validate_required!(self.room_id, "room_id 不能为空");

        // url: DELETE:/open-apis/meeting_room/rooms/:room_id
        let api_endpoint = MeetingRoomApi::RoomDelete(self.room_id.clone());
        let req: ApiRequest<DeleteRoomResponse> =
            ApiRequest::delete(api_endpoint.to_url()).body(serde_json::json!({
                "room_id": self.room_id
            }));

        Transport::request_typed(req, &self.config, Some(option), "删除会议室").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../meeting_room/rooms/{room_id} → DeleteRoomResponse。
    #[tokio::test]
    async fn test_delete_room_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/meeting_room/rooms/room_001"))
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

        let resp = DeleteRoomRequest::new(config)
            .room_id("room_001")
            .execute()
            .await
            .expect("删除会议室应成功");
        assert_eq!(resp, DeleteRoomResponse {});

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/meeting_room/rooms/room_001"
        );
        assert_eq!(received[0].method, "DELETE");
    }
}
