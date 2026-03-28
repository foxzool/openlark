//! 查询会议室配置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/query>

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

/// 查询会议室配置请求

#[derive(Debug, Clone)]
pub struct QueryRoomConfigRequest {
    /// 配置信息
    config: Config,
}

/// 查询会议室配置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryRoomConfigResponse {
    /// 会议室配置列表
    pub configs: Vec<RoomConfigItem>,
}

/// 会议室配置项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomConfigItem {
    /// 配置 ID
    pub config_id: String,
    /// 配置名称
    pub name: String,
}

impl ApiResponseTrait for QueryRoomConfigResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl QueryRoomConfigRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_config/query>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<QueryRoomConfigResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<QueryRoomConfigResponse> {
        let api_endpoint = VcApiV1::RoomConfigList;
        let api_request: ApiRequest<QueryRoomConfigResponse> =
            ApiRequest::get(api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "查询会议室配置")
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
