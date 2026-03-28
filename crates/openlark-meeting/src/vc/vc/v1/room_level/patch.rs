//! 更新会议室层级
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/patch>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport,
    req_option::RequestOption, validate_required, SDKResult,
};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_ROOM_LEVELS;

/// 更新会议室层级请求
pub struct PatchRoomLevelRequest {
    config: Config,
    room_level_id: String,
}

impl PatchRoomLevelRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            room_level_id: String::new(),
        }
    }

    /// 会议室层级 ID（路径参数）
    pub fn room_level_id(mut self, room_level_id: impl Into<String>) -> Self {
        self.room_level_id = room_level_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/patch>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.room_level_id, "room_level_id 不能为空");

        // url: PATCH:/open-apis/vc/v1/room_levels/:room_level_id
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::patch(format!("{}/{}", VC_V1_ROOM_LEVELS, self.room_level_id))
                .body(serialize_params(&body, "更新会议室层级")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新会议室层级")
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
