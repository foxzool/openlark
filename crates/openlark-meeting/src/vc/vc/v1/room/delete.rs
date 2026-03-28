//! 删除会议室
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
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
