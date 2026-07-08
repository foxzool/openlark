//! 删除会议室层级
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/del>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_ROOM_LEVELS;

/// 删除会议室层级请求
pub struct DeleteRoomLevelRequest {
    config: Config,
}

impl DeleteRoomLevelRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口通常通过 body 传递要删除的 room_level_id，建议按文档构造 JSON。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/del>
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
        // url: POST:/open-apis/vc/v1/room_levels/del
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/del", VC_V1_ROOM_LEVELS))
                .body(serialize_params(&body, "删除会议室层级")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除会议室层级")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../vc/v1/room_levels/del → 裸 Value 解析（单层 resp["field"]）。
    #[tokio::test]
    async fn test_delete_room_level_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/vc/v1/room_levels/del"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "success": true }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = DeleteRoomLevelRequest::new(config)
            .execute(json!({ "room_level_id": "lvl_001" }))
            .await
            .expect("删除会议室层级应成功");
        assert_eq!(resp["success"], json!(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/room_levels/del");
    }
}
