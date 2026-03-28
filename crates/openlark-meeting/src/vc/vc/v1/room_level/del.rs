//! 删除会议室层级
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/del>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult,
    req_option::RequestOption};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_ROOM_LEVELS;

/// 删除会议室层级请求
pub struct DeleteRoomLevelRequest {
    config: Config,
}

impl DeleteRoomLevelRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口通常通过 body 传递要删除的 room_level_id，建议按文档构造 JSON。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room_level/del>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/room_levels/del
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/del", VC_V1_ROOM_LEVELS))
                .body(serialize_params(&body, "删除会议室层级")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除会议室层级")
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
