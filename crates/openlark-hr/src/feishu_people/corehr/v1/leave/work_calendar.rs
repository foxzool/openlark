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
    body: WorkCalendarRequestBody,
}

/// 获取工作日历请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCalendarRequestBody {
    /// 工作日历 ID 列表（必填，第一次传入时可以传入空数组）
    pub wk_calendar_ids: Vec<String>,
    /// 工作日历 ID 大于
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_calendar_id_gt: Option<String>,
    /// 分页、排序等选项，如未填写将赋默认值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_option: Option<WorkCalendarOption>,
    /// 是否只返回启用的工作日历（默认 true）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only_enable: Option<bool>,
}

/// 工作日历查询选项（官方 wk_option 结构）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkCalendarOption {
    /// 是否返回符合条件的工作日历总数（默认 true）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<bool>,
    /// 分页查询的位移，从 0 开始（默认 0）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页查询单次查询数量（默认 2000，取值范围 1～2000）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// 排序选项（最多 50 个）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_options: Option<Vec<WorkCalendarSortOption>>,
}

/// 工作日历排序选项（官方 sort_option 结构）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkCalendarSortOption {
    /// 排序字段（如 wk_id）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    /// 排序顺序：0=升序，1=降序
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
    /// 排序国际化：0=中文关键字，1=英文关键字，2=拼音
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_i18n: Option<i32>,
    /// 是否按某个字段的层级深度排序
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_by_strand_length: Option<bool>,
    /// 是否按照拼音排序
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_by_pinyin: Option<bool>,
    /// 是否按照枚举类型 value_order 排序
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_by_enum_value_order: Option<bool>,
}

impl WorkCalendarRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: WorkCalendarRequestBody {
                wk_calendar_ids: Vec::new(),
                wk_calendar_id_gt: None,
                wk_option: None,
                only_enable: None,
            },
        }
    }

    /// 设置工作日历 ID 列表（必填，第一次传入时可以传入空数组）
    pub fn wk_calendar_ids(mut self, wk_calendar_ids: Vec<String>) -> Self {
        self.body.wk_calendar_ids = wk_calendar_ids;
        self
    }

    /// 设置工作日历 ID 大于
    pub fn wk_calendar_id_gt(mut self, wk_calendar_id_gt: impl Into<String>) -> Self {
        self.body.wk_calendar_id_gt = Some(wk_calendar_id_gt.into());
        self
    }

    /// 设置分页、排序等选项
    pub fn wk_option(mut self, wk_option: WorkCalendarOption) -> Self {
        self.body.wk_option = Some(wk_option);
        self
    }

    /// 设置是否只返回启用的工作日历
    pub fn only_enable(mut self, only_enable: bool) -> Self {
        self.body.only_enable = Some(only_enable);
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

        // 官方：wk_calendar_ids 必填，但第一次查询允许传空数组，故不做非空校验
        let api_endpoint = FeishuPeopleApiV1::LeaveWorkCalendar;
        let request = ApiRequest::<WorkCalendarResponse>::post(api_endpoint.to_url())
            .body(serde_json::to_value(&self.body)?);
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
    /// 工作日历列表，对应入参的 wk_calendar_ids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_calendars: Option<Vec<WorkCalendarDetail>>,
    /// 入参 count=true 时返回符合条件的工作日历总数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

/// 工作日历详情（官方 work_calendar_detail 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WorkCalendarDetail {
    /// 工作日历 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<String>,
    /// 工作日历名称（官方 wk_calendar_i18n：{zh_cn, en_us}）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_name: Option<crate::common::shared_models::I18nText>,
    /// 工作日历是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
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
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/leaves/work_calendar
    /// （官方 data 内层键为 work_calendars / count；请求体必填 wk_calendar_ids）
    #[tokio::test]
    async fn test_work_calendar_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/leaves/work_calendar"))
            .and(body_json(json!({
                "wk_calendar_ids": ["7136914415957591596"],
                "wk_option": {
                    "count": true,
                    "limit": 20
                },
                "only_enable": true
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "work_calendars": [
                        {
                            "calendar_id": "7136914415957591596",
                            "calendar_name": {
                                "zh_cn": "日历1",
                                "en_us": "calendar1"
                            },
                            "enable": true
                        }
                    ],
                    "count": 20
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

        let resp = WorkCalendarRequest::new(config)
            .wk_calendar_ids(vec!["7136914415957591596".to_string()])
            .wk_option(WorkCalendarOption {
                count: Some(true),
                limit: Some(20),
                ..Default::default()
            })
            .only_enable(true)
            .execute()
            .await
            .expect("请求应成功");

        let calendars = resp.work_calendars.expect("work_calendars 应解码");
        assert_eq!(calendars.len(), 1);
        assert_eq!(
            calendars[0].calendar_id.as_deref(),
            Some("7136914415957591596")
        );
        assert_eq!(
            calendars[0]
                .calendar_name
                .as_ref()
                .and_then(|n| n.zh_cn.clone()),
            Some("日历1".to_string())
        );
        assert_eq!(calendars[0].enable, Some(true));
        assert_eq!(resp.count, Some(20));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
