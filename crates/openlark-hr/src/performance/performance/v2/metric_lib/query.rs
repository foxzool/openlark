//! 获取指标列表
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/metric_lib/query>

use crate::common::shared_models::I18nText;
use crate::performance::performance::v2::models::UserId;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取指标列表请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 指标启用状态（可选）
    is_active: Option<bool>,
    /// 指标标签 ID 列表（可选，≤99）
    tag_ids: Option<Vec<String>>,
    /// 指标类型 ID 列表（可选，≤99）
    type_ids: Option<Vec<String>>,
    /// 指标可用范围（可选：admins_and_reviewees/only_admins）
    range_of_availability: Option<String>,
    /// 指标评分类型（可选：score_manually/score_by_formula）
    scoring_setting_type: Option<String>,
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
            is_active: None,
            tag_ids: None,
            type_ids: None,
            range_of_availability: None,
            scoring_setting_type: None,
            user_id_type: None,
            page_token: None,
            page_size: None,
            config,
        }
    }

    /// 设置指标启用状态（可选）
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    /// 设置指标标签 ID 列表（可选，≤99）
    pub fn tag_ids(mut self, tag_ids: Vec<String>) -> Self {
        self.tag_ids = Some(tag_ids);
        self
    }

    /// 设置指标类型 ID 列表（可选，≤99）
    pub fn type_ids(mut self, type_ids: Vec<String>) -> Self {
        self.type_ids = Some(type_ids);
        self
    }

    /// 设置指标可用范围（可选）
    pub fn range_of_availability(mut self, range_of_availability: String) -> Self {
        self.range_of_availability = Some(range_of_availability);
        self
    }

    /// 设置指标评分类型（可选）
    pub fn scoring_setting_type(mut self, scoring_setting_type: String) -> Self {
        self.scoring_setting_type = Some(scoring_setting_type);
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
        let api_endpoint = PerformanceApiV1::MetricLibQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("page_token", self.page_token)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()));

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            is_active: self.is_active,
            tag_ids: self.tag_ids,
            type_ids: self.type_ids,
            range_of_availability: self.range_of_availability,
            scoring_setting_type: self.scoring_setting_type,
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
            "获取指标列表响应数据为空",
        )
        .await
    }
}

/// 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 指标启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// 指标标签 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    /// 指标类型 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_ids: Option<Vec<String>>,
    /// 指标可用范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_of_availability: Option<String>,
    /// 指标评分类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_setting_type: Option<String>,
}

/// 获取指标列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    /// 指标信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<MetricLib>>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记（当 has_more 为 true 时返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 指标信息（官方 metric_in_library 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricLib {
    /// 指标 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    /// 指标名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 指标类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    /// 指标标签信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<MetricTagInfo>>,
    /// 指标字段信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<MetricFieldInLibrary>>,
    /// 指标评分设置类型：score_manually / score_by_formula
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoring_setting_type: Option<String>,
    /// 指标评分公式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoring_formula: Option<ScoringFormula>,
    /// 指标数据源录入人 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_source_inputters: Option<Vec<UserId>>,
    /// 指标可用范围：admins_and_reviewees / only_admins
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_of_availability: Option<String>,
    /// 指标状态是否为启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

/// 指标标签信息（官方 metric_tag 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricTagInfo {
    /// 指标标签 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<String>,
    /// 标签名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<I18nText>,
}

/// 指标字段信息（官方 metric_field_in_library 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricFieldInLibrary {
    /// 指标字段 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    /// 指标字段填写设置类型：admin / data_source_inputter / reviewee
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_setting: Option<String>,
    /// 指标字段值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    /// 字段值（当字段为人员信息时有值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_value_person: Option<UserId>,
}

/// 指标评分公式（官方 formula 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ScoringFormula {
    /// 公式 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formula_id: Option<String>,
    /// 公式名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formula_name: Option<String>,
    /// 公式详情
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formula_details: Option<String>,
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
    async fn test_performance_v2_metric_lib_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（metric_id/tags/fields/scoring_formula 等）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "items": [
                    {
                        "metric_id": "7272581996315099155",
                        "name": "销售额",
                        "type_id": "7272578300650717203",
                        "tags": [
                            {
                                "tag_id": "7302271694582841364",
                                "tag_name": { "zh_cn": "体验", "en_us": "Interactive experience" }
                            }
                        ],
                        "fields": [
                            {
                                "field_id": "7296701873237786643",
                                "input_setting": "admin",
                                "field_value": "90",
                                "field_value_person": { "open_id": "od-asd2dasdasd", "user_id": "ou-ux987dsf6x" }
                            }
                        ],
                        "scoring_setting_type": "score_by_formula",
                        "scoring_formula": {
                            "formula_id": "7302271694582841364",
                            "formula_name": "示例公式",
                            "formula_details": "1 + 1"
                        },
                        "data_source_inputters": [
                            { "open_id": "od-asd2dasdasd", "user_id": "ou-ux987dsf6x" }
                        ],
                        "range_of_availability": "admins_and_reviewees",
                        "is_active": true
                    }
                ],
                "has_more": true,
                "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/metric_libs/query"))
            .and(body_json(json!({
                "is_active": true,
                "tag_ids": ["7302271694582841364"],
                "type_ids": ["7272578300650717203"],
                "range_of_availability": "admins_and_reviewees",
                "scoring_setting_type": "score_manually"
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
            .is_active(true)
            .tag_ids(vec!["7302271694582841364".to_string()])
            .type_ids(vec!["7272578300650717203".to_string()])
            .range_of_availability("admins_and_reviewees".to_string())
            .scoring_setting_type("score_manually".to_string())
            .page_size(20)
            .execute()
            .await
            .expect("performance_v2_metric_lib_query 应成功");

        let items = data.items.as_ref().expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].metric_id.as_deref(), Some("7272581996315099155"));
        assert_eq!(items[0].name.as_deref(), Some("销售额"));
        assert_eq!(items[0].type_id.as_deref(), Some("7272578300650717203"));
        assert_eq!(items[0].is_active, Some(true));
        let tags = items[0].tags.as_ref().expect("tags 应存在");
        assert_eq!(
            tags[0].tag_name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("体验")
        );
        let fields = items[0].fields.as_ref().expect("fields 应存在");
        assert_eq!(fields[0].field_value.as_deref(), Some("90"));
        assert_eq!(
            fields[0]
                .field_value_person
                .as_ref()
                .and_then(|p| p.open_id.as_deref()),
            Some("od-asd2dasdasd")
        );
        let formula = items[0]
            .scoring_formula
            .as_ref()
            .expect("scoring_formula 应存在");
        assert_eq!(formula.formula_name.as_deref(), Some("示例公式"));
        let inputters = items[0]
            .data_source_inputters
            .as_ref()
            .expect("data_source_inputters 应存在");
        assert_eq!(inputters.len(), 1);
        assert_eq!(data.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/metric_libs/query"
        );
    }
}
