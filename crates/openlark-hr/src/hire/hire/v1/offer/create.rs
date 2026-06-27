//! 创建 Offer
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/offer/create

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 创建 Offer请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        let api_endpoint = HireApiV1::OfferCreate;
        let mut request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建 Offer 响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 创建 Offer响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferCatalogRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `zh_name` 字段。
    pub zh_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `en_name` 字段。
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferContractPeriod`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferContractPeriod {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `period_type` 字段。
    pub period_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `period` 字段。
    pub period: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferBasicInfoUpsert`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferBasicInfoUpsert {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `department_id` 字段。
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `leader_user_id` 字段。
    pub leader_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `employment_job_id` 字段。
    pub employment_job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `employee_type_id` 字段。
    pub employee_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_family_id` 字段。
    pub job_family_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_level_id` 字段。
    pub job_level_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `probation_month` 字段。
    pub probation_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `contract_year` 字段。
    pub contract_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `contract_period` 字段。
    pub contract_period: Option<OfferContractPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `expected_onboard_date` 字段。
    pub expected_onboard_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `onboard_address_id` 字段。
    pub onboard_address_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `work_address_id` 字段。
    pub work_address_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `owner_user_id` 字段。
    pub owner_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `recommended_words` 字段。
    pub recommended_words: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_requirement_id` 字段。
    pub job_requirement_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_process_type_id` 字段。
    pub job_process_type_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `attachment_id_list` 字段。
    pub attachment_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `common_attachment_id_list` 字段。
    pub common_attachment_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `attachment_description` 字段。
    pub attachment_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `operator_user_id` 字段。
    pub operator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `position_id` 字段。
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_offered` 字段。
    pub job_offered: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_grade_id` 字段。
    pub job_grade_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `pathway_id` 字段。
    pub pathway_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferSalaryInfo` 信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferSalaryInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `currency` 字段。
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `basic_salary` 字段。
    pub basic_salary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `probation_salary_percentage` 字段。
    pub probation_salary_percentage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `award_salary_multiple` 字段。
    pub award_salary_multiple: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `option_shares` 字段。
    pub option_shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `quarterly_bonus` 字段。
    pub quarterly_bonus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `half_year_bonus` 字段。
    pub half_year_bonus: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferCustomizedInfo` 信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferCustomizedInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 值。
    pub value: Option<Value>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `CreateResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_id` 字段。
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `schema_id` 字段。
    pub schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_type` 字段。
    pub offer_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 基础信息。
    pub basic_info: Option<OfferBasicInfoUpsert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `salary_info` 字段。
    pub salary_info: Option<OfferSalaryInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `customized_info_list` 字段。
    pub customized_info_list: Option<Vec<OfferCustomizedInfo>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
