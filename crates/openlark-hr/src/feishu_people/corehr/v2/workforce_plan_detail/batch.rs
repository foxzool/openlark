//! 查询编制规划明细信息（不支持自定义组织）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/workforce_plan_detail/batch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询编制规划明细信息（不支持自定义组织）请求
#[derive(Debug, Clone)]
pub struct BatchRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl BatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchResponse> {
        let mut request =
            ApiRequest::<BatchResponse>::post("/open-apis/corehr/v2/workforce_plan_details/batch");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 查询编制规划明细信息（不支持自定义组织）响应
///
/// 官方响应字段直接平铺在 `data` 顶层（无 `data` 包装键）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchResponse {
    /// 响应数据
    /// 编制规划方案 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workforce_plan_id: Option<String>,
    /// 集中填报项目 ID（集中填报查询时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub centralized_reporting_project_id: Option<String>,
    /// 编制规划明细信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<WorkforcePlanDetail>>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 编制规划明细条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkforcePlanDetail {
    /// 编制规划明细 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workforce_plan_detail_id: Option<String>,
    /// 部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<DimensionInfo>,
    /// 人员类型信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<DimensionInfo>,
    /// 工作地点信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location: Option<DimensionInfo>,
    /// 序列信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family: Option<DimensionInfo>,
    /// 职级信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level: Option<DimensionInfo>,
    /// 职务信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<DimensionInfo>,
    /// 成本中心信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<DimensionInfo>,
    /// 编制规划值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workforce_plan: Option<String>,
    /// 预估在职人数明细
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_active_individuals_detail: Option<Vec<WorkforcePlanEaiDetail>>,
    /// 是否为缺维度的明细行
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_missing_dimension: Option<bool>,
}

/// 维度信息（部门/人员类型/地点/序列/职级/职务/成本中心通用）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DimensionInfo {
    /// 维度 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 维度名称（i18n 多语数组）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
}

/// 预估在职人数明细条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkforcePlanEaiDetail {
    /// 预估月份
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 预估在职人数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_active_individuals: Option<String>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言信息（中文是 zh-CN，英文是 en-US）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for BatchResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/workforce_plan_details/batch
    #[tokio::test]
    async fn test_batch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/workforce_plan_details/batch"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "workforce_plan_id": "7128319234123",
                    "centralized_reporting_project_id": "7128319234123",
                    "items": [
                        {
                            "workforce_plan_detail_id": "123456",
                            "department": {
                                "id": "123456",
                                "name": [{ "lang": "zh-CN", "value": "中文示例" }]
                            },
                            "employee_type": {
                                "id": "123456",
                                "name": [{ "lang": "zh-CN", "value": "中文示例" }]
                            },
                            "work_location": {
                                "id": "123456",
                                "name": [{ "lang": "zh-CN", "value": "中文示例" }]
                            },
                            "job_family": {
                                "id": "123456",
                                "name": [{ "lang": "zh-CN", "value": "中文示例" }]
                            },
                            "job_level": {
                                "id": "123456",
                                "name": [{ "lang": "zh-CN", "value": "中文示例" }]
                            },
                            "job": {
                                "id": "123456",
                                "name": [{ "lang": "zh-CN", "value": "中文示例" }]
                            },
                            "cost_center": {
                                "id": "123456",
                                "name": [{ "lang": "zh-CN", "value": "中文示例" }]
                            },
                            "workforce_plan": "10.00",
                            "estimated_active_individuals_detail": [
                                { "date": "2020-10-31", "estimated_active_individuals": "10.00" }
                            ],
                            "is_missing_dimension": false
                        }
                    ],
                    "page_token": "34523459",
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

        let resp = BatchRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.workforce_plan_id.as_deref(), Some("7128319234123"));
        assert_eq!(
            resp.centralized_reporting_project_id.as_deref(),
            Some("7128319234123")
        );
        let detail = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("应返回编制规划明细条目");
        assert_eq!(detail.workforce_plan_detail_id.as_deref(), Some("123456"));
        assert_eq!(detail.workforce_plan.as_deref(), Some("10.00"));
        assert_eq!(detail.is_missing_dimension, Some(false));
        assert_eq!(
            detail.department.as_ref().and_then(|d| d.id.as_deref()),
            Some("123456")
        );
        assert_eq!(
            detail
                .department
                .as_ref()
                .and_then(|d| d.name.as_ref())
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("中文示例")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
