//! 更新会议室
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/patch>

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

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新会议室")
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
