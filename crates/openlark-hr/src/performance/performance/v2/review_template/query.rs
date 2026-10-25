//! 获取绩效模板配置
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/review_template/query>

use crate::common::shared_models::I18nText;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取绩效模板配置请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 绩效模板 ID 列表（可选，≤50，不传返回所有）
    review_template_ids: Option<Vec<String>>,
    /// 分页标记（可选 query 参数）
    page_token: Option<String>,
    /// 分页大小（可选 query 参数，默认 20，0～50）
    page_size: Option<i32>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            review_template_ids: None,
            page_token: None,
            page_size: None,
            config,
        }
    }

    /// 设置绩效模板 ID 列表（可选，≤50）
    pub fn review_template_ids(mut self, review_template_ids: Vec<String>) -> Self {
        self.review_template_ids = Some(review_template_ids);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小（可选，默认 20，0～50）
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
        let api_endpoint = PerformanceApiV1::ReviewTemplateQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query_opt("page_token", self.page_token)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()));

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            review_template_ids: self.review_template_ids,
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
            "获取绩效模板配置响应数据为空",
        )
        .await
    }
}

/// 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 绩效模板 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_template_ids: Option<Vec<String>>,
}

/// 获取绩效模板配置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    /// 绩效模板信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_templates: Option<Vec<ReviewTemplate>>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记（当 has_more 为 true 时返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 绩效模板（官方 review_template 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReviewTemplate {
    /// 环节模板列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<TemplateItem>>,
    /// 评估内容列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units: Option<Vec<ReviewUnit>>,
    /// 绩效模板 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_template_id: Option<String>,
    /// 绩效模板名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 绩效模板描述（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 状态：to_be_configured / to_be_enabled / enabled / disabled / deleted
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 环节模板（官方 template 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TemplateItem {
    /// 环节模板 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 环节模板对应的环节名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 环节类型：summarize_key_outputs / review / communication_and_open_result / view_result / calibration / reconsideration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_type: Option<String>,
    /// 评估型环节的执行人角色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_stage_role: Option<String>,
    /// 评估型环节评估模式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_stage_data_write_mode: Option<String>,
}

/// 评估内容（官方 unit 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReviewUnit {
    /// 评估内容 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    /// 评估内容名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 评估题列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<UnitField>>,
}

/// 评估题（官方 field 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UnitField {
    /// 评估题 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    /// 评估题名称（i18n；填写项可能为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 评估项 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_id: Option<String>,
    /// 标签填写题 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_based_question_id: Option<String>,
    /// O 的填写项标题（i18n；官方文档字段名即为 qustion）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective_text_qustion_title: Option<I18nText>,
    /// KR 的填写项标题（i18n；官方文档字段名即为 qustion）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyresult_text_qustion_title: Option<I18nText>,
    /// 关联的父级评估项 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_field_id: Option<String>,
    /// 指标模板 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kpi_template_id: Option<String>,
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
    async fn test_performance_v2_review_template_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（review_templates 容器 + templates/units/fields 子树）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "review_templates": [
                    {
                        "templates": [
                            {
                                "template_id": "7343513161666707459",
                                "name": { "zh_cn": "体验", "en_us": "Interactive experience" },
                                "stage_type": "leader_review",
                                "review_stage_role": "reviewee",
                                "review_stage_data_write_mode": "cooperate,independent"
                            }
                        ],
                        "units": [
                            {
                                "unit_id": "7343513161666723843",
                                "name": { "zh_cn": "体验", "en_us": "Interactive experience" },
                                "fields": [
                                    {
                                        "field_id": "7343513161666707459",
                                        "name": { "zh_cn": "体验", "en_us": "Interactive experience" },
                                        "indicator_id": "7343513161666707459",
                                        "tag_based_question_id": "7343513161666707459",
                                        "objective_text_qustion_title": { "zh_cn": "体验", "en_us": "Interactive experience" },
                                        "keyresult_text_qustion_title": { "zh_cn": "体验", "en_us": "Interactive experience" },
                                        "parent_field_id": "7343513161666707459",
                                        "kpi_template_id": "7494252079230222371"
                                    }
                                ]
                            }
                        ],
                        "review_template_id": "7343513161666723843",
                        "name": { "zh_cn": "体验", "en_us": "Interactive experience" },
                        "description": { "zh_cn": "体验", "en_us": "Interactive experience" },
                        "status": "enabled"
                    }
                ],
                "has_more": true,
                "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/review_templates/query"))
            .and(body_json(json!({
                "review_template_ids": ["7343513161666707459"]
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
            .review_template_ids(vec!["7343513161666707459".to_string()])
            .page_size(30)
            .execute()
            .await
            .expect("performance_v2_review_template_query 应成功");

        let templates = data
            .review_templates
            .as_ref()
            .expect("review_templates 应存在");
        assert_eq!(templates.len(), 1);
        assert_eq!(
            templates[0].review_template_id.as_deref(),
            Some("7343513161666723843")
        );
        assert_eq!(templates[0].status.as_deref(), Some("enabled"));
        assert_eq!(
            templates[0].name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("体验")
        );
        let stage_templates = templates[0].templates.as_ref().expect("templates 应存在");
        assert_eq!(
            stage_templates[0].stage_type.as_deref(),
            Some("leader_review")
        );
        assert_eq!(
            stage_templates[0].review_stage_role.as_deref(),
            Some("reviewee")
        );
        let units = templates[0].units.as_ref().expect("units 应存在");
        let fields = units[0].fields.as_ref().expect("fields 应存在");
        assert_eq!(
            fields[0].kpi_template_id.as_deref(),
            Some("7494252079230222371")
        );
        assert_eq!(data.has_more, Some(true));
        assert!(data.page_token.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/review_templates/query"
        );
    }
}
