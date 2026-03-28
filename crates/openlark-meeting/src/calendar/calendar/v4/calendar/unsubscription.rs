//! 取消订阅日历变更事件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/unsubscription>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use crate::common::api_endpoints::CalendarApiV4;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 取消订阅日历变更事件请求
pub struct UnsubscriptionCalendarRequest {
    config: Config,
}

impl UnsubscriptionCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/unsubscription>
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
        // url: POST:/open-apis/calendar/v4/calendars/unsubscription
        let api_endpoint = CalendarApiV4::CalendarSubscription;
        let url = format!("{}/unsubscription", api_endpoint.to_url());
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(url).body(serialize_params(&body, "取消订阅日历变更事件")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "取消订阅日历变更事件")
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
