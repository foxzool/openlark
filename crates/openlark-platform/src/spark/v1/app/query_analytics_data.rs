//! 获取妙搭应用运营数据趋势
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app/query_analytics_data>
//!
//! 权限点 `spark:app:read`。Authorization 为 tenant 或 user（默认即可）。
//! `group_by` 文档笔误 `device_ytpe`，按 `device_type` 实现。

use crate::common::{api_endpoints::SparkApiV1, api_utils::serialize_params};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 时间聚合粒度。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SparkTimeAggregationUnit {
    /// 按天。
    Day,
    /// 按周。
    Week,
    /// 按月。
    Month,
}

/// 运营数据趋势过滤条件。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SparkAnalyticsFilter {
    /// 页面。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// 设备类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_types: Option<Vec<String>>,
}

/// 获取妙搭应用运营数据趋势请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryAnalyticsDataBody {
    /// 指标类型，长度 1-20。
    pub metric_types: Vec<String>,
    /// 开始时间（纳秒字符串）。
    pub start_timestamp_ns: String,
    /// 结束时间（纳秒字符串）。
    pub end_timestamp_ns: String,
    /// 时间聚合粒度。
    pub time_aggregation_unit: SparkTimeAggregationUnit,
    /// 过滤条件。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<SparkAnalyticsFilter>,
    /// 是否补点。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_pack_lack_point: Option<bool>,
    /// 分组维度。文档笔误 `device_ytpe`，按 `device_type`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
}

impl QueryAnalyticsDataBody {
    /// 校验请求体。
    pub fn validate(&self) -> SDKResult<()> {
        validate_required_list!(
            self.metric_types,
            20,
            "metric_types 不能为空且不能超过 20 个"
        );
        validate_required!(self.start_timestamp_ns, "start_timestamp_ns 不能为空");
        validate_required!(self.end_timestamp_ns, "end_timestamp_ns 不能为空");
        Ok(())
    }
}

/// 趋势维度。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SparkAnalyticsDimension {
    /// 维度名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 维度值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 趋势数据点。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SparkAnalyticsDataPoint {
    /// 时间戳（纳秒字符串）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_ns: Option<String>,
    /// 指标值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// 维度。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<SparkAnalyticsDimension>,
}

/// 趋势序列。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SparkAnalyticsSeries {
    /// 指标类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    /// 数据点。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub points: Vec<SparkAnalyticsDataPoint>,
}

/// 获取妙搭应用运营数据趋势响应 `data`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct QueryAnalyticsDataResponse {
    /// 指标序列。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub series: Vec<SparkAnalyticsSeries>,
}

impl ApiResponseTrait for QueryAnalyticsDataResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取妙搭应用运营数据趋势请求。
#[derive(Debug, Clone)]
pub struct QuerySparkAppAnalyticsDataRequest {
    config: Arc<Config>,
    app_id: String,
}

impl QuerySparkAppAnalyticsDataRequest {
    /// 创建请求构建器。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: QueryAnalyticsDataBody,
    ) -> SDKResult<QueryAnalyticsDataResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: QueryAnalyticsDataBody,
        option: RequestOption,
    ) -> SDKResult<QueryAnalyticsDataResponse> {
        validate_required!(self.app_id, "app_id 不能为空");
        body.validate()?;

        let req: ApiRequest<QueryAnalyticsDataResponse> =
            ApiRequest::post(SparkApiV1::AppsQueryAnalyticsData(self.app_id.clone()).to_url())
                .body(serialize_params(&body, "获取妙搭应用运营数据趋势")?);
        Transport::request_typed(req, &self.config, Some(option), "获取妙搭应用运营数据趋势").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, method, path},
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

    fn sample_body() -> QueryAnalyticsDataBody {
        QueryAnalyticsDataBody {
            metric_types: vec!["active_users".into()],
            start_timestamp_ns: "1700000000000000000".into(),
            end_timestamp_ns: "1700000100000000000".into(),
            time_aggregation_unit: SparkTimeAggregationUnit::Day,
            filter: Some(SparkAnalyticsFilter {
                page: Some("/home".into()),
                device_types: Some(vec!["mobile".into()]),
            }),
            need_pack_lack_point: Some(true),
            group_by: Some("device_type".into()),
        }
    }

    #[test]
    fn query_analytics_data_url() {
        assert_eq!(
            SparkApiV1::AppsQueryAnalyticsData("cli_x".into()).to_url(),
            "/open-apis/spark/v1/apps/cli_x/query_analytics_data"
        );
    }

    #[test]
    fn time_aggregation_unit_screaming_snake() {
        assert_eq!(
            serde_json::to_value(SparkTimeAggregationUnit::Day).unwrap(),
            json!("DAY")
        );
        assert_eq!(
            serde_json::to_value(SparkTimeAggregationUnit::Week).unwrap(),
            json!("WEEK")
        );
        assert_eq!(
            serde_json::to_value(SparkTimeAggregationUnit::Month).unwrap(),
            json!("MONTH")
        );
    }

    #[tokio::test]
    async fn query_analytics_data_rejects_empty_metric_types() {
        let server = MockServer::start().await;
        let mut body = sample_body();
        body.metric_types.clear();
        let result = QuerySparkAppAnalyticsDataRequest::new(test_config(server.uri()), "cli_x")
            .execute(body)
            .await;
        result.expect_err("空 metric_types 应失败");
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn query_analytics_data_posts_nested_filter() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/spark/v1/apps/cli_x/query_analytics_data"))
            .and(body_json(json!({
                "metric_types": ["active_users"],
                "start_timestamp_ns": "1700000000000000000",
                "end_timestamp_ns": "1700000100000000000",
                "time_aggregation_unit": "DAY",
                "filter": {
                    "page": "/home",
                    "device_types": ["mobile"]
                },
                "need_pack_lack_point": true,
                "group_by": "device_type"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "series": [{
                        "metric_type": "active_users",
                        "points": [{
                            "timestamp_ns": "1700000000000000000",
                            "value": 12.0,
                            "dimensions": [{"name": "device_type", "value": "mobile"}]
                        }]
                    }]
                }
            })))
            .mount(&server)
            .await;

        let resp = QuerySparkAppAnalyticsDataRequest::new(test_config(server.uri()), "cli_x")
            .execute_with_options(sample_body(), tenant_option())
            .await
            .expect("运营数据趋势应成功");
        assert_eq!(resp.series[0].metric_type.as_deref(), Some("active_users"));
        assert_eq!(
            resp.series[0].points[0].timestamp_ns.as_deref(),
            Some("1700000000000000000")
        );
    }
}
