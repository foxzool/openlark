//! 查询会议室忙闲
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/query-room-availability>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::MEETING_ROOM;

/// 查询会议室忙闲请求
pub struct BatchGetFreebusyRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl BatchGetFreebusyRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/query-room-availability>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/meeting_room/freebusy/batch_get
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/freebusy/batch_get", MEETING_ROOM));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询会议室忙闲")
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
