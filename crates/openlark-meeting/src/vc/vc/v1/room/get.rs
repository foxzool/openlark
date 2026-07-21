//! 查询会议室详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::validate_required_field;

/// 查询会议室详情请求

#[derive(Debug, Clone)]
pub struct GetRoomRequest {
    /// 配置信息
    config: Config,
    /// 会议室 ID（路径参数）
    room_id: String,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 查询会议室详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRoomResponse {
    /// 会议室 ID
    pub room_id: String,
    /// 会议室名称
    pub name: String,
    /// 会议室层级 ID
    pub room_level_id: String,
    /// 会议室容量
    pub capacity: i32,
    /// 建筑物 ID
    pub building_id: String,
    /// 楼层
    pub floor: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 会议室状态
    pub status: String,
    /// 是否启用
    pub active: bool,
    /// 是否需要审批
    pub approval_required: bool,
}

impl ApiResponseTrait for GetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetRoomRequest {
    /// 创建新的查询请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            room_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 设置会议室 ID（路径参数）
    pub fn room_id(mut self, room_id: impl Into<String>) -> Self {
        self.room_id = room_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/get>
    pub async fn execute(self) -> SDKResult<GetRoomResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<GetRoomResponse> {
        validate_required_field("room_id", Some(&self.room_id), "会议室 ID 不能为空")?;

        let api_endpoint = VcApiV1::RoomGet(self.room_id.clone());
        let mut api_request: ApiRequest<GetRoomResponse> = ApiRequest::get(api_endpoint.to_url());

        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        Transport::request_typed(api_request, &self.config, Some(option), "查询会议室详情").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../vc/v1/rooms/{room_id} → 强类型 GetRoomResponse（无 inner data，单层 resp.field）。
    #[tokio::test]
    async fn test_get_room_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/rooms/room_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "room_id": "room_001",
                    "name": "大会议室",
                    "room_level_id": "lvl_1",
                    "capacity": 12,
                    "building_id": "bldg_1",
                    "floor": "3F",
                    "email": "room_001@example.com",
                    "status": "available",
                    "active": true,
                    "approval_required": false
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

        let resp = GetRoomRequest::new(config)
            .room_id("room_001")
            .execute()
            .await
            .expect("查询会议室应成功");
        assert_eq!(resp.room_id, "room_001");
        assert_eq!(resp.name, "大会议室");
        assert_eq!(resp.capacity, 12);
        assert!(resp.active);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/rooms/room_001");
    }
}
