//! 设置会议室配置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/set>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use crate::common::api_utils::{extract_response_data, serialize_params};

/// 设置会议室配置请求
pub struct SetRoomConfigRequest {
    config: Config,
}

impl SetRoomConfigRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/set>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default(), body)
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
        body: serde_json::Value,
    ) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/room_configs/set
        let url = "/open-apis/vc/v1/room_configs/set";
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(url).body(serialize_params(&body, "设置会议室配置")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "设置会议室配置")
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
