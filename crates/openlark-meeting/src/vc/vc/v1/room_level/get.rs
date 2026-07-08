//! 查询会议室层级详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, validate_required_field};

/// 查询会议室层级详情请求

#[derive(Debug, Clone)]
pub struct GetRoomLevelRequest {
    /// 配置信息
    config: Config,
    /// 会议室层级 ID（路径参数）
    room_level_id: String,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 查询会议室层级详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRoomLevelResponse {
    /// 会议室层级 ID
    pub room_level_id: String,
    /// 层级名称
    pub name: String,
    /// 容量范围
    pub capacity_min: Option<i32>,
    /// 最大容量。
    pub capacity_max: Option<i32>,
}

impl ApiResponseTrait for GetRoomLevelResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetRoomLevelRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            room_level_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 设置会议室层级 ID（路径参数）
    pub fn room_level_id(mut self, room_level_id: impl Into<String>) -> Self {
        self.room_level_id = room_level_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/get>
    pub async fn execute(self) -> SDKResult<GetRoomLevelResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetRoomLevelResponse> {
        validate_required_field(
            "room_level_id",
            Some(&self.room_level_id),
            "会议室层级 ID 不能为空",
        )?;

        let api_endpoint = VcApiV1::RoomLevelGet(self.room_level_id.clone());
        let mut api_request: ApiRequest<GetRoomLevelResponse> =
            ApiRequest::get(api_endpoint.to_url());

        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "查询会议室层级详情")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../vc/v1/room_levels/{room_level_id} → 强类型 GetRoomLevelResponse（单层 resp.field）+ query 断言。
    #[tokio::test]
    async fn test_get_room_level_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/room_levels/lvl_001"))
            .and(query_param("room_level_type", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "room_level_id": "lvl_001",
                    "name": "一楼层级",
                    "capacity_min": 4,
                    "capacity_max": 20
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

        let resp = GetRoomLevelRequest::new(config)
            .room_level_id("lvl_001")
            .query_param("room_level_type", "1")
            .execute()
            .await
            .expect("查询会议室层级应成功");
        assert_eq!(resp.room_level_id, "lvl_001");
        assert_eq!(resp.name, "一楼层级");
        assert_eq!(resp.capacity_max, Some(20));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/room_levels/lvl_001"
        );
    }
}
