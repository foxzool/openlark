//! 创建请假日程
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::{common::api_endpoints::CalendarApiV4, common::api_utils::serialize_params};

/// 创建请假日程请求
pub struct CreateTimeoffEventRequest {
    config: Config,
}

impl CreateTimeoffEventRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/create>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let api_endpoint = CalendarApiV4::TimeoffEventCreate;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "创建请假日程")?);

        Transport::request_typed(req, &self.config, Some(option), "创建请假日程").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CreateTimeoffEventRequest::new(config.clone());
        let _ = request;
    }
}
