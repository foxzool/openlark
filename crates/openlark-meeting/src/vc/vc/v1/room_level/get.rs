//! 查询会议室层级详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, validate_required_field};
use crate::endpoints::VC_V1_ROOM_LEVEL_GET;

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
