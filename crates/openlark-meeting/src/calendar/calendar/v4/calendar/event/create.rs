//! 创建日程
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::CALENDAR_V4_EVENT_CREATE;

/// 创建日程请求
#[derive(Debug, Clone)]
pub struct CreateCalendarEventRequest {
    config: Config,
    calendar_id: String,
}

/// 创建日程响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCalendarEventResponse {
    /// 日程数据
    pub data: serde_json::Value,
}

impl ApiResponseTrait for CreateCalendarEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateCalendarEventRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/create>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateCalendarEventResponse> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");

        let url = CALENDAR_V4_EVENT_CREATE.replace("{calendar_id}", &self.calendar_id);
        let req: ApiRequest<CreateCalendarEventResponse> =
            ApiRequest::post(&url).body(serialize_params(&body, "创建日程")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建日程")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder().app_id("test_app").app_secret("test_secret").build();
        let request = CreateCalendarEventRequest::new(config.clone())
            .calendar_id("test".to_string());
        let _ = request;
    }
}
