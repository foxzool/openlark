//! 创建请假日程
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/create>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_endpoints::CalendarApiV4,
    common::api_utils::{extract_response_data, serialize_params},
};

/// 创建请假日程请求
pub struct CreateTimeoffEventRequest {
    config: Config,
}

impl CreateTimeoffEventRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/create>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        let api_endpoint = CalendarApiV4::TimeoffEventCreate;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "创建请假日程")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建请假日程")
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
