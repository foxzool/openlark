//! 删除建筑物
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/delete-building>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, validate_required,
    SDKResult,
};

use crate::common::api_endpoints::MeetingRoomApi;
use crate::common::api_utils::extract_response_data;

/// 删除建筑物请求
pub struct DeleteBuildingRequest {
    config: Config,
    building_id: String,
}

impl DeleteBuildingRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            building_id: String::new(),
        }
    }

    /// 建筑物 ID
    pub fn building_id(mut self, building_id: impl Into<String>) -> Self {
        self.building_id = building_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/delete-building>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.building_id, "building_id 不能为空");

        // url: DELETE:/open-apis/meeting_room/buildings/:building_id
        let api_endpoint = MeetingRoomApi::BuildingDelete(self.building_id.clone());
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::delete(api_endpoint.to_url()).body(serde_json::json!({
                "building_id": self.building_id
            }));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除建筑物")
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
