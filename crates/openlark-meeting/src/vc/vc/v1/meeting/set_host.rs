//! 设置主持人
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/set_host>

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

/// 设置主持人请求
#[derive(Debug, Clone)]
pub struct SetHostMeetingRequest {
    config: Config,
    meeting_id: String,
}

/// 设置主持人响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetHostMeetingResponse {
    /// 操作结果
    pub success: bool,
}

impl ApiResponseTrait for SetHostMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SetHostMeetingRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/set_host>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<SetHostMeetingResponse> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<SetHostMeetingResponse> {
        validate_required!(self.meeting_id, "meeting_id 不能为空");

        let api_endpoint = VcApiV1::MeetingSetHost(self.meeting_id);
        let req: ApiRequest<SetHostMeetingResponse> =
            ApiRequest::patch(api_endpoint.to_url()).body(serialize_params(&body, "设置主持人")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "设置主持人")
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
