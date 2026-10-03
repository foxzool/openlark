//! 查询会议室详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/query-meeting-room-details>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::endpoints::MEETING_ROOM;
use crate::meeting_room::responses::BatchGetRoomResponse;

/// 查询会议室详情请求
pub struct BatchGetRoomRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl BatchGetRoomRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/query-meeting-room-details>
    pub async fn execute(self) -> SDKResult<BatchGetRoomResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchGetRoomResponse> {
        // url: GET:/open-apis/meeting_room/room/batch_get
        let mut req: ApiRequest<BatchGetRoomResponse> =
            ApiRequest::get(format!("{MEETING_ROOM}/room/batch_get"));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }
        Transport::request_typed(req, &self.config, Some(option), "查询会议室详情").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../meeting_room/room/batch_get → BatchGetRoomResponse。
    #[tokio::test]
    async fn test_batch_get_room_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/meeting_room/room/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "rooms": [
                        {
                            "room_id": "room_001",
                            "name": "大会议室",
                            "capacity": 20,
                            "is_disabled": false
                        }
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

        let resp = BatchGetRoomRequest::new(config)
            .query_param("page_size", "10")
            .execute()
            .await
            .expect("查询会议室详情应成功");
        assert_eq!(resp.rooms[0].room_id, "room_001");
        assert_eq!(resp.rooms[0].name.as_deref(), Some("大会议室"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/meeting_room/room/batch_get"
        );
        assert_eq!(received[0].url.query(), Some("page_size=10"));
    }
}
