//! 获取职位设置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/config>

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

use crate::common::shared_models::IdNameObject;
use crate::hire::hire::common_models::JobConfigInfo;
use crate::hire::hire::v1::job::update_config::InterviewAppointmentConfig;

/// 获取职位设置请求
#[derive(Debug, Clone)]
pub struct ConfigRequest {
    job_id: String,
    /// 配置信息
    config: Config,
}

impl ConfigRequest {
    /// 创建请求
    pub fn new(config: Config, job_id: String) -> Self {
        Self { job_id, config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ConfigResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ConfigResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.job_id.trim(), "职位 ID 不能为空");

        let api_endpoint = HireApiV1::JobConfig(self.job_id);
        let request = ApiRequest::<ConfigResponse>::get(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取职位设置响应数据为空",
        )
        .await
    }
}

/// 登记表信息（官方 `registration_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RegistrationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 登记表模版 ID。
    pub schema_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 登记表名称。
    pub name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 建议面试官轮次（官方 `job_config_interview_round`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobConfigInterviewRound {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试官列表。
    pub interviewer_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次。
    pub round: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试轮次类型（官方 `job_config_round_type_result`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobConfigRoundTypeResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次类型。
    pub assessment_round: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表。
    pub assessment_template: Option<IdNameObject>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位设置（官方 `job_config_result`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobConfigResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 申请表。
    pub offer_apply_schema: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 审批流程。
    pub offer_process_conf: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 建议评估人列表。
    pub recommended_evaluator_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表。
    pub assessment_template: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 建议面试官列表。
    pub interview_round_list: Option<Vec<JobConfigInterviewRound>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘需求列表。
    pub job_requirement_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试登记表。
    pub interview_registration: Option<RegistrationInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 入职登记表。
    pub onboard_registration: Option<RegistrationInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次类型列表。
    pub interview_round_type_list: Option<Vec<JobConfigRoundTypeResult>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 关联职位列表。
    pub related_job_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位属性：1 实体职位 / 2 虚拟职位。
    pub job_attribute: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试官安排面试配置。
    pub interview_appointment_config: Option<InterviewAppointmentConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网申请表。
    pub portal_website_apply_form_schema_info: Option<RegistrationInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取职位设置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConfigResponse {
    #[serde(flatten)]
    /// `config` 字段。
    pub config: JobConfigInfo,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位设置。
    pub job_config: Option<JobConfigResult>,
}

impl ApiResponseTrait for ConfigResponse {
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
    fn test_config_request_builder_new() {
        let request = ConfigRequest::new(TestConfigBuilder::new().build(), "test".to_string());
        let _ = request;
    }

    /// 官方响应示例：`data.job_config` 容器应被类型化解析。
    #[test]
    fn test_config_response_parses_official_shape() {
        let resp: ConfigResponse = serde_json::from_value(serde_json::json!({
            "job_config": {
                "offer_apply_schema": {
                    "id": "7392444731470563625",
                    "name": { "zh_cn": "申请表1", "en_us": "Offer Apply 1" }
                },
                "offer_process_conf": {
                    "id": "7392444731470563625",
                    "name": { "zh_cn": "审批流程1", "en_us": "Test Process 1" }
                },
                "recommended_evaluator_list": [
                    { "id": "7392444731470563625", "name": { "zh_cn": "张三", "en_us": "ZhangSan" } }
                ],
                "assessment_template": {
                    "id": "7392444731470563625",
                    "name": { "zh_cn": "评价表1", "en_us": "assessment_template·1" }
                },
                "id": "6960663240925956574",
                "interview_round_list": [
                    {
                        "interviewer_list": [
                            { "id": "7392444731470563625", "name": { "zh_cn": "张三", "en_us": "ZhangSan" } }
                        ],
                        "round": 1
                    }
                ],
                "job_requirement_list": [
                    { "id": "6949805467799537964", "name": { "zh_cn": "华中大区部门", "en_us": "JR Test" } }
                ],
                "interview_registration": { "schema_id": "6930815272790114324", "name": "默认登记表" },
                "onboard_registration": { "schema_id": "6930815272790114324", "name": "默认登记表" },
                "interview_round_type_list": [
                    {
                        "assessment_round": {
                            "id": "7392444731470563625",
                            "name": { "zh_cn": "测试类型", "en_us": "Test type" }
                        },
                        "assessment_template": {
                            "id": "7392444731470563625",
                            "name": { "zh_cn": "测试评价表", "en_us": "Test Name" }
                        }
                    }
                ],
                "related_job_list": [
                    { "id": "7392444731470563625", "name": { "zh_cn": "职位1", "en_us": "job1" } }
                ],
                "job_attribute": 1,
                "interview_appointment_config": {
                    "enable_interview_appointment_by_interviewer": true,
                    "config": {
                        "interview_type": 1,
                        "talent_timezone_code": "Asia/Shanghai",
                        "contact_user_id": "ou_c99c5f35d542efc7ee492afe11af19ef",
                        "contact_mobile": "177xxxx1773",
                        "contact_email": "test@open.com",
                        "address_id": "6960663240925956576",
                        "video_type": 1,
                        "cc": ["ou_c99c5f35d542efc7ee492afe11af19ef"],
                        "remark": "这个职位最好选择现场牛客网面试",
                        "interview_notification_template_id": "6960663240925956573",
                        "appointment_notification_template_id": "6960663240925956573",
                        "cancel_interview_notification_template_id": "6960663240925956573"
                    }
                },
                "portal_website_apply_form_schema_info": {
                    "schema_id": "6930815272790114324",
                    "name": "默认申请表"
                }
            }
        }))
        .expect("官方示例应能反序列化");

        let job_config = resp.job_config.as_ref().expect("job_config 应被类型化解析");
        assert_eq!(
            job_config
                .offer_apply_schema
                .as_ref()
                .and_then(|s| s.id.as_deref()),
            Some("7392444731470563625")
        );
        assert_eq!(job_config.id.as_deref(), Some("6960663240925956574"));
        assert_eq!(job_config.job_attribute, Some(1));
        assert_eq!(
            job_config
                .recommended_evaluator_list
                .as_ref()
                .map(|l| l.len()),
            Some(1)
        );
        assert_eq!(
            job_config
                .interview_round_list
                .as_ref()
                .and_then(|l| l.first())
                .and_then(|r| r.round),
            Some(1)
        );
        assert_eq!(
            job_config
                .interview_registration
                .as_ref()
                .and_then(|r| r.name.as_deref()),
            Some("默认登记表")
        );
    }
}
