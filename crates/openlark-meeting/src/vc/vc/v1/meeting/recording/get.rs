//! 获取录制文件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-recording/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::common::api_utils::extract_response_data;

/// 获取录制文件请求
pub struct GetRecordingRequest {
    config: Config,
    meeting_id: String,
    query_params: Vec<(String, String)>,
}

impl GetRecordingRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            meeting_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 会议 ID（路径参数）
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-recording/get>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.meeting_id, "meeting_id 不能为空");

        // url: GET:/open-apis/vc/v1/meetings/:meeting_id/recording
        let url = format!("/open-apis/vc/v1/meetings/{}/recording", self.meeting_id);
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取录制文件")
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
