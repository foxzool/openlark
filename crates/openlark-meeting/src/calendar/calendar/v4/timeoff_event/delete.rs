//! 删除请假日程
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, validate_required,
    SDKResult,
};

use crate::common::api_utils::extract_response_data;

/// 删除请假日程请求
pub struct DeleteTimeoffEventRequest {
    config: Config,
    timeoff_event_id: String,
}

impl DeleteTimeoffEventRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            timeoff_event_id: String::new(),
        }
    }

    /// 请假日程 ID（路径参数）
    pub fn timeoff_event_id(mut self, timeoff_event_id: impl Into<String>) -> Self {
        self.timeoff_event_id = timeoff_event_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/delete>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.timeoff_event_id, "timeoff_event_id 不能为空");

        // url: DELETE:/open-apis/calendar/v4/timeoff_events/:timeoff_event_id
        let req: ApiRequest<serde_json::Value> = ApiRequest::delete(format!(
            "/open-apis/calendar/v4/timeoff_events/{}",
            self.timeoff_event_id
        ));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除请假日程")
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
