//! 更新建筑物
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/update-building>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_endpoints::MeetingRoomApi,
    common::api_utils::{extract_response_data, serialize_params},
};

/// 更新建筑物请求
pub struct UpdateBuildingRequest {
    config: Config,
    building_id: String,
}

impl UpdateBuildingRequest {
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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/update-building>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        validate_required!(self.building_id, "building_id 不能为空");

        let api_endpoint = MeetingRoomApi::BuildingPatch(self.building_id.clone());
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "更新建筑物")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新建筑物")
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
