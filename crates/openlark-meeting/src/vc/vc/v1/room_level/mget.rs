//! 批量查询会议室层级详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/mget>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_ROOM_LEVELS;

/// 批量查询会议室层级详情请求
pub struct MgetRoomLevelRequest {
    config: Config,
}

impl MgetRoomLevelRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/mget>
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
        // url: POST:/open-apis/vc/v1/room_levels/mget
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/mget", VC_V1_ROOM_LEVELS))
                .body(serialize_params(&body, "批量查询会议室层级详情")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "批量查询会议室层级详情")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../vc/v1/room_levels/mget → 裸 Value 解析（单层 resp["field"]）。
    #[tokio::test]
    async fn test_mget_room_level_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/vc/v1/room_levels/mget"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "room_levels": [
                        { "room_level_id": "lvl_001", "name": "一楼层级" }
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

        let resp = MgetRoomLevelRequest::new(config)
            .execute(json!({ "room_level_id_list": ["lvl_001"] }))
            .await
            .expect("批量查询会议室层级应成功");
        assert_eq!(resp["room_levels"][0]["room_level_id"], json!("lvl_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/room_levels/mget");
    }
}
