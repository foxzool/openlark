//! 搜索会议室
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/search>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 搜索会议室请求
pub struct SearchRoomRequest {
    config: Config,
}

impl SearchRoomRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/search>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let api_endpoint = VcApiV1::RoomSearch;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "搜索会议室")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "搜索会议室")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../vc/v1/rooms/search → 裸 Value 解析（单层 resp["field"]）。
    #[tokio::test]
    async fn test_search_room_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/vc/v1/rooms/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "rooms": [
                        { "room_id": "room_001", "name": "大会议室" }
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

        let resp = SearchRoomRequest::new(config)
            .execute(json!({ "query": "大" }))
            .await
            .expect("搜索会议室应成功");
        assert_eq!(resp["rooms"][0]["room_id"], json!("room_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/rooms/search");
    }
}
