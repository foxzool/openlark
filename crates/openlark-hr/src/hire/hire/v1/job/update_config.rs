//! 更新职位设置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/update_config>

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

/// 更新职位设置请求
#[derive(Debug, Clone)]
pub struct UpdateConfigRequest {
    job_id: String,
    request_body: UpdateConfigRequestBody,
    /// 配置信息
    config: Config,
}

impl UpdateConfigRequest {
    /// 创建请求
    pub fn new(config: Config, job_id: String, request_body: UpdateConfigRequestBody) -> Self {
        Self {
            job_id,
            request_body,
            config,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: UpdateConfigRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateConfigResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateConfigResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.job_id.trim(), "职位 ID 不能为空");
        self.request_body.validate()?;

        let api_endpoint = HireApiV1::JobUpdateConfig(self.job_id);
        let request = ApiRequest::<UpdateConfigResponse>::post(api_endpoint.to_url());
        let request = request.body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新职位设置响应数据为空",
        )
        .await
    }
}

/// `UpdateConfigRequestBody`。
///
/// 接口按 `update_option_list` 所选配置项做参数校验与更新；
/// 所选配置项对应的必填字段未填时接口会报错。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateConfigRequestBody {
    /// Offer 申请表 ID，可通过获取 Offer 申请表列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_apply_schema_id: Option<String>,
    /// Offer 审批流程 ID，可通过获取 Offer 审批流配置列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_process_conf: Option<String>,
    /// 建议评估人 ID 列表，需与 `user_id_type` 查询参数类型一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_evaluator_id_list: Option<Vec<String>>,
    /// 更新选项（必填）：1 更新面试评价表 / 2 更新 Offer 申请表 / 3 更新 Offer 审批流程 /
    /// 4 更新招聘需求 / 5 更新建议面试官 / 6 更新建议评估人 / 8 更新关联职位 /
    /// 9 更新自助约面配置 / 10 更新面试登记表 / 11 更新入职登记表 / 12 更新官网申请表。
    pub update_option_list: Vec<i32>,
    /// 面试评价表 ID，可通过获取面试评价表列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_template_biz_id: Option<String>,
    /// 建议面试官列表，当 `update_option_list` 包含「更新建议面试官」时必填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_round_conf_list: Option<Vec<JobConfigInterviewRoundConf>>,
    /// 关联招聘需求 ID 列表，可通过获取招聘需求信息接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jr_id_list: Option<Vec<String>>,
    /// 面试登记表 ID，可通过获取面试登记表模板列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_registration_schema_id: Option<String>,
    /// 入职登记表 ID，可通过获取信息登记表模板列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboard_registration_schema_id: Option<String>,
    /// 面试轮次类型配置列表，当「更新面试评价表」且启用面试轮次类型时必填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_round_type_conf_list: Option<Vec<JobConfigRoundType>>,
    /// 关联职位 ID 列表：实体职位关联虚拟职位 ID，虚拟职位关联实体职位 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_job_id_list: Option<Vec<String>>,
    /// 自助约面配置，当 `update_option_list` 包含「更新自助约面配置」时必填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_appointment_config: Option<InterviewAppointmentConfig>,
    /// 官网申请表 ID，可通过获取招聘官网申请表模板列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_website_apply_form_schema_id: Option<String>,
}

impl UpdateConfigRequestBody {
    /// 创建新的请求实例（`update_option_list` 为官方必填字段）。
    pub fn new(update_option_list: Vec<i32>) -> Self {
        Self {
            offer_apply_schema_id: None,
            offer_process_conf: None,
            recommended_evaluator_id_list: None,
            update_option_list,
            assessment_template_biz_id: None,
            interview_round_conf_list: None,
            jr_id_list: None,
            interview_registration_schema_id: None,
            onboard_registration_schema_id: None,
            interview_round_type_conf_list: None,
            related_job_id_list: None,
            interview_appointment_config: None,
            portal_website_apply_form_schema_id: None,
        }
    }

    /// 设置 Offer 申请表 ID。
    pub fn offer_apply_schema_id(mut self, value: impl Into<String>) -> Self {
        self.offer_apply_schema_id = Some(value.into());
        self
    }

    /// 设置 Offer 审批流程 ID。
    pub fn offer_process_conf(mut self, value: impl Into<String>) -> Self {
        self.offer_process_conf = Some(value.into());
        self
    }

    /// 设置建议评估人 ID 列表。
    pub fn recommended_evaluator_id_list(mut self, value: Vec<String>) -> Self {
        self.recommended_evaluator_id_list = Some(value);
        self
    }

    /// 设置面试评价表 ID。
    pub fn assessment_template_biz_id(mut self, value: impl Into<String>) -> Self {
        self.assessment_template_biz_id = Some(value.into());
        self
    }

    /// 设置建议面试官列表。
    pub fn interview_round_conf_list(mut self, value: Vec<JobConfigInterviewRoundConf>) -> Self {
        self.interview_round_conf_list = Some(value);
        self
    }

    /// 设置关联招聘需求 ID 列表。
    pub fn jr_id_list(mut self, value: Vec<String>) -> Self {
        self.jr_id_list = Some(value);
        self
    }

    /// 设置面试登记表 ID。
    pub fn interview_registration_schema_id(mut self, value: impl Into<String>) -> Self {
        self.interview_registration_schema_id = Some(value.into());
        self
    }

    /// 设置入职登记表 ID。
    pub fn onboard_registration_schema_id(mut self, value: impl Into<String>) -> Self {
        self.onboard_registration_schema_id = Some(value.into());
        self
    }

    /// 设置面试轮次类型配置列表。
    pub fn interview_round_type_conf_list(mut self, value: Vec<JobConfigRoundType>) -> Self {
        self.interview_round_type_conf_list = Some(value);
        self
    }

    /// 设置关联职位 ID 列表。
    pub fn related_job_id_list(mut self, value: Vec<String>) -> Self {
        self.related_job_id_list = Some(value);
        self
    }

    /// 设置自助约面配置。
    pub fn interview_appointment_config(mut self, value: InterviewAppointmentConfig) -> Self {
        self.interview_appointment_config = Some(value);
        self
    }

    /// 设置官网申请表 ID。
    pub fn portal_website_apply_form_schema_id(mut self, value: impl Into<String>) -> Self {
        self.portal_website_apply_form_schema_id = Some(value.into());
        self
    }

    fn validate(&self) -> SDKResult<()> {
        validate_required!(
            self.update_option_list,
            "update_option_list 不能为空且需传入要更新的配置项"
        );

        Ok(())
    }
}

/// 建议面试官轮次配置（`job_config_interview_round_conf`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobConfigInterviewRoundConf {
    /// 建议面试官 ID 列表，需与 `user_id_type` 查询参数类型一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interviewer_id_list: Option<Vec<String>>,
    /// 面试轮次。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round: Option<i32>,
}

/// 面试轮次类型配置（`job_config_round_type`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobConfigRoundType {
    /// 面试轮次类型 ID，可通过获取面试轮次类型列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub round_biz_id: Option<String>,
    /// 面试评价表 ID，可通过获取面试评价表列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_template_biz_id: Option<String>,
}

/// 自助约面配置（`interview_appointment_config`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewAppointmentConfig {
    /// 是否开启面试官自助约面。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_interview_appointment_by_interviewer: Option<bool>,
    /// 配置详情。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<InterviewAppointmentConfigContent>,
}

/// 自助约面配置详情（`interview_appointment_config_content`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewAppointmentConfigContent {
    /// 面试类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_type: Option<i32>,
    /// 候选人时区。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_timezone_code: Option<String>,
    /// 面试联系人 ID，需与 `user_id_type` 查询参数类型一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_user_id: Option<String>,
    /// 面试联系人电话。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_mobile: Option<String>,
    /// 面试联系人邮箱。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    /// 面试地点 ID，可通过获取地址列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// 视频面试类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_type: Option<i32>,
    /// 抄送人 ID 列表，需与 `user_id_type` 查询参数类型一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    /// 面试配置备注。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 面试通知模板 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_notification_template_id: Option<String>,
    /// 预约通知模板 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointment_notification_template_id: Option<String>,
    /// 取消面试通知模板 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_interview_notification_template_id: Option<String>,
}

/// 更新职位设置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateConfigResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for UpdateConfigResponse {
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

    /// 端到端：POST /open-apis/hire/v1/jobs/test001/update_config
    #[tokio::test]
    async fn test_update_config_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/jobs/test001/update_config"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "job_id": "test001", "success": true }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = UpdateConfigRequestBody::new(vec![1, 5])
            .assessment_template_biz_id("6960663240925956571")
            .interview_round_conf_list(vec![JobConfigInterviewRoundConf {
                interviewer_id_list: Some(vec!["6960663240925956571".to_string()]),
                round: Some(1),
            }])
            .interview_appointment_config(InterviewAppointmentConfig {
                enable_interview_appointment_by_interviewer: Some(true),
                config: Some(InterviewAppointmentConfigContent {
                    interview_type: Some(1),
                    talent_timezone_code: Some("Asia/Shanghai".to_string()),
                    ..InterviewAppointmentConfigContent::default()
                }),
            });

        let resp = UpdateConfigRequest::new(config, "test001".to_string(), body)
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(resp.job_id.as_deref(), Some("test001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let wire_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(
            wire_body,
            json!({
                "update_option_list": [1, 5],
                "assessment_template_biz_id": "6960663240925956571",
                "interview_round_conf_list": [
                    { "interviewer_id_list": ["6960663240925956571"], "round": 1 }
                ],
                "interview_appointment_config": {
                    "enable_interview_appointment_by_interviewer": true,
                    "config": { "interview_type": 1, "talent_timezone_code": "Asia/Shanghai" }
                }
            })
        );
    }

    /// `update_option_list` 为空时应快速失败，不发起请求。
    #[tokio::test]
    async fn test_update_config_rejects_empty_update_options() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let result = UpdateConfigRequest::new(
            config,
            "test001".to_string(),
            UpdateConfigRequestBody::new(Vec::new()),
        )
        .execute()
        .await;
        assert!(result.is_err());
    }
}
