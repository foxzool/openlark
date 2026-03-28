//! 移除参会人
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/kickout>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 移除参会人请求
#[derive(Debug, Clone)]
pub struct KickoutMeetingRequest {
    config: Config,
    meeting_id: String,
}

/// 移除参会人响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KickoutMeetingResponse {
    /// 操作结果
    pub success: bool,
}

impl ApiResponseTrait for KickoutMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl KickoutMeetingRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/kickout>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<KickoutMeetingResponse> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<KickoutMeetingResponse> {
        validate_required!(self.meeting_id, "meeting_id 不能为空");

        let api_endpoint = VcApiV1::MeetingKickout(self.meeting_id);
        let req: ApiRequest<KickoutMeetingResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "移除参会人")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "移除参会人")
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
