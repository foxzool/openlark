//! 获取面试评价表列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/interview_feedback_form/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::common::shared_models::I18nText;

/// 面试评价表（官方 `interview_feedback_form`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewFeedbackForm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表版本号。
    pub version: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表名称。
    pub name: Option<I18nText>,
    /// 面试评价表类型：1 普通评价表 / 2 打分评价表 / 3 权重评价表。
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub form_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 打分计算配置，当评价表类型为打分评价表（type=2）时生效。
    pub score_calculation_config: Option<ScoreCalculationConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表模块列表。
    pub modules: Option<Vec<InterviewFeedbackFormModule>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 打分计算配置（官方 `score_calculation_config`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ScoreCalculationConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否启用。
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 计算类型：1 计算总分 / 2 计算平均分。
    pub calculation_mode: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试评价表模块（官方 `interview_feedback_form_module`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewFeedbackFormModule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表模块 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块描述。
    pub description: Option<I18nText>,
    /// 模块类型：1 系统预置「面试结论」模块 / 2 自定义模块。
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub module_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块顺序。
    pub sequence: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块权重。
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块维度列表。
    pub dimensions: Option<Vec<InterviewFeedbackFormDimension>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试评价表模块维度（官方 `interview_feedback_form_dimension`）。
///
/// 维度深层的 `score_dimension_config` / `option_items` / `ability_list` /
/// `related_dimension_config` 暂落入 `extra` 过渡。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewFeedbackFormDimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块维度 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 维度名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 维度描述。
    pub description: Option<I18nText>,
    /// 维度类型：1 单选题 / 2 多选题 / 3 描述题 / 5 职级建议 / 6 打分题(单选) / 7 打分题(填空) / 10-12 系统预置。
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub dimension_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否启用。
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 维度顺序。
    pub sequence: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否必选。
    pub is_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 维度权重。
    pub weight: Option<f64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `ListRequestBody`。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
}

/// 获取面试评价表列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::HireApiV1;

        if let Some(size) = self.page_size
            && !(1..=100).contains(&size)
        {
            return Err(openlark_core::error::validation_error(
                "分页大小超出范围",
                "page_size 必须在 1-100 之间",
            ));
        }

        let api_endpoint = HireApiV1::InterviewFeedbackFormList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());
        let request_body = ListRequestBody {
            page_size: self.page_size,
            page_token: self.page_token,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取面试评价表列表响应数据为空",
        )
        .await
    }
}

/// 获取面试评价表列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<InterviewFeedbackForm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/hire/v1/interview_feedback_forms
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/interview_feedback_forms"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [{
                        "id": "6969137186734393644",
                        "version": 1,
                        "name": { "zh_cn": "默认面试评价表", "en_us": "default interview assessment template" },
                        "type": 1,
                        "score_calculation_config": { "enabled": true, "calculation_mode": 1 },
                        "modules": [{
                            "id": "6930815272790114324",
                            "name": { "zh_cn": "默认面试评价表模块", "en_us": "default interview assessment module" },
                            "description": { "zh_cn": "用于填写面试评价", "en_us": "for completing interview evaluation" },
                            "type": 1,
                            "sequence": 1,
                            "weight": 1,
                            "dimensions": [{
                                "id": "6930815272790114324",
                                "name": { "zh_cn": "默认评价维度", "en_us": "default dimension" },
                                "type": 1,
                                "enabled": true,
                                "sequence": 1,
                                "is_required": true,
                                "weight": 1
                            }]
                        }]
                    }],
                    "has_more": true
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

        let resp = ListRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        // 官方 item 含 type/score_calculation_config/modules（.md L45-66）
        let form = resp.items.first().expect("items 应反序列化");
        assert_eq!(form.id.as_deref(), Some("6969137186734393644"));
        assert_eq!(form.form_type, Some(1));
        assert_eq!(
            form.score_calculation_config
                .as_ref()
                .and_then(|v| v.calculation_mode),
            Some(1)
        );
        let module = form
            .modules
            .as_ref()
            .and_then(|v| v.first())
            .expect("modules 应反序列化");
        assert_eq!(module.module_type, Some(1));
        assert!(
            module
                .dimensions
                .as_ref()
                .map(|v| !v.is_empty())
                .unwrap_or(false)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
