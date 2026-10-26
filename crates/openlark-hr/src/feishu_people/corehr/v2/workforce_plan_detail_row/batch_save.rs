//! 批量创建/更新明细行
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/workforce_plan_detail_row/batchSave>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量创建/更新明细行请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchsaveBody {
    /// 编制规划 id（必填）
    pub workforce_plan_id: String,
    /// 编制规划的明细行列表（必填，1～5 条）
    pub items: Vec<WorkforcePlanDetailRow>,
}

/// 编制规划明细行（官方 workforce_plan_detail_row 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkforcePlanDetailRow {
    /// 编制规划的维度信息（必填，1～20 条，需与用户创建的维度匹配）
    pub dimensions: Vec<DimensionEntity>,
    /// 预估在职人数相关信息（0～15 条）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eai_details: Option<Vec<WorkforcePlanEaiDetail>>,
    /// 编制规划值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_value: Option<String>,
    /// 自然周期的编制规划信息（0～15 条，编制规划方案为"按自然周期选择"时设置）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_period_values: Option<Vec<WorkforcePlanMultiPeriodValue>>,
}

/// 维度信息（官方 dimension_entity 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DimensionEntity {
    /// 维度 key（department/employee_type/location/position/cost_center/job_family/job_level/job/pathway/custom_org_01~05）
    pub dimension_key: String,
    /// 维度 value（对应维度的对象 ID）
    pub dimension_value: String,
}

/// 预估在职人数信息（官方 workforce_plan_eai_detail 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkforcePlanEaiDetail {
    /// 预估在职人数的日期（示例值格式："2020-10-31"）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 预估在职人数（需与创建编制规划时指定的小数位数匹配）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_active_individuals: Option<String>,
}

/// 自然周期编制规划信息（官方 workforce_plan_multi_period_value 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkforcePlanMultiPeriodValue {
    /// 周期的最后一天（填写自然周期编制规划信息时必填，示例值格式："2022-10-31"）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_date: Option<String>,
    /// 对应自然周期的编制规划值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workforce_plan: Option<String>,
    /// 对应自然周期的预增员数量（批量创建更新时无需写入）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_to_be_added: Option<String>,
    /// 对应自然周期的预减员数量（批量创建更新时无需写入）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_to_be_removed: Option<String>,
}

/// 批量创建/更新明细行请求
#[derive(Debug, Clone)]
pub struct BatchsaveRequest {
    /// 配置信息
    config: Config,
    body: BatchsaveBody,
}

impl BatchsaveRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: BatchsaveBody::default(),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: BatchsaveBody) -> Self {
        self.body = body;
        self
    }

    /// 设置编制规划 id。
    pub fn workforce_plan_id(mut self, workforce_plan_id: impl Into<String>) -> Self {
        self.body.workforce_plan_id = workforce_plan_id.into();
        self
    }

    /// 设置编制规划的明细行列表（1～5 条）。
    pub fn items(mut self, items: Vec<WorkforcePlanDetailRow>) -> Self {
        self.body.items = items;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchsaveResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchsaveResponse> {
        validate_required!(self.body.workforce_plan_id, "workforce_plan_id 不能为空");
        validate_required_list!(self.body.items, 5, "items 不能为空且单次最多 5 条");
        for item in &self.body.items {
            validate_required_list!(item.dimensions, 20, "dimensions 不能为空且单次最多 20 条");
            for dimension in &item.dimensions {
                validate_required!(dimension.dimension_key, "dimension_key 不能为空");
                validate_required!(dimension.dimension_value, "dimension_value 不能为空");
            }
        }

        let request = ApiRequest::<BatchsaveResponse>::post(
            "/open-apis/corehr/v2/workforce_plan_detail_row/batchSave",
        )
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 批量创建/更新明细行响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchsaveResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for BatchsaveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/workforce_plan_detail_row/batchSave
    #[tokio::test]
    async fn test_batchsave_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v2/workforce_plan_detail_row/batchSave",
            ))
            .and(body_json(json!({
                "workforce_plan_id": "7435976673130317356",
                "items": [
                    {
                        "dimensions": [
                            { "dimension_key": "department", "dimension_value": "7371716294248908332" },
                            { "dimension_key": "location", "dimension_value": "7312702817660487212" }
                        ],
                        "plan_value": "919",
                        "eai_details": [
                            { "date": "2029-01-31", "estimated_active_individuals": "9" }
                        ]
                    }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        BatchsaveRequest::new(config)
            .workforce_plan_id("7435976673130317356")
            .items(vec![WorkforcePlanDetailRow {
                dimensions: vec![
                    DimensionEntity {
                        dimension_key: "department".to_string(),
                        dimension_value: "7371716294248908332".to_string(),
                    },
                    DimensionEntity {
                        dimension_key: "location".to_string(),
                        dimension_value: "7312702817660487212".to_string(),
                    },
                ],
                plan_value: Some("919".to_string()),
                eai_details: Some(vec![WorkforcePlanEaiDetail {
                    date: Some("2029-01-31".to_string()),
                    estimated_active_individuals: Some("9".to_string()),
                }]),
                multi_period_values: None,
            }])
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
