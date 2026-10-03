//! 创建会议室
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/create-meeting-room>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_endpoints::MeetingRoomApi;
use crate::common::api_utils::serialize_params;
use crate::meeting_room::responses::CreateRoomResponse;

/// 创建会议室请求
pub struct CreateRoomRequest {
    config: Config,
}

impl CreateRoomRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/create-meeting-room>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateRoomResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<CreateRoomResponse> {
        // url: POST:/open-apis/meeting_room/rooms
        let api_endpoint = MeetingRoomApi::RoomCreate;
        let req: ApiRequest<CreateRoomResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "创建会议室")?);

        Transport::request_typed(req, &self.config, Some(option), "创建会议室").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_room_request_builder() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateRoomRequest::new(config);

        // 验证 request 被成功创建且配置正确
        assert_eq!(request.config.app_id(), "test_app");
    }

    #[test]
    fn test_create_room_request_new() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateRoomRequest::new(config);

        // 验证配置正确
        assert_eq!(request.config.app_id(), "test_app");
        assert_eq!(request.config.app_secret(), "test_secret");
    }

    /// 端到端：POST .../meeting_room/rooms → CreateRoomResponse。
    #[tokio::test]
    async fn test_create_room_returns_typed_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/meeting_room/rooms"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "room_id": "omm_eada1d61a550955240c28757e7dec3af" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CreateRoomRequest::new(config)
            .execute(json!({ "name": "测试会议室", "capacity": 10 }))
            .await
            .expect("创建会议室应成功");
        assert_eq!(resp.room_id, "omm_eada1d61a550955240c28757e7dec3af");
    }
}
