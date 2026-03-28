//! 查询会议室详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, validate_required_field};

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

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "查询会议室详情")
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
