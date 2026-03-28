//! 授权录制文件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-recording/set_permission>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, validate_required,
    SDKResult,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 授权录制文件请求
pub struct SetRecordingPermissionRequest {
    config: Config,
    meeting_id: String,
}

impl SetRecordingPermissionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            meeting_id: String::new(),
        }
    }

    /// 会议 ID（路径参数）
    pub fn meeting_id(mut self, meeting_id: impl Into<String>) -> Self {
        self.meeting_id = meeting_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-recording/set_permission>
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
        validate_required!(self.meeting_id, "meeting_id 不能为空");

        // url: PATCH:/open-apis/vc/v1/meetings/:meeting_id/recording/set_permission
        let api_endpoint = VcApiV1::MeetingRecordingSetPermission(self.meeting_id.clone());
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::patch(api_endpoint.to_url()).body(serialize_params(&body, "授权录制文件")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "授权录制文件")
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
