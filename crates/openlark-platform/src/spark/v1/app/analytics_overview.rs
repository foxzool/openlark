//! 获取妙搭应用运营数据总览
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app/open_api_analytics_overview>
//!
//! 权限点 `spark:app:read`。Authorization 为 tenant 或 user（默认即可）。

use crate::common::api_endpoints::SparkApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 运营指标（活跃用户 / 注册 / 页面浏览）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkAnalyticsMetric {
    /// 当前值（字符串数字）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 环比对照值（字符串数字）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_value: Option<String>,
    /// 差值（字符串数字）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
    /// 环比。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<f64>,
}

/// 获取妙搭应用运营数据总览响应 `data`。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GetSparkAppAnalyticsOverviewResponse {
    /// 活跃用户。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_users: Option<SparkAnalyticsMetric>,
    /// 注册用户。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signups: Option<SparkAnalyticsMetric>,
    /// 页面浏览。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_views: Option<SparkAnalyticsMetric>,
}

impl ApiResponseTrait for GetSparkAppAnalyticsOverviewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取妙搭应用运营数据总览请求。
#[derive(Debug, Clone)]
pub struct GetSparkAppAnalyticsOverviewRequest {
    config: Arc<Config>,
    app_id: String,
    start_time: Option<String>,
    end_time: Option<String>,
}

impl GetSparkAppAnalyticsOverviewRequest {
    /// 创建请求构建器。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            start_time: None,
            end_time: None,
        }
    }

    /// 设置开始时间（秒级字符串，必填）。
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }

    /// 设置结束时间（秒级字符串，必填）。
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetSparkAppAnalyticsOverviewResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetSparkAppAnalyticsOverviewResponse> {
        validate_required!(self.app_id, "app_id 不能为空");
        let start_time = self.start_time.unwrap_or_default();
        let end_time = self.end_time.unwrap_or_default();
        validate_required!(start_time, "start_time 不能为空");
        validate_required!(end_time, "end_time 不能为空");

        let req: ApiRequest<GetSparkAppAnalyticsOverviewResponse> =
            ApiRequest::get(SparkApiV1::AppsAnalyticsOverview(self.app_id.clone()).to_url())
                .query("start_time", start_time)
                .query("end_time", end_time);
        Transport::request_typed(req, &self.config, Some(option), "获取妙搭应用运营数据总览").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path, query_param},
    };

    fn test_config(base_url: impl Into<String>) -> Arc<Config> {
        Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(base_url)
                .enable_token_cache(false)
                .build(),
        )
    }

    fn tenant_option() -> RequestOption {
        RequestOption::builder()
            .tenant_access_token("tenant-token")
            .build()
    }

    #[test]
    fn analytics_overview_url() {
        assert_eq!(
            SparkApiV1::AppsAnalyticsOverview("cli_x".into()).to_url(),
            "/open-apis/spark/v1/apps/cli_x/analytics/overview"
        );
    }

    #[tokio::test]
    async fn analytics_overview_rejects_missing_start_time() {
        let server = MockServer::start().await;
        let result = GetSparkAppAnalyticsOverviewRequest::new(test_config(server.uri()), "cli_x")
            .end_time("1700000100")
            .execute()
            .await;
        result.expect_err("缺 start_time 应失败");
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn analytics_overview_gets_typed_metrics() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/spark/v1/apps/cli_x/analytics/overview"))
            .and(query_param("start_time", "1700000000"))
            .and(query_param("end_time", "1700000100"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "active_users": {
                        "value": "10",
                        "prev_value": "8",
                        "diff": "2",
                        "ratio": 0.25
                    },
                    "signups": {
                        "value": "3",
                        "prev_value": "1",
                        "diff": "2",
                        "ratio": 2.0
                    },
                    "page_views": {
                        "value": "100",
                        "prev_value": "80",
                        "diff": "20",
                        "ratio": 0.25
                    }
                }
            })))
            .mount(&server)
            .await;

        let resp = GetSparkAppAnalyticsOverviewRequest::new(test_config(server.uri()), "cli_x")
            .start_time("1700000000")
            .end_time("1700000100")
            .execute_with_options(tenant_option())
            .await
            .expect("运营数据总览应成功");
        let active = resp.active_users.expect("应包含 active_users");
        assert_eq!(active.value.as_deref(), Some("10"));
        assert_eq!(active.ratio, Some(0.25));
        assert!(resp.signups.is_some());
        assert!(resp.page_views.is_some());
    }
}
