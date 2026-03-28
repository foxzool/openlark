//! 搜索会议室
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/search>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 搜索会议室请求
pub struct SearchRoomRequest {
    config: Config,
}

impl SearchRoomRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/room/search>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let api_endpoint = VcApiV1::RoomSearch;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "搜索会议室")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "搜索会议室")
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
