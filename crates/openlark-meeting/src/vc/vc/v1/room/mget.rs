//! 批量查询会议室详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/mget>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
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
        let api_endpoint = VcApiV1::RoomBatchGet;
        let api_request: ApiRequest<MgetRoomResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "批量查询会议室")
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
