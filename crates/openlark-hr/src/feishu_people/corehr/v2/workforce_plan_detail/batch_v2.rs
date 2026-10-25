//! 查询编制规划明细信息（支持自定义组织）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/workforce_plan_detail/batch_v2>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询编制规划明细信息（支持自定义组织）请求
#[derive(Debug, Clone)]
pub struct BatchV2Request {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl BatchV2Request {
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
    pub async fn execute(self) -> SDKResult<BatchV2Response> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchV2Response> {
        let mut request = ApiRequest::<BatchV2Response>::post(
            "/open-apis/corehr/v2/workforce_plan_details/batch_v2",
        );

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 查询编制规划明细信息（支持自定义组织）响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchV2Response {
    /// 编制规划方案 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workforce_plan_id: Option<String>,
    /// 集中填报项目 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub centralized_reporting_project_id: Option<String>,
    /// 编制规划明细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<WorkforcePlanDetailV2>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 编制规划明细（官方 workforce_plan_detail_v2 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkforcePlanDetailV2 {
    /// 编制规划明细 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workforce_plan_detail_id: Option<String>,
    /// 维度信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_info_datas: Option<Vec<DimensionInfoData>>,
    /// 编制规划值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workforce_plan: Option<String>,
    /// 在职人数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_individuals: Option<String>,
    /// 预增员数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_to_be_added: Option<String>,
    /// 预减员数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_to_be_removed: Option<String>,
    /// 预估在职人数明细
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_active_individuals_details: Option<Vec<WorkforcePlanEaiDetail>>,
    /// 自然周期的编制规划信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_period_values: Option<Vec<WorkforcePlanMultiPeriodValue>>,
    /// 是否为缺维度的明细行
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_missing_dimension: Option<bool>,
    /// 是否在职、预增/预减人员、编制数、预估在职人数都为 0 的明细行
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_all_zero_value: Option<bool>,
}

/// 维度信息（官方 dimension_info_data 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DimensionInfoData {
    /// 维度 key（department/employee_type/location/position/cost_center/job_family/job_level/job/pathway/custom_org_xx）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_key: Option<String>,
    /// 维度信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_info: Option<DimensionInfo>,
}

/// 维度信息（官方 dimension_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DimensionInfo {
    /// 维度 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 维度名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
}

/// 预估在职人数明细（官方 workforce_plan_eai_detail 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkforcePlanEaiDetail {
    /// 预估月份
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 预估在职人数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_active_individuals: Option<String>,
}

/// 自然周期的编制规划信息（官方 workforce_plan_multi_period_value 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkforcePlanMultiPeriodValue {
    /// 自然周期的最后一天
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_date: Option<String>,
    /// 对应自然周期的编制规划值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workforce_plan: Option<String>,
    /// 对应自然周期的预增员数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_to_be_added: Option<String>,
    /// 对应自然周期的预减员数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub individuals_to_be_removed: Option<String>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for BatchV2Response {
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

    /// 端到端：POST /open-apis/corehr/v2/workforce_plan_details/batch_v2
    #[tokio::test]
    async fn test_batch_v2_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/workforce_plan_details/batch_v2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "workforce_plan_id": "7128319234123",
                    "centralized_reporting_project_id": "7128319234123",
                    "items": [
                        {
                            "workforce_plan_detail_id": "123456",
                            "dimension_info_datas": [
                                {
                                    "dimension_key": "department",
                                    "dimension_info": {
                                        "id": "123456",
                                        "name": [
                                            { "lang": "zh-CN", "value": "中文示例" }
                                        ]
                                    }
                                }
                            ],
                            "workforce_plan": "10.00",
                            "active_individuals": "10.00",
                            "individuals_to_be_added": "10.00",
                            "individuals_to_be_removed": "10.00",
                            "estimated_active_individuals_details": [
                                {
                                    "date": "2020-10-31",
                                    "estimated_active_individuals": "10.00"
                                }
                            ],
                            "multi_period_values": [
                                {
                                    "period_date": "2022-10-31",
                                    "workforce_plan": "12.00",
                                    "individuals_to_be_added": "10.00",
                                    "individuals_to_be_removed": "10.00"
                                }
                            ],
                            "is_missing_dimension": false,
                            "is_all_zero_value": false
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

        let resp = BatchV2Request::new(config)
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.workforce_plan_id, Some("7128319234123".to_string()));
        assert_eq!(
            resp.centralized_reporting_project_id,
            Some("7128319234123".to_string())
        );
        assert_eq!(resp.has_more, Some(true));
        let items = resp.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].workforce_plan_detail_id,
            Some("123456".to_string())
        );
        assert_eq!(items[0].active_individuals, Some("10.00".to_string()));
        assert_eq!(
            items[0].individuals_to_be_removed,
            Some("10.00".to_string())
        );
        let dimension_info_datas = items[0]
            .dimension_info_datas
            .clone()
            .expect("dimension_info_datas 应存在");
        assert_eq!(
            dimension_info_datas[0].dimension_key,
            Some("department".to_string())
        );
        assert_eq!(
            dimension_info_datas[0].dimension_info,
            Some(DimensionInfo {
                id: Some("123456".to_string()),
                name: Some(vec![I18nText {
                    lang: Some("zh-CN".to_string()),
                    value: Some("中文示例".to_string()),
                }]),
            })
        );
        let eai_details = items[0]
            .estimated_active_individuals_details
            .clone()
            .expect("estimated_active_individuals_details 应存在");
        assert_eq!(
            eai_details[0].estimated_active_individuals,
            Some("10.00".to_string())
        );
        let multi_period_values = items[0]
            .multi_period_values
            .clone()
            .expect("multi_period_values 应存在");
        assert_eq!(
            multi_period_values[0].period_date,
            Some("2022-10-31".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
