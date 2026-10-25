//! 获取职位信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/get>

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

use crate::common::shared_models::{CodeNameObject, I18nText, IdNameObject};
use crate::hire::hire::common_models::JobRecruiterRecord;

/// 获取职位信息请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    job_id: String,
    /// 配置信息
    config: Config,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            job_id: String::new(),
            config,
        }
    }

    /// 设置 `job_id`。
    pub fn job_id(mut self, job_id: String) -> Self {
        self.job_id = job_id;
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

        validate_required!(self.job_id.trim(), "职位 ID 不能为空");

        let api_endpoint = HireApiV1::JobGet(self.job_id);
        let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取职位信息响应数据为空",
        )
        .await
    }
}

/// 获取职位信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `job` 字段（官方响应 data 下的职位数据容器）。
    pub job: Option<JobDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_name` 字段。
    pub job_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `process_id` 字段。
    pub process_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `process_name` 字段。
    pub process_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `department_id` 字段。
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `recruiters` 字段。
    pub recruiters: Option<Vec<JobRecruiterRecord>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位详情（官方 `job` 对象，data 下唯一业务键）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位名称。
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位描述。
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位编号。
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位要求。
    pub requirement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 雇佣类型。
    pub recruitment_type: Option<JobRecruitmentType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门。
    pub department: Option<JobDepartmentInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作地点。
    pub city: Option<JobCityInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低职级。
    pub min_job_level: Option<JobLevelInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高职级。
    pub max_job_level: Option<JobLevelInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位亮点列表。
    pub highlight_list: Option<Vec<JobHighlightInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列。
    pub job_category: Option<JobCategoryInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别。
    pub job_type: Option<JobTypeInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建人 ID，若为空则为系统或其他对接系统创建。
    pub create_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间（已废弃，请使用 `create_timestamp`）。
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 更新时间（已废弃，请使用 `update_timestamp`）。
    pub update_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘流程类型（1：社招流程 2：校招流程）。
    pub process_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘流程 ID。
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘流程中文名称。
    pub process_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘流程英文名称。
    pub process_en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 自定义字段列表。
    pub customized_data_list: Option<Vec<JobCustomizedData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职能分类。
    pub job_function: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位项目。
    pub subject: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘数量。
    pub head_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作年限。
    pub experience: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 到期日期（已废弃，请使用 `expiry_timestamp`）。
    pub expiry_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低薪资，单位:k。
    pub min_salary: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高薪资，单位:k。
    pub max_salary: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 学历要求。
    pub required_degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作地点列表。
    pub city_list: Option<Vec<CodeNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位属性（1：实体职位 2：虚拟职位）。
    pub job_attribute: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间戳。
    pub create_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 更新时间戳。
    pub update_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 到期时间戳。
    pub expiry_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 目标专业列表（「0」为不限专业）。
    pub target_major_list: Option<Vec<JobTargetMajorInfo>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 雇佣类型（官方 `job_recruitment_type`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRecruitmentType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 雇佣类型 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 雇佣类型中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 雇佣类型英文名称。
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 雇佣类型启用状态（1：启用 2：未启用）。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位部门（官方 `job_department`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobDepartmentInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门英文名称。
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位工作地点（官方 `job_city`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCityInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作地点城市代码。
    pub city_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作地点中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作地点英文名称。
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位职级（官方 `job_level`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobLevelInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职级 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职级中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职级英文名称。
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职级启用状态（1：启用 2：未启用）。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位亮点（官方 `job_highlight`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobHighlightInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位亮点 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位亮点中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位亮点英文名称。
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位序列（官方 `job_category`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCategoryInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列英文名称。
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列启用状态（1：启用 2：未启用）。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位类别（官方 `job_type`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobTypeInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别英文名称。
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 启用状态（1：启用 2：未启用）。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位自定义字段（官方 `job_customized_data`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCustomizedData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 自定义字段 ID。
    pub object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段类型（1-11，见官方枚举）。
    pub object_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 自定义字段值。
    pub value: Option<JobCustomizedValue>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位自定义字段值（官方 `job_customized_value`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCustomizedValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当字段类型为单行文本、多行文本、模块、默认字段时，从此字段取值。
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当字段类型为单选时，从此字段取值。
    pub option: Option<JobCustomizedOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当字段类型为多选时，从此字段取值。
    pub option_list: Option<Vec<JobCustomizedOption>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当字段类型为时间段时，从此字段取值。
    pub time_range: Option<JobCustomizedTimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当字段类型为日期选择、月份选择、年份选择时，从此字段取值（毫秒级时间戳）。
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当字段类型为数字时，从此字段取值。
    pub number: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位自定义字段选项（官方 `job_customized_option`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCustomizedOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项 ID。
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项名称。
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位自定义字段时间段（官方 `job_customized_time_range`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCustomizedTimeRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 开始时间。
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 结束时间。
    pub end_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位目标专业（官方 `target_major_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobTargetMajorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 目标专业 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 目标专业中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 目标专业英文名称。
    pub en_name: Option<String>,
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
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_get_request_builder_new() {
        let request = GetRequest::new(TestConfigBuilder::new().build()).job_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_get_request_validation_fails_on_default_request() {
        let request = GetRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
