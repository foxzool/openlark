//! 更新会议室
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::validate_required_field;
use serde::{Deserialize, Serialize};

/// 更新会议室请求

#[derive(Debug, Clone)]
pub struct PatchRoomRequest {
    /// 配置信息
    config: Config,
    /// 会议室 ID（路径参数）
    room_id: String,
}

/// 更新会议室响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchRoomResponse {
    /// 更新状态
    pub success: bool,
}

impl ApiResponseTrait for PatchRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchRoomRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            room_id: String::new(),
        }
    }

    /// 设置会议室 ID（路径参数）
    pub fn room_id(mut self, room_id: impl Into<String>) -> Self {
        self.room_id = room_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/patch>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<PatchRoomResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<PatchRoomResponse> {
        validate_required_field("room_id", Some(&self.room_id), "会议室 ID 不能为空")?;

        let api_endpoint = VcApiV1::RoomPatch(self.room_id.clone());
        let api_request: ApiRequest<PatchRoomResponse> =
            ApiRequest::patch(api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "更新会议室").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../vc/v1/rooms/{room_id} → 强类型 PatchRoomResponse（无 inner data，单层 resp.success）。
    #[tokio::test]
    async fn test_patch_room_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/vc/v1/rooms/room_001"))
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

        let resp = PatchRoomRequest::new(config)
            .room_id("room_001")
            .execute(json!({ "name": "更新会议室" }))
            .await
            .expect("更新会议室应成功");
        assert!(resp.success);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/rooms/room_001");
        assert_eq!(received[0].method, "PATCH");
    }
}
