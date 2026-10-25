//! 获取工作日历日期详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/leave/work_calendar_date>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取工作日历日期详情请求
#[derive(Debug, Clone)]
pub struct WorkCalendarDateRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl WorkCalendarDateRequest {
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
    pub async fn execute(self) -> SDKResult<WorkCalendarDateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<WorkCalendarDateResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::LeaveWorkCalendarDate;
        let mut request = ApiRequest::<WorkCalendarDateResponse>::post(api_endpoint.to_url());
        for (key, value) in self.query_params {
            request = request.query(&key, value);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取工作日历日期详情响应数据为空",
        )
        .await
    }
}

/// 获取工作日历日期详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkCalendarDateResponse {
    /// 日期列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_dates: Option<Vec<WorkCalendarItem>>,
}

/// 工作日历条目（官方 wk_calendar_date 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WorkCalendarItem {
    /// 工作日历 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<String>,
    /// 日期，格式："2006-01-02"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 日期类型（day_off / public_holiday / workday）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_type: Option<String>,
    /// 日期 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

impl ApiResponseTrait for WorkCalendarDateResponse {
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

    /// 端到端：POST /open-apis/corehr/v1/leaves/work_calendar_date
    /// （官方 data 内层键为 calendar_dates，元素含 calendar_id / date / date_type / id）
    #[tokio::test]
    async fn test_work_calendar_date_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/leaves/work_calendar_date"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "calendar_dates": [
                        {
                            "calendar_id": "7390282135276635692",
                            "date": "2006-01-02",
                            "date_type": "day_off",
                            "id": "7390282135276635692"
                        }
                    ]
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = WorkCalendarDateRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let dates = resp.calendar_dates.expect("calendar_dates 应解码");
        assert_eq!(dates.len(), 1);
        assert_eq!(dates[0].calendar_id.as_deref(), Some("7390282135276635692"));
        assert_eq!(dates[0].date.as_deref(), Some("2006-01-02"));
        assert_eq!(dates[0].date_type.as_deref(), Some("day_off"));
        assert_eq!(dates[0].id.as_deref(), Some("7390282135276635692"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
