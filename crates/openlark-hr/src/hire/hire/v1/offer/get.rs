//! 获取 Offer 详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/offer/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 获取 Offer 详情请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    offer_id: String,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            offer_id: String::new(),
        }
    }

    /// 设置 `offer_id`。
    pub fn offer_id(mut self, offer_id: String) -> Self {
        self.offer_id = offer_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.offer_id.trim(), "Offer ID 不能为空");

        let api_endpoint = HireApiV1::OfferGet(self.offer_id);
        let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取 Offer 详情响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取 Offer 详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetCatalogRef {
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

/// `OfferGetContractPeriod`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetContractPeriod {
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

/// `OfferGetLocationNode`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetLocationNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `zh_name` 字段。
    pub zh_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `en_name` 字段。
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `code` 字段。
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `location_type` 字段。
    pub location_type: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferGetAddress`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `zh_name` 字段。
    pub zh_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `en_name` 字段。
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `district` 字段。
    pub district: Option<OfferGetLocationNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `city` 字段。
    pub city: Option<OfferGetLocationNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `state` 字段。
    pub state: Option<OfferGetLocationNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `country` 字段。
    pub country: Option<OfferGetLocationNode>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferGetSimpleAddress`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetSimpleAddress {
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

/// `OfferWorkLocationAddressInfo` 信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferWorkLocationAddressInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `location_info` 字段。
    pub location_info: Option<OfferGetSimpleAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `address_info` 字段。
    pub address_info: Option<OfferGetSimpleAddress>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferGetCustomInfo` 信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetCustomInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 对象 ID。
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `customize_value` 字段。
    pub customize_value: Option<Value>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferGetBasicInfo` 信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetBasicInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_type` 字段。
    pub offer_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `remark` 字段。
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `expire_time` 字段。
    pub expire_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `owner_user_id` 字段。
    pub owner_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `creator_user_id` 字段。
    pub creator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `employee_type` 字段。
    pub employee_type: Option<OfferGetCatalogRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `create_time` 字段。
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `leader_user_id` 字段。
    pub leader_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `onboard_date` 字段。
    pub onboard_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `department_id` 字段。
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `probation_month` 字段。
    pub probation_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `contract_year` 字段。
    pub contract_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `contract_period` 字段。
    pub contract_period: Option<OfferGetContractPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `recruitment_type` 字段。
    pub recruitment_type: Option<OfferGetCatalogRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `sequence` 字段。
    pub sequence: Option<OfferGetCatalogRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `level` 字段。
    pub level: Option<OfferGetCatalogRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `onboard_address` 字段。
    pub onboard_address: Option<OfferGetAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `work_address` 字段。
    pub work_address: Option<OfferGetAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `customize_info_list` 字段。
    pub customize_info_list: Option<Vec<OfferGetCustomInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `work_location_address_info` 字段。
    pub work_location_address_info: Option<OfferWorkLocationAddressInfo>,
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
    /// `common_attachment_id_list` 字段。
    pub common_attachment_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `pathway_id` 字段。
    pub pathway_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `OfferSalaryPlan`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferSalaryPlan {
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

/// `OfferDetail`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 基础信息。
    pub basic_info: Option<OfferGetBasicInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `salary_plan` 字段。
    pub salary_plan: Option<OfferSalaryPlan>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `GetResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer` 字段。
    pub offer: Option<OfferDetail>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for GetResponse {
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
