//! 获取指标模板列表
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/metric_template/query>

use crate::common::shared_models::I18nText;
use crate::performance::performance::v2::models::UserId;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取指标模板列表请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 指标模板 ID 列表（可选，≤50，不传分页返回所有）
    metrics_template_ids: Option<Vec<String>>,
    /// 指标模版状态（可选：to_be_configured/to_be_activated/enabled/disabled）
    status: Option<String>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 分页标记（可选 query 参数）
    page_token: Option<String>,
    /// 分页大小（可选 query 参数，默认 20，1～50）
    page_size: Option<i32>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            metrics_template_ids: None,
            status: None,
            user_id_type: None,
            page_token: None,
            page_size: None,
            config,
        }
    }

    /// 设置指标模板 ID 列表（可选，≤50）
    pub fn metrics_template_ids(mut self, metrics_template_ids: Vec<String>) -> Self {
        self.metrics_template_ids = Some(metrics_template_ids);
        self
    }

    /// 设置指标模版状态（可选）
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小（可选，默认 20，1～50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
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

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::MetricTemplateQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("page_token", self.page_token)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()));

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            metrics_template_ids: self.metrics_template_ids,
            status: self.status,
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
            "获取指标模板列表响应数据为空",
        )
        .await
    }
}

/// 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 指标模板 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_template_ids: Option<Vec<String>>,
    /// 指标模版状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 获取指标模板列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    /// 指标模板信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MetricTemplate>>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记（当 has_more 为 true 时返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 指标模板（官方 metric_template 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricTemplate {
    /// 指标模板 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 模版名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 模板描述（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 模版状态：to_be_configured / to_be_activated / enabled / disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 模版是否分人群设置指标
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_set_by_group: Option<bool>,
    /// 模版指标总分计算方式：review_manually / sum / weight / formula
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_metric_score_method: Option<String>,
    /// 指标权重计算方式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_weight_method: Option<String>,
    /// 指标维度列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_dimensions: Option<Vec<MetricDimension>>,
    /// 指标列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<MetricInTemplate>>,
    /// 人群分组
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<MetricGroup>>,
}

/// 指标维度（官方 metric_dimension 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricDimension {
    /// 所属人群分组 ID（未分人群设置时为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 指标维度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_dimension_id: Option<String>,
    /// 指标维度名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 指标评估规则 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluation_rule_id_for_each_metric: Option<String>,
    /// 维度权重（未设置时为空，单位为百分比）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_weight: Option<String>,
    /// 维度描述（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 各指标的评估规则：0 使用相同规则 / 1 使用不同规则
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_rule_option: Option<i32>,
    /// 被评估人添加指标的设置
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metric_config: Option<CustomMetricConfig>,
}

/// 被评估人添加指标的设置（官方 custom_metric_config 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CustomMetricConfig {
    /// 非指标库指标的评分方式（手动评分是 0；评分公式是具体的公式 ID）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_formula_id: Option<String>,
    /// 最少需添加的指标数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub least_metrics_size: Option<i32>,
    /// 添加指标的方式：1 可选用指标库的指标 / 2 可选用自定义的指标
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_metric_options: Option<Vec<i32>>,
}

/// 模板中的指标（官方 metric_in_template 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricInTemplate {
    /// 所属人群分组 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 指标 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    /// 指标名称（指标在当前模板中的名称）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 指标类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    /// 指标字段信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<MetricFieldInTemplate>>,
    /// 指标是否引自指标库
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_from_library: Option<bool>,
    /// 评分设置类型：score_manually / score_by_formula
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoring_setting_type: Option<String>,
    /// 数据源录入人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_source_inputters: Option<Vec<UserId>>,
    /// 指标维度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_dimension_id: Option<String>,
    /// 评估规则
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_rule_config: Option<MetricReviewRuleConfig>,
}

/// 模板中的指标字段（官方 metric_field_in_template 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricFieldInTemplate {
    /// 指标字段 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 字段填写方式设置：admin / data_source_inputter / reviewee
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_setting: Option<String>,
    /// 字段值（官方文档字段名即为 filed_value）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filed_value: Option<String>,
    /// 字段值（当字段为人员信息时有值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_value_person: Option<UserId>,
}

/// 指标评估规则（官方 metric_review_rule_config 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricReviewRuleConfig {
    /// 最高分
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    /// 最低分
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
}

/// 人群分组（官方 metric_group 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricGroup {
    /// 人群分组 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 人群分组名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    async fn test_performance_v2_metric_template_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（i18n name/description + metric_dimensions/
        // metrics/groups 子树，注意 filed_value 为官方拼写）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "items": [
                    {
                        "id": "7296488199415660563",
                        "name": { "zh_cn": "体验", "en_us": "Interactive experience" },
                        "description": { "zh_cn": "体验", "en_us": "Interactive experience" },
                        "status": "to_be_configured",
                        "is_set_by_group": true,
                        "total_metric_score_method": "weight",
                        "metric_weight_method": "sum_of_metric_weights_for_each_dimension_equals_100",
                        "metric_dimensions": [
                            {
                                "group_id": "7296753366268215316",
                                "metric_dimension_id": "7296753366268215316",
                                "name": { "zh_cn": "体验", "en_us": "Interactive experience" },
                                "evaluation_rule_id_for_each_metric": "7296701873237786643",
                                "dimension_weight": "90",
                                "description": { "zh_cn": "维度描述", "en_us": "Dimension description" },
                                "review_rule_option": 0,
                                "custom_metric_config": {
                                    "default_formula_id": "7296701873237786643",
                                    "least_metrics_size": 1,
                                    "add_metric_options": [1, 2]
                                }
                            }
                        ],
                        "metrics": [
                            {
                                "group_id": "7272581996315099155",
                                "metric_id": "7272581996315099155",
                                "name": "销售额",
                                "type_id": "7272581996315099155",
                                "fields": [
                                    {
                                        "id": "7272581996315099155",
                                        "input_setting": "admin",
                                        "filed_value": "100",
                                        "field_value_person": { "open_id": "od-asd2dasdasd", "user_id": "ou-ux987dsf6x" }
                                    }
                                ],
                                "is_from_library": true,
                                "scoring_setting_type": "score_by_formula",
                                "data_source_inputters": [
                                    { "open_id": "od-asd2dasdasd", "user_id": "ou-ux987dsf6x" }
                                ],
                                "metric_dimension_id": "7272581996315099155",
                                "review_rule_config": { "max": "10", "min": "0" }
                            }
                        ],
                        "groups": [
                            { "group_id": "7296466589195436051", "name": "分组 1" }
                        ]
                    }
                ],
                "has_more": true,
                "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/metric_templates/query"))
            .and(body_json(json!({
                "metrics_template_ids": ["7360956875099078676"],
                "status": "to_be_configured"
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

        let data = QueryRequest::new(config)
            .metrics_template_ids(vec!["7360956875099078676".to_string()])
            .status("to_be_configured".to_string())
            .page_size(30)
            .execute()
            .await
            .expect("performance_v2_metric_template_query 应成功");

        let items = data.items.as_ref().expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id.as_deref(), Some("7296488199415660563"));
        assert_eq!(
            items[0].name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("体验")
        );
        assert_eq!(items[0].status.as_deref(), Some("to_be_configured"));
        assert_eq!(items[0].is_set_by_group, Some(true));
        assert_eq!(
            items[0].total_metric_score_method.as_deref(),
            Some("weight")
        );
        let dimensions = items[0]
            .metric_dimensions
            .as_ref()
            .expect("metric_dimensions 应存在");
        assert_eq!(dimensions[0].dimension_weight.as_deref(), Some("90"));
        let custom = dimensions[0]
            .custom_metric_config
            .as_ref()
            .expect("custom_metric_config 应存在");
        assert_eq!(custom.least_metrics_size, Some(1));
        let metrics = items[0].metrics.as_ref().expect("metrics 应存在");
        assert_eq!(metrics[0].name.as_deref(), Some("销售额"));
        let fields = metrics[0].fields.as_ref().expect("fields 应存在");
        assert_eq!(fields[0].filed_value.as_deref(), Some("100"));
        let rule = metrics[0]
            .review_rule_config
            .as_ref()
            .expect("review_rule_config 应存在");
        assert_eq!(rule.max.as_deref(), Some("10"));
        let groups = items[0].groups.as_ref().expect("groups 应存在");
        assert_eq!(groups[0].name.as_deref(), Some("分组 1"));
        assert_eq!(data.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/metric_templates/query"
        );
    }
}
