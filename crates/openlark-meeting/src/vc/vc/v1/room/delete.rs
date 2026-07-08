//! 删除会议室
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, validate_required_field};
use serde::{Deserialize, Serialize};

/// 删除会议室请求

#[derive(Debug, Clone)]
pub struct DeleteRoomRequest {
    /// 配置信息
    config: Config,
    /// 会议室 ID（路径参数）
    room_id: String,
}

/// 删除会议室响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteRoomResponse {
    /// 删除状态
    pub success: bool,
}

impl ApiResponseTrait for DeleteRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteRoomRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/delete>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required_field("room_id", Some(&self.room_id), "会议室 ID 不能为空")?;

        let api_endpoint = VcApiV1::RoomDelete(self.room_id.clone());
        let api_request: ApiRequest<serde_json::Value> = ApiRequest::delete(api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除会议室")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../vc/v1/rooms/{room_id} → 裸 Value 解析（单层 resp["field"]）。
    #[tokio::test]
    async fn test_delete_room_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
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

        let resp = DeleteRoomRequest::new(config)
            .room_id("room_001")
            .execute()
            .await
            .expect("删除会议室应成功");
        assert_eq!(resp["success"], json!(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/rooms/room_001");
        assert_eq!(received[0].method, "DELETE");
    }
}
