//! 获取被评估人关键指标结果
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/metric_detail/query>

use crate::common::shared_models::I18nText;
use crate::performance::performance::v2::models::UserId;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use openlark_core::{validate_required, validate_required_list};
use serde::{Deserialize, Serialize};

/// 获取被评估人关键指标结果请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 周期 ID（必填，1 次只允许查询 1 个周期）
    semester_id: String,
    /// 被评估人 ID 列表（必填，1～50，与入参 user_id_type 类型一致）
    reviewee_user_ids: Vec<String>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config, semester_id: String) -> Self {
        Self {
            semester_id,
            reviewee_user_ids: Vec::new(),
            user_id_type: None,
            config,
        }
    }

    /// 追加一个被评估人 ID
    pub fn add_reviewee_user_id(mut self, reviewee_user_id: String) -> Self {
        self.reviewee_user_ids.push(reviewee_user_id);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        validate_required!(self.semester_id.trim(), "semester_id");
        validate_required_list!(
            self.reviewee_user_ids,
            50,
            "reviewee_user_ids 不能为空且不能超过 50 个"
        );

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::MetricDetailQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type);

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            semester_id: self.semester_id,
            reviewee_user_ids: self.reviewee_user_ids,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取关键指标结果响应数据为空",
        )
        .await
    }
}

/// 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 周期 ID
    pub semester_id: String,
    /// 被评估人 ID 列表
    pub reviewee_user_ids: Vec<String>,
}

/// 获取被评估人关键指标结果响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    /// 周期 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    /// 指标明细列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewee_metrics: Option<Vec<RevieweeMetric>>,
}

/// 被评估人指标明细（官方 reviewee_metric 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RevieweeMetric {
    /// 被评估人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewee_user_id: Option<UserId>,
    /// 指标模板 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_template_id: Option<String>,
    /// 指标详情
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_details: Option<MetricDetail>,
    /// 指标对应的环节状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewee_stage_statuses: Option<Vec<RevieweeStageStatus>>,
}

/// 指标详情（官方 metric_detail 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricDetail {
    /// 指标 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    /// 指标名称（指标在该明细数据中的名称）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 指标字段信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<MetricFieldInDetail>>,
    /// 指标维度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_id: Option<String>,
    /// 指标维度名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<I18nText>,
    /// 指标维度权重（未设置时为空，单位为百分比）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_weight: Option<String>,
    /// 指标添加来源：reviewee 指标制定人添加 / admin 管理员添加
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_from: Option<String>,
    /// 指标是否引自指标库
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_from_library: Option<bool>,
}

/// 明细中的指标字段（官方 metric_field_in_details 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricFieldInDetail {
    /// 指标字段 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    /// 指标字段值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    /// 字段值（当字段为人员信息时有值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_value_person: Option<UserId>,
}

/// 指标对应的环节状态（官方 reviewee_stage_status 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RevieweeStageStatus {
    /// 环节 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
    /// 环节类型：kpi_metric_setting 指标制定环节 / kpi_result_recording 结果录入环节
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_type: Option<String>,
    /// 环节状态：0 未开始 / 1 待提交 / 2 已逾期 / 3 确认中 / 4 被驳回 / 5 已完成
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_status: Option<i32>,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_performance_v2_metric_detail_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（semester_id + reviewee_metrics 子树）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "semester_id": "7291278856547794964",
                "reviewee_metrics": [
                    {
                        "reviewee_user_id": { "open_id": "od-asd2dasdasd", "user_id": "ou-ux987dsf6x" },
                        "metric_template_id": "7294570803306168339",
                        "metric_details": {
                            "metric_id": "7272581996315099155",
                            "name": "示例指标",
                            "fields": [
                                {
                                    "field_id": "7296082067266207764",
                                    "field_value": "90",
                                    "field_value_person": { "open_id": "od-asd2dasdasd", "user_id": "ou-ux987dsf6x" }
                                }
                            ],
                            "dimension_id": "7303895818346430484",
                            "dimension_name": { "zh_cn": "维度一", "en_us": "Dimension 1" },
                            "dimension_weight": "90%",
                            "add_from": "admin",
                            "is_from_library": true
                        },
                        "reviewee_stage_statuses": [
                            {
                                "stage_id": "1",
                                "stage_type": "kpi_metric_setting",
                                "stage_status": 0
                            }
                        ]
                    }
                ]
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/metric_details/query"))
            .and(body_json(json!({
                "semester_id": "6992035450862224940",
                "reviewee_user_ids": ["ou_3245842393d09e9428ad4655da6e30b3"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = QueryRequest::new(config, "6992035450862224940".to_string())
            .add_reviewee_user_id("ou_3245842393d09e9428ad4655da6e30b3".to_string())
            .user_id_type("open_id".to_string())
            .execute()
            .await
            .expect("performance_v2_metric_detail_query 应成功");

        assert_eq!(data.semester_id.as_deref(), Some("7291278856547794964"));
        let metrics = data
            .reviewee_metrics
            .as_ref()
            .expect("reviewee_metrics 应存在");
        assert_eq!(metrics.len(), 1);
        assert_eq!(
            metrics[0]
                .reviewee_user_id
                .as_ref()
                .and_then(|u| u.open_id.as_deref()),
            Some("od-asd2dasdasd")
        );
        assert_eq!(
            metrics[0].metric_template_id.as_deref(),
            Some("7294570803306168339")
        );
        let detail = metrics[0]
            .metric_details
            .as_ref()
            .expect("metric_details 应存在");
        assert_eq!(detail.metric_id.as_deref(), Some("7272581996315099155"));
        assert_eq!(detail.name.as_deref(), Some("示例指标"));
        assert_eq!(detail.add_from.as_deref(), Some("admin"));
        assert_eq!(detail.is_from_library, Some(true));
        assert_eq!(
            detail
                .dimension_name
                .as_ref()
                .and_then(|n| n.zh_cn.as_deref()),
            Some("维度一")
        );
        let fields = detail.fields.as_ref().expect("fields 应存在");
        assert_eq!(fields[0].field_value.as_deref(), Some("90"));
        let statuses = metrics[0]
            .reviewee_stage_statuses
            .as_ref()
            .expect("reviewee_stage_statuses 应存在");
        assert_eq!(
            statuses[0].stage_type.as_deref(),
            Some("kpi_metric_setting")
        );
        assert_eq!(statuses[0].stage_status, Some(0));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/metric_details/query"
        );
    }
}
