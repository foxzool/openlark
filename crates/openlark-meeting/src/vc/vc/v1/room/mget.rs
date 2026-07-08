//! 批量查询会议室详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/mget>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::extract_response_data;

/// 批量查询会议室详情请求

#[derive(Debug, Clone)]
pub struct MgetRoomRequest {
    /// 配置信息
    config: Config,
}

/// 批量查询会议室详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MgetRoomResponse {
    /// 会议室列表
    pub rooms: Vec<RoomItem>,
}

/// 会议室信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomItem {
    /// 会议室 ID
    pub room_id: String,
    /// 会议室名称
    pub name: String,
    /// 会议室容量
    pub capacity: i32,
}

impl ApiResponseTrait for MgetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl MgetRoomRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/mget>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<MgetRoomResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<MgetRoomResponse> {
        let api_request: ApiRequest<MgetRoomResponse> =
            ApiRequest::post("/open-apis/vc/v1/rooms/mget").body(serde_json::to_vec(&body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "批量查询会议室")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../vc/v1/rooms/mget → 强类型 MgetRoomResponse（无 inner data，单层 resp.rooms）。
    #[tokio::test]
    async fn test_mget_room_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/vc/v1/rooms/mget"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "rooms": [
                        { "room_id": "room_001", "name": "大会议室", "capacity": 12 },
                        { "room_id": "room_002", "name": "小会议室", "capacity": 6 }
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

        let resp = MgetRoomRequest::new(config)
            .execute(json!({ "room_ids": ["room_001", "room_002"] }))
            .await
            .expect("批量查询会议室应成功");
        assert_eq!(resp.rooms.len(), 2);
        assert_eq!(resp.rooms[0].room_id, "room_001");
        assert_eq!(resp.rooms[1].capacity, 6);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/rooms/mget");
    }
}
