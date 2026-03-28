//! 创建访问控制
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-acl/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, validate_required,
    SDKResult,
};

use crate::common::api_endpoints::CalendarApiV4;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 创建访问控制请求
pub struct CreateCalendarAclRequest {
    config: Config,
    calendar_id: String,
}

impl CreateCalendarAclRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
        }
    }

    /// 日历 ID（路径参数）
    pub fn calendar_id(mut self, calendar_id: impl Into<String>) -> Self {
        self.calendar_id = calendar_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-acl/create>
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
        validate_required!(self.calendar_id, "calendar_id 不能为空");

        // url: POST:/open-apis/calendar/v4/calendars/:calendar_id/acls
        let endpoint = CalendarApiV4::CalendarAclCreate(self.calendar_id.clone());
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(endpoint.to_url()).body(serialize_params(&body, "创建访问控制")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建访问控制")
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
        let request = CreateCalendarAclRequest::new(config.clone()).calendar_id("test".to_string());
        let _ = request;
    }
}
