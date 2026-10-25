//! 获取职位详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/get_detail>

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
use crate::hire::hire::common_models::JobCustomizedData;

/// 职位详情命名对象（官方 `job_detail_recruitment_type` / `job_detail_level` /
/// `job_detail_category` / `job_detail_type`：`{id, name, active_status}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobDetailNamedObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 启用状态：1 启用 / 2 未启用。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位基本信息（官方 `job_detail_basic_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobDetailBasicInfo {
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
    /// 职位雇佣类型。
    pub recruitment_type: Option<JobDetailNamedObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位部门。
    pub department: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低职级。
    pub min_job_level: Option<JobDetailNamedObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高职级。
    pub max_job_level: Option<JobDetailNamedObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位亮点列表。
    pub highlight_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列。
    pub job_category: Option<JobDetailNamedObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别。
    pub job_type: Option<JobDetailNamedObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位状态：1 启用 / 2 关闭。
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建人 ID，与入参 `user_id_type` 类型一致。
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间，毫秒时间戳。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 更新时间，毫秒时间戳。
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位流程类型：1 社招流程 / 2 校招流程。
    pub process_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位流程 ID。
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位流程名称。
    pub process_name: Option<I18nText>,
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
    /// 到期日期，毫秒时间戳。
    pub expiry_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 月薪范围-最低薪资，单位：千。
    pub min_salary: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 月薪范围-最高薪资，单位：千。
    pub max_salary: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 学历要求。
    pub required_degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作地点列表。
    pub city_list: Option<Vec<CodeNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位属性：1 实体职位 / 2 虚拟职位。
    pub job_attribute: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 目标专业列表。
    pub target_major_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否门店职位：1 普通职位 / 2 门店职位。
    pub storefront_mode: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位人员信息（官方 `job_user_info`：`{id, name}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobUserInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人员 ID，与入参 `user_id_type` 类型一致。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘需求简要信息（官方 `job_requirement_simple`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRequirementSimple {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘需求 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘需求编号。
    pub short_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 需求名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 需求部门 ID，与入参 `department_id_type` 类型一致。
    pub department_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 通用地址（官方 `common_address`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CommonAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 地址 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 地址名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 区域信息。
    pub district: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 城市信息。
    pub city: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 省信息。
    pub state: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 国家信息。
    pub country: Option<CodeNameObject>,
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
pub struct JobConfigRoundType {
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

/// 面试官安排面试配置详情（官方 `interview_appointment_config_content`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewAppointmentConfigContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试类型：1 现场面试 / 2 电话面试 / 3 视频面试。
    pub interview_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 时区。
    pub talent_timezone_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 联系人 ID，与入参 `user_id_type` 类型一致。
    pub contact_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 联系人电话。
    pub contact_mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 联系人邮箱。
    pub contact_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 联系人地址 ID。
    pub address_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 视频面试类型。
    pub video_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 抄送人 ID 列表，与入参 `user_id_type` 类型一致。
    pub cc: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试配置备注。
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试通知模板 ID。
    pub interview_notification_template_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 预约通知模板 ID。
    pub appointment_notification_template_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 取消面试通知模版 ID。
    pub cancel_interview_notification_template_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试官安排面试配置（官方 `interview_appointment_config`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewAppointmentConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否开启面试官安排面试。
    pub enable_interview_appointment_by_interviewer: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 配置详情。
    pub config: Option<InterviewAppointmentConfigContent>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位设置（官方 `job_config_detail`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobConfigDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 申请表。
    pub offer_apply_schema: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 审批流。
    pub offer_process_conf: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 建议评估人列表。
    pub recommended_evaluator_list: Option<Vec<IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表。
    pub assessment_template: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 建议面试官列表。
    pub interview_round_list: Option<Vec<JobConfigInterviewRound>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试登记表。
    pub interview_registration: Option<super::models::RegistrationSchemaInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 入职登记表。
    pub onboard_registration: Option<super::models::RegistrationSchemaInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次类型列表。
    pub interview_round_type_list: Option<Vec<JobConfigRoundType>>,
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
    /// 实习 Offer 申请表。
    pub internship_offer_apply_schema: Option<IdNameObject>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 门店（官方 `job_storefront`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobStorefront {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 门店 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 门店名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 启用状态：1 启用 / 2 停用。
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 门店部门。
    pub department: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 门店地址。
    pub address: Option<CommonAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 门店负责人。
    pub manager: Option<JobUserInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间，毫秒时间戳。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 门店备注。
    pub remark: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位标签（官方 `job_detail_tag`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobDetailTag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 标签 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 标签名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 标签顺序。
    pub order: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位详情（官方 `job_detail`，获取职位详情响应 data 下唯一容器）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobDetailInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位基本信息。
    pub basic_info: Option<JobDetailBasicInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位负责人。
    pub recruiter: Option<JobUserInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位协助人列表。
    pub assistant_list: Option<Vec<JobUserInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位用人经理列表。
    pub hiring_manager_list: Option<Vec<JobUserInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘需求列表。
    pub job_requirement_list: Option<Vec<JobRequirementSimple>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位地址列表。
    pub address_list: Option<Vec<CommonAddress>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位设置。
    pub job_config: Option<JobConfigDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 门店列表，仅当 `storefront_mode` 值为 `2` 的时候会填充。
    pub storefront_list: Option<Vec<JobStorefront>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位标签列表，根据标签顺序降序排列返回。
    pub tag_list: Option<Vec<JobDetailTag>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取职位详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetDetailResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `job_detail` 字段（官方响应 data 下唯一容器）。
    pub job_detail: Option<JobDetailInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取职位详情请求
#[derive(Debug, Clone)]
pub struct GetDetailRequest {
    job_id: String,
    /// 配置信息
    config: Config,
}

impl GetDetailRequest {
    /// 创建请求
    pub fn new(config: Config, job_id: String) -> Self {
        Self { job_id, config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetDetailResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetDetailResponse> {
        validate_required!(self.job_id.trim(), "职位 ID 不能为空");

        let request = ApiRequest::<GetDetailResponse>::get(format!(
            "/open-apis/hire/v1/jobs/{}/get_detail",
            self.job_id
        ));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取职位详情响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for GetDetailResponse {
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

    /// 端到端：GET /open-apis/hire/v1/jobs/test001/get_detail
    #[tokio::test]
    async fn test_get_detail_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/jobs/test001/get_detail"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "job_detail": {
                        "basic_info": {
                            "id": "7395082456917805339",
                            "title": "测试职位",
                            "description": "这是一个测试职位，仅仅用来测试。",
                            "code": "R18",
                            "requirement": "职位要求文本",
                            "recruitment_type": {
                                "id": "7395082456917805339",
                                "name": { "zh_cn": "全职", "en_us": "Full-time" },
                                "active_status": 1
                            },
                            "department": {
                                "id": "7395082456917805339",
                                "name": { "zh_cn": "飞书", "en_us": "Lark" }
                            },
                            "city_list": [
                                { "code": "CT_112", "name": { "zh_cn": "上海市", "en_us": "ShangHai" } }
                            ],
                            "storefront_mode": 1
                        },
                        "recruiter": {
                            "id": "ou_efk39117c300506837def50545420c6a",
                            "name": { "zh_cn": "张三", "en_us": "ZhangSan" }
                        },
                        "job_requirement_list": [
                            {
                                "id": "6949805467799537964",
                                "short_code": "2232",
                                "name": "华中大区部门",
                                "department_id": "6833685612520950030"
                            }
                        ],
                        "tag_list": [
                            { "id": "6949805467799537964", "name": { "zh_cn": "985、211", "en_us": "985、211" }, "order": 1 }
                        ]
                    }
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

        let resp = GetDetailRequest::new(config, "test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        // 官方响应 data.job_detail 容器（.md L48-320）
        let detail = resp.job_detail.as_ref().expect("job_detail 应反序列化");
        let basic = detail.basic_info.as_ref().expect("basic_info 应反序列化");
        assert_eq!(basic.id.as_deref(), Some("7395082456917805339"));
        assert_eq!(basic.title.as_deref(), Some("测试职位"));
        assert_eq!(basic.storefront_mode, Some(1));
        assert_eq!(
            detail.recruiter.as_ref().and_then(|v| v.id.as_deref()),
            Some("ou_efk39117c300506837def50545420c6a")
        );
        assert_eq!(
            detail
                .job_requirement_list
                .as_ref()
                .and_then(|v| v.first())
                .and_then(|v| v.short_code.as_deref()),
            Some("2232")
        );
        assert_eq!(
            detail
                .tag_list
                .as_ref()
                .and_then(|v| v.first())
                .and_then(|v| v.order),
            Some(1)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
