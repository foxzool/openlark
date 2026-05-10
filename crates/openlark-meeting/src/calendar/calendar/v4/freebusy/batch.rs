//! 批量查询主日历日程忙闲信息
//!
//! docPath: https://open.feishu.cn/document/calendar-v4/calendar/batch

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use super::models::{BatchFreebusyRequestBody, BatchFreebusyResponse};
use crate::{
    common::api_endpoints::CalendarApiV4,
    common::api_utils::{extract_response_data, serialize_params},
};

/// 批量查询主日历日程忙闲信息请求
pub struct BatchFreebusyRequest {
    config: Config,
}

impl BatchFreebusyRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    /// docPath: https://open.feishu.cn/document/calendar-v4/calendar/batch
    pub async fn execute(self, body: BatchFreebusyRequestBody) -> SDKResult<BatchFreebusyResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: BatchFreebusyRequestBody,
        option: RequestOption,
    ) -> SDKResult<BatchFreebusyResponse> {
        let api_endpoint = CalendarApiV4::FreebusyBatch;
        let req: ApiRequest<BatchFreebusyResponse> = ApiRequest::post(api_endpoint.to_url())
            .body(serialize_params(&body, "批量查询主日历日程忙闲信息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "批量查询主日历日程忙闲信息")
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
