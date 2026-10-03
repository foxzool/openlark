//! 获取工作日历
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/leave/work_calendar>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取工作日历请求
#[derive(Debug, Clone)]
pub struct WorkCalendarRequest {
    /// 配置信息
    config: Config,
    query_params: Vec<(String, String)>,
}

impl WorkCalendarRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数。
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<WorkCalendarResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<WorkCalendarResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::LeaveWorkCalendar;
        let mut request = ApiRequest::<WorkCalendarResponse>::post(api_endpoint.to_url());
        for (key, value) in self.query_params {
            request = request.query(&key, value);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取工作日历响应数据为空",
        )
        .await
    }
}

/// 获取工作日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkCalendarResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<WorkCalendarInfo>,
}

/// 工作日历信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WorkCalendarInfo {
    /// 工作日历条目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<WorkCalendarItem>>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 工作日历条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WorkCalendarItem {
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

impl ApiResponseTrait for WorkCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/leaves/work_calendar
    #[tokio::test]
    async fn test_work_calendar_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/leaves/work_calendar"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        WorkCalendarRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
