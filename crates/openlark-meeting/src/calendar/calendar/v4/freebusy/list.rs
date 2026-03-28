//! 查询主日历日程忙闲信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/list>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use crate::common::api_utils::{extract_response_data, serialize_params};

/// 查询主日历日程忙闲信息请求
pub struct ListFreebusyRequest {
    config: Config,
}

impl ListFreebusyRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar/v4/calendar/list>
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
        // url: POST:/open-apis/calendar/v4/freebusy/list
        let url = "/open-apis/calendar/v4/freebusy/list";
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(url).body(serialize_params(&body, "查询主日历日程忙闲信息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询主日历日程忙闲信息")
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
        let request = ListFreebusyRequest::new(config.clone());
        let _ = request;
    }
}
