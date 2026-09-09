//! 获取妙搭应用消耗 AI 额度
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app/open_api_credit_usage>
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

/// 额度消耗时间点。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SparkCreditUsagePoint {
    /// 时间戳，秒级字符串。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 总额度消耗。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_usage: Option<f64>,
    /// 企业额度消耗。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_usage_enterprise: Option<f64>,
    /// 个人额度消耗。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_usage_personal: Option<f64>,
}

/// 获取妙搭应用消耗 AI 额度响应 `data`。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GetSparkAppCreditUsageResponse {
    /// 总额度。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    /// 分时消耗。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub points: Vec<SparkCreditUsagePoint>,
    /// 企业额度合计。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_enterprise: Option<f64>,
    /// 个人额度合计。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_personal: Option<f64>,
}

impl ApiResponseTrait for GetSparkAppCreditUsageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取妙搭应用消耗 AI 额度请求。
#[derive(Debug, Clone)]
pub struct GetSparkAppCreditUsageRequest {
    config: Arc<Config>,
    app_id: String,
    start_time: Option<String>,
    end_time: Option<String>,
}

impl GetSparkAppCreditUsageRequest {
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
    pub async fn execute(self) -> SDKResult<GetSparkAppCreditUsageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetSparkAppCreditUsageResponse> {
        validate_required!(self.app_id, "app_id 不能为空");
        let start_time = self.start_time.unwrap_or_default();
        let end_time = self.end_time.unwrap_or_default();
        validate_required!(start_time, "start_time 不能为空");
        validate_required!(end_time, "end_time 不能为空");

        let req: ApiRequest<GetSparkAppCreditUsageResponse> =
            ApiRequest::get(SparkApiV1::AppsCreditUsage(self.app_id.clone()).to_url())
                .query("start_time", start_time)
                .query("end_time", end_time);
        Transport::request_typed(req, &self.config, Some(option), "获取妙搭应用消耗 AI 额度").await
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
    fn credit_usage_url() {
        assert_eq!(
            SparkApiV1::AppsCreditUsage("cli_x".into()).to_url(),
            "/open-apis/spark/v1/apps/cli_x/credit_usage"
        );
    }

    #[tokio::test]
    async fn credit_usage_gets_typed_points() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/spark/v1/apps/cli_x/credit_usage"))
            .and(query_param("start_time", "1700000000"))
            .and(query_param("end_time", "1700000100"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "total": 12.5,
                    "total_enterprise": 10.0,
                    "total_personal": 2.5,
                    "points": [{
                        "timestamp": "1700000000",
                        "credit_usage": 1.5,
                        "credit_usage_enterprise": 1.0,
                        "credit_usage_personal": 0.5
                    }]
                }
            })))
            .mount(&server)
            .await;

        let resp = GetSparkAppCreditUsageRequest::new(test_config(server.uri()), "cli_x")
            .start_time("1700000000")
            .end_time("1700000100")
            .execute_with_options(tenant_option())
            .await
            .expect("额度查询应成功");
        assert_eq!(resp.total, Some(12.5));
        assert_eq!(resp.total_enterprise, Some(10.0));
        assert_eq!(resp.points[0].timestamp.as_deref(), Some("1700000000"));
    }
}
