//! 获取工作日历日期详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/leave/work_calendar_date>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取工作日历日期详情请求
#[derive(Debug, Clone)]
pub struct WorkCalendarDateRequest {
    config: Config,
    body: WorkCalendarDateRequestBody,
}

/// 获取工作日历日期详情请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarDateRequestBody {
    /// 工作日历 ID 列表（必填，最多 100）
    pub wk_calendar_ids: Vec<String>,
    /// 日期列表，格式："2006-01-02"，最多 50 个；不传 ids 时必传 dates 或 [begin_date, end_date]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<Vec<String>>,
    /// 日期范围-开始日期，格式："2006-01-02"，需和 end_date 一起使用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub begin_date: Option<String>,
    /// 日期范围-结束日期(含)，格式："2006-01-02"，需和 begin_date 一起使用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 分页
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页大小，无默认值需手动设置
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// 日期 ID，可使用响应体中的 data.calendar_dates.id；传 ids 时 dates/[begin_date, end_date] 不生效
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

impl WorkCalendarDateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: WorkCalendarDateRequestBody {
                wk_calendar_ids: Vec::new(),
                dates: None,
                begin_date: None,
                end_date: None,
                offset: None,
                limit: None,
                ids: None,
            },
        }
    }

    /// 设置工作日历 ID 列表（必填，最多 100）
    pub fn wk_calendar_ids(mut self, wk_calendar_ids: Vec<String>) -> Self {
        self.body.wk_calendar_ids = wk_calendar_ids;
        self
    }

    /// 设置日期列表（最多 50 个）
    pub fn dates(mut self, dates: Vec<String>) -> Self {
        self.body.dates = Some(dates);
        self
    }

    /// 设置日期范围-开始日期（需和 end_date 一起使用）
    pub fn begin_date(mut self, begin_date: impl Into<String>) -> Self {
        self.body.begin_date = Some(begin_date.into());
        self
    }

    /// 设置日期范围-结束日期(含)（需和 begin_date 一起使用）
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.body.end_date = Some(end_date.into());
        self
    }

    /// 设置分页
    pub fn offset(mut self, offset: i32) -> Self {
        self.body.offset = Some(offset);
        self
    }

    /// 设置分页大小
    pub fn limit(mut self, limit: i32) -> Self {
        self.body.limit = Some(limit);
        self
    }

    /// 设置日期 ID 列表
    pub fn ids(mut self, ids: Vec<String>) -> Self {
        self.body.ids = Some(ids);
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

        validate_required_list!(
            self.body.wk_calendar_ids,
            100,
            "wk_calendar_ids 不能为空且最多 100 个"
        );

        let api_endpoint = FeishuPeopleApiV1::LeaveWorkCalendarDate;
        let request = ApiRequest::<WorkCalendarDateResponse>::post(api_endpoint.to_url())
            .body(serde_json::to_value(&self.body)?);
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
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/leaves/work_calendar_date
    /// （官方 data 内层键为 calendar_dates，元素含 calendar_id / date / date_type / id；
    /// 请求体必填 wk_calendar_ids）
    #[tokio::test]
    async fn test_work_calendar_date_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/leaves/work_calendar_date"))
            .and(body_json(json!({
                "wk_calendar_ids": ["7390282135276635692"],
                "dates": ["2006-01-02"]
            })))
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
            .wk_calendar_ids(vec!["7390282135276635692".to_string()])
            .dates(vec!["2006-01-02".to_string()])
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
