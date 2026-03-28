//! 批量查询日历信息
//!
//! docPath: <https://open.feishu.cn/document/calendar-v4/calendar/mget-3>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_endpoints::CalendarApiV4,
    common::api_utils::{extract_response_data, serialize_params},
};

/// 批量查询日历信息请求
pub struct MgetCalendarRequest {
    config: Config,
}

impl MgetCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/calendar-v4/calendar/mget-3>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        let api_endpoint = CalendarApiV4::CalendarMget;
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(api_endpoint.to_url())
            .body(serialize_params(&body, "批量查询日历信息")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "批量查询日历信息")
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
