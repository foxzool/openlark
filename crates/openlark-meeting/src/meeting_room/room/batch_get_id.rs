//! 查询会议室ID
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-meeting-room-id>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::endpoints::MEETING_ROOM;
use crate::meeting_room::responses::BatchGetRoomIdResponse;

/// 查询会议室ID请求
pub struct BatchGetRoomIdRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl BatchGetRoomIdRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-meeting-room-id>
    pub async fn execute(self) -> SDKResult<BatchGetRoomIdResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchGetRoomIdResponse> {
        // url: GET:/open-apis/meeting_room/room/batch_get_id
        let mut req: ApiRequest<BatchGetRoomIdResponse> =
            ApiRequest::get(format!("{MEETING_ROOM}/room/batch_get_id"));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }
        Transport::request_typed(req, &self.config, Some(option), "查询会议室ID").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../meeting_room/room/batch_get_id → BatchGetRoomIdResponse。
    #[tokio::test]
    async fn test_batch_get_id_room_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/meeting_room/room/batch_get_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "rooms": [
                        { "room_id": "room_001", "custom_room_id": "test01" },
                        { "room_id": "room_002", "custom_room_id": "test02" }
                    ]
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

        let resp = BatchGetRoomIdRequest::new(config)
            .query_param("page_size", "20")
            .execute()
            .await
            .expect("查询会议室ID应成功");
        assert_eq!(resp.rooms[0].room_id, "room_001");
        assert_eq!(resp.rooms[0].custom_room_id.as_deref(), Some("test01"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/meeting_room/room/batch_get_id"
        );
        assert_eq!(received[0].url.query(), Some("page_size=20"));
    }
}
