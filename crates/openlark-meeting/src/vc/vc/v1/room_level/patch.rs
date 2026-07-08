//! 更新会议室层级
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_ROOM_LEVELS;

/// 更新会议室层级请求
pub struct PatchRoomLevelRequest {
    config: Config,
    room_level_id: String,
}

impl PatchRoomLevelRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            room_level_id: String::new(),
        }
    }

    /// 会议室层级 ID（路径参数）
    pub fn room_level_id(mut self, room_level_id: impl Into<String>) -> Self {
        self.room_level_id = room_level_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/patch>
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
        validate_required!(self.room_level_id, "room_level_id 不能为空");

        // url: PATCH:/open-apis/vc/v1/room_levels/:room_level_id
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::patch(format!("{}/{}", VC_V1_ROOM_LEVELS, self.room_level_id))
                .body(serialize_params(&body, "更新会议室层级")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新会议室层级")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../vc/v1/room_levels/{room_level_id} → 裸 Value 解析（单层 resp["field"]）。
    #[tokio::test]
    async fn test_patch_room_level_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/vc/v1/room_levels/lvl_001"))
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

        let resp = PatchRoomLevelRequest::new(config)
            .room_level_id("lvl_001")
            .execute(json!({ "name": "更新层级" }))
            .await
            .expect("更新会议室层级应成功");
        assert_eq!(resp["success"], json!(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/room_levels/lvl_001"
        );
        assert_eq!(received[0].method, "PATCH");
    }
}
