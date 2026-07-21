//! 获取会议室列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-meeting-room-list>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::endpoints::MEETING_ROOM;
use crate::meeting_room::responses::ListRoomResponse;

/// 获取会议室列表请求
pub struct ListRoomRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl ListRoomRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数。
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-meeting-room-list>
    pub async fn execute(self) -> SDKResult<ListRoomResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListRoomResponse> {
        // url: GET:/open-apis/meeting_room/room/list
        let mut req: ApiRequest<ListRoomResponse> =
            ApiRequest::get(format!("{MEETING_ROOM}/room/list"));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }
        Transport::request_typed(req, &self.config, Some(option), "获取会议室列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_room_request_builder() {
        let config = Config::default();
        let request = ListRoomRequest::new(config)
            .query_param("building_id", "bld_123")
            .query_param("floor_name", "3F");

        assert_eq!(request.query_params.len(), 2);
        assert_eq!(
            request.query_params[0],
            ("building_id".to_string(), "bld_123".to_string())
        );
        assert_eq!(
            request.query_params[1],
            ("floor_name".to_string(), "3F".to_string())
        );
    }

    #[test]
    fn test_list_room_request_minimal() {
        let config = Config::default();
        let request = ListRoomRequest::new(config);

        assert!(request.query_params.is_empty());
    }

    #[test]
    fn test_list_room_request_single_param() {
        let config = Config::default();
        let request = ListRoomRequest::new(config).query_param("page_size", "50");

        assert_eq!(request.query_params.len(), 1);
        assert_eq!(
            request.query_params[0],
            ("page_size".to_string(), "50".to_string())
        );
    }

    /// 端到端：GET .../meeting_room/room/list → ListRoomResponse。
    #[tokio::test]
    async fn test_list_room_returns_typed_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/meeting_room/room/list"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "page_token": "1",
                    "has_more": false,
                    "rooms": [{
                        "room_id": "omm_eada1d61a550955240c28757e7dec3af",
                        "building_id": "omb_8ec170b937536a5d87c23b418b83f9bb",
                        "building_name": "Building name",
                        "capacity": 14,
                        "description": "Some description",
                        "display_id": "FM537532166",
                        "floor_name": "F1",
                        "is_disabled": false,
                        "name": "Room name"
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

        let resp = ListRoomRequest::new(config)
            .query_param("building_id", "bld_123")
            .execute()
            .await
            .expect("获取会议室列表应成功");
        assert_eq!(resp.has_more, Some(false));
        assert_eq!(resp.rooms[0].capacity, Some(14));
        assert_eq!(resp.rooms[0].name.as_deref(), Some("Room name"));
    }
}
