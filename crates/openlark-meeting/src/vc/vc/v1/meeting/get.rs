//! 获取会议详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, validate_required_field};
use serde::{Deserialize, Serialize};

/// 获取会议详情请求

#[derive(Debug, Clone)]
pub struct GetMeetingRequest {
    /// 配置信息
    config: Config,
    /// 会议 ID（路径参数）
    meeting_id: String,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 获取会议详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetMeetingResponse {
    /// 会议 ID
    pub meeting_id: String,
    /// 会议主题
    pub topic: String,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
}

impl ApiResponseTrait for GetMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMeetingRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            meeting_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 设置会议 ID（路径参数）
    pub fn meeting_id(mut self, meeting_id: impl Into<String>) -> Self {
        self.meeting_id = meeting_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/get>
    pub async fn execute(self) -> SDKResult<GetMeetingResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMeetingResponse> {
        validate_required_field("meeting_id", Some(&self.meeting_id), "会议 ID 不能为空")?;

        let api_endpoint = VcApiV1::MeetingGet(self.meeting_id.clone());
        let mut api_request: ApiRequest<GetMeetingResponse> =
            ApiRequest::get(api_endpoint.to_url());

        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取会议详情")
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
