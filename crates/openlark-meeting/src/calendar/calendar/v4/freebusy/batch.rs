//! 批量查询主日历日程忙闲信息
//!
//! docPath: <https://open.feishu.cn/document/calendar-v4/calendar/batch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required, validate_required_list,
};

use super::models::{BatchFreebusyRequestBody, BatchFreebusyResponse};
use crate::{common::api_endpoints::CalendarApiV4, common::api_utils::serialize_params};

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
    /// docPath: <https://open.feishu.cn/document/calendar-v4/calendar/batch>
    pub async fn execute(self, body: BatchFreebusyRequestBody) -> SDKResult<BatchFreebusyResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用自定义请求选项和请求体执行请求。
    pub async fn execute_with_options(
        self,
        body: BatchFreebusyRequestBody,
        option: RequestOption,
    ) -> SDKResult<BatchFreebusyResponse> {
        validate_required!(body.time_min, "time_min 不能为空");
        validate_required!(body.time_max, "time_max 不能为空");
        validate_required_list!(body.user_ids, 10, "user_ids 不能为空且不能超过 10 个");
        let api_endpoint = CalendarApiV4::FreebusyBatch;
        let req: ApiRequest<BatchFreebusyResponse> = ApiRequest::post(api_endpoint.to_url())
            .body(serialize_params(&body, "批量查询主日历日程忙闲信息")?);

        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "批量查询主日历日程忙闲信息",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../calendar/v4/freebusy/batch → BatchFreebusyResponse（data.freebusy_list）。
    #[tokio::test]
    async fn test_batch_freebusy_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/calendar/v4/freebusy/batch"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "freebusy_list": [
                        {
                            "user_id": "user_001",
                            "freebusy_list": [
                                { "start_time": 1000, "end_time": 2000, "status": 1 }
                            ]
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

        let body = BatchFreebusyRequestBody {
            time_min: "2020-10-28T12:00:00+08:00".to_string(),
            time_max: "2020-10-29T12:00:00+08:00".to_string(),
            user_ids: vec!["user_001".to_string()],
            include_external_calendar: None,
            only_busy: None,
            need_rsvp_status: None,
        };

        let resp = BatchFreebusyRequest::new(config)
            .execute(body)
            .await
            .expect("批量查询忙闲应成功");
        assert_eq!(resp.freebusy_list.len(), 1);
        assert_eq!(resp.freebusy_list[0].user_id, "user_001");
        assert_eq!(resp.freebusy_list[0].freebusy_list[0].status, 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/freebusy/batch"
        );
        assert_eq!(received[0].method, "POST");
        // 官方请求体字段：time_min/time_max/user_ids
        let sent: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(sent["time_min"], "2020-10-28T12:00:00+08:00");
        assert_eq!(sent["time_max"], "2020-10-29T12:00:00+08:00");
        assert_eq!(sent["user_ids"], serde_json::json!(["user_001"]));
    }
}
