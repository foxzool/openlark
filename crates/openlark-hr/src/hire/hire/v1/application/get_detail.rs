//! 获取投递详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/application/get_detail>

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
use crate::hire::hire::v1::job::get_detail::JobDetailNamedObject;

/// 投递阶段信息（官方 `application_stage_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationStageInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递阶段 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递阶段中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递阶段英文名称。
    pub en_name: Option<String>,
    /// 阶段类型：1 筛选型 / 2 评估型 / 3 笔试型 / 4 面试型 / 5 Offer 型 / 6 待入职 / 7 已入职 / 8 其它 / 255 系统默认。
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub stage_type: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递来源信息（官方 `application_resume_source`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationResumeSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递来源 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递来源名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递来源类型：10000 内推 / 10001 猎头 / 10002 内部来源 等。
    pub resume_source_type: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 官网推广渠道（官方 `application_website_channel`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationWebsiteChannel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推广渠道 ID。
    pub channel_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推广渠道名称。
    pub channel_name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 官网投递来源（官方 `application_website_resume_source`，当从官网进行投递时返回）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationWebsiteResumeSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网站点 ID。
    pub website_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网站点名称。
    pub website_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推广渠道来源。
    pub channel: Option<ApplicationWebsiteChannel>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递阶段变更时间（官方 `application_stage_time`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationStageTime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阶段 ID。
    pub stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最近一次进入该阶段的时间（单位：毫秒）。
    pub enter_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最后一次离开时间（单位：毫秒），如当前在该阶段，则为空。
    pub exit_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 终止原因信息（官方 `termination_reason_info`，children 递归同构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TerminationReasonInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 终止原因 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 终止原因名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 子级终止原因。
    pub children: Option<Vec<TerminationReasonInfo>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递基本信息（官方 `application_detail_basic_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationDetailBasicInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人才 ID。
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递处于的阶段。
    pub stage: Option<ApplicationStageInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递活跃状态：1 活跃 / 2 非活跃。
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递方式：1 HR 寻访 / 2 候选人主动投递。
    pub delivery_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递来源信息。
    pub resume_source_info: Option<ApplicationResumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网投递来源（当从官网进行投递时返回）。
    pub website_resume_source: Option<ApplicationWebsiteResumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 简历附件 ID。
    pub talent_attachment_resume_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递阶段变更时间列表。
    pub stage_time_list: Option<Vec<ApplicationStageTime>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递入职状态：1 未入职 / 2 已入职。
    pub onboard_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 意向投递城市列表。
    pub application_preferred_city_list: Option<Vec<CodeNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递终止原因。
    pub termination_reason: Option<TerminationReasonInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递创建者 ID，与入参 `user_id_type` 类型一致。
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递所有者 ID，与入参 `user_id_type` 类型一致。
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递终止者 ID，与入参 `user_id_type` 类型一致。
    pub terminator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间戳（单位：毫秒）。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 修改时间戳（单位：毫秒）。
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 锁定状态：1 无锁 / 2 被其他投递锁定 / 3 被当前投递锁定。
    pub lock_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 终止备注。
    pub termination_reason_note: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递职位信息（官方 `job_basic_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationDetailJobInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位编码。
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位描述。
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位要求。
    pub requirement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 雇佣类型。
    pub recruitment_type: Option<JobDetailNamedObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门。
    pub department: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位流程类型：1 社招流程 / 2 校招流程。
    pub process_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位流程 ID。
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作地点。
    pub city_list: Option<CodeNameObject>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递人才信息（官方 `talent_basic_info_v2`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentBasicInfoV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人才 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人才名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人才手机国家区号。
    pub mobile_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人才手机号。
    pub mobile_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人才邮箱。
    pub email: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递评估信息（官方 `evaluation`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationEvaluation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 评估 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递阶段 ID。
    pub stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建人用户 ID，与入参 `user_id_type` 类型一致。
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 评估人用户 ID，与入参 `user_id_type` 类型一致。
    pub evaluator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 提交状态：1 已提交 / 2 未提交。
    pub commit_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 评估结论：1 通过 / 2 未通过。
    pub conclusion: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 评估详情。
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 评估创建时间戳（单位：毫秒）。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 评估最近更新时间戳（单位：毫秒）。
    pub update_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 基础用户信息（官方 `basic_user_info`：`{id, name}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BasicUserInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 用户 ID，与入参 `user_id_type` 类型一致。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试评价附件（官方 `interview_attachment`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewAttachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附件 ID。
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附件名称。
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附件类型。
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附件创建时间戳（单位：毫秒）。
    pub create_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试评价分数（官方 `record_score`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RecordScore {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价得分（精确到小数点后两位）。
    pub score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价总分（精确到小数点后两位）。
    pub total_score: Option<f64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 维度选项（官方 `dimension_option`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DimensionOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项对应的分数。
    pub score_val: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项全称。
    pub alias_name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职级建议（官方 `recommended_job_level`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RecommendedJobLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低职级建议。
    pub lower_limit_job_level_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高职级建议。
    pub higher_limit_job_level_name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 关联面试题（官方 `question_assessment`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QuestionAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 所关联面试题的类型：1 普通面试题目 / 2 在线编程题目。
    pub question_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 关联面试题的名称。
    pub title: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 关联面试题的描述。
    pub description: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试者作答内容。
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 能力项列表。
    pub abilities: Option<Vec<Ability>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 能力项（官方 `ability`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Ability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 能力项 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 能力项名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 能力项描述。
    pub description: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 能力项评价（官方 `ability_assessment`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AbilityAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 能力项评价 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 能力项 ID。
    pub ability_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 能力项评价内容。
    pub content: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 模块评价（官方 `dimension_assessment`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DimensionAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 对应模版中维度 ID。
    pub interview_feedback_form_dimension_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 维度名称。
    pub dimension_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 维度类型：1 单选题 / 2 多选题 / 3 描述题 / 5 职级建议 / 6 打分题(单选) / 7 打分题(填空) / 10-12 系统预置。
    pub dimension_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 维度权重。
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当维度类型为描述题时，从此取值。
    pub dimension_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当维度类型为单选题时，从此取值。
    pub dimension_option: Option<DimensionOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当维度类型为多选题时，从此取值。
    pub dimension_options: Option<Vec<DimensionOption>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当维度评价方式为「打分题(填空)」时，从此取值。
    pub dimension_score: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当维度为「职级建议」时，从此取值。
    pub recommended_job_level: Option<RecommendedJobLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 维度关联面试题。
    pub question_assessments: Option<Vec<QuestionAssessment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 能力项评价。
    pub ability_assessments: Option<Vec<AbilityAssessment>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 模块评价（官方 `module_assessment`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ModuleAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表模块 ID。
    pub interview_feedback_form_module_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块名称。
    pub module_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块类型：1 系统预置「面试结论」模块 / 2 自定义模块。
    pub module_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块权重。
    pub module_weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块打分（精确到小数点后两位）。
    pub module_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块评价。
    pub dimension_assessments: Option<Vec<DimensionAssessment>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试评价信息（官方 `interview_record_v2`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRecordV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表 ID。
    pub feedback_form_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 提交状态：1 已提交 / 2 未提交。
    pub commit_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价提交时间戳（单位：毫秒）。
    pub submit_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价分数。
    pub record_score: Option<RecordScore>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试官信息。
    pub interviewer: Option<BasicUserInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价附件列表。
    pub attachments: Option<Vec<InterviewAttachment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块评价列表。
    pub module_assessments: Option<Vec<ModuleAssessment>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试会议室（官方 `interview_meeting_room`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewMeetingRoom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 会议室 ID。
    pub room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 会议室名称。
    pub room_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 建筑名称。
    pub building_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 会议室预定状态：1 预约中 / 2 预约成功 / 3 预约失败。
    pub reserved_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 楼层。
    pub floor_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 基础区域信息（官方 `base_location`：`{name, code, location_type}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BaseLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 编码。
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 地址类型：1 国家 / 2 省 / 3 市 / 4 区 / 5 地址。
    pub location_type: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 基础地址（官方 `base_address_v2`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BaseAddressV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 地点 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 地点名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 区域信息。
    pub district: Option<BaseLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 城市信息。
    pub city: Option<BaseLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 省信息。
    pub state: Option<BaseLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 国家信息。
    pub country: Option<BaseLocation>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试信息（官方 `interview_extend_v2`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewExtendV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试开始时间戳（单位：毫秒）。
    pub begin_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试结束时间戳（单位：毫秒）。
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次（从 0 开始计数）。
    pub round: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价信息列表。
    pub interview_record_list: Option<Vec<InterviewRecordV2>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价提交时间戳（单位：毫秒）。
    pub feedback_submit_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试关联的投递阶段。
    pub stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阶段信息。
    pub stage: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建人。
    pub creator: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间戳（单位：毫秒）。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 更新时间戳（单位：毫秒）。
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试状态。
    pub interview_round_summary: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试安排 ID。
    pub interview_arrangement_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试类型：1 现场面试 / 2 电话面试 / 3 视频面试。
    pub interview_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 候选人时区。
    pub talent_time_zone: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试联系人。
    pub contact_user: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试联系人电话。
    pub contact_mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 备注。
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试地点。
    pub address: Option<BaseAddressV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 视频面试工具。
    pub video_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 集中面试的安排状态：1 未开始 / 2 进行中 / 3 已结束 / 4 已取消。
    pub arrangement_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 安排类型：1 社招单面 / 2 集中面试 / 3 集体面试。
    pub arrangement_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 安排方式：1 直接安排 / 2 自助约面。
    pub arrangement_appointment_kind: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试会议室列表。
    pub meeting_room_list: Option<Vec<InterviewMeetingRoom>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次类型。
    pub interview_round_type: Option<IdNameObject>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递面试信息（官方 `interview_aggregation`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewAggregation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试信息列表。
    pub interviews: Option<Vec<InterviewExtendV2>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// Offer 附件（官方 `offer_attachment_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferAttachmentInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 附件 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 附件名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 附件大小。
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 通用附件 ID。
    pub common_attachment_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// Offer 自定义字段数据（官方 `application_offer_custom_value`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationOfferCustomValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 自定义字段 ID。
    pub object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 自定义字段 Value。
    pub customize_value: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 合同期（官方 `contract_period_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ContractPeriodInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 合同周期类型：1 月 / 2 年。
    pub period_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 合同时长。
    pub period: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// Offer 基本信息（官方 `offer_basic_info_v2`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferBasicInfoV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 状态。
    pub offer_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 直属上级。
    pub leader: Option<BasicUserInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人员类型。
    pub employee_type: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门。
    pub department: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 序列。
    pub sequence: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 级别。
    pub level: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 公司主体。
    pub company_main_body: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘需求 ID。
    pub job_requirement_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 试用期（单位：月）。
    pub probation_month: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 合同期（年/月）。
    pub contract_period: Option<ContractPeriodInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 入职日期，格式为 YYYY-MM-DD。
    pub onboard_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 负责人。
    pub owner: Option<BasicUserInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 入职地址。
    pub onboard_address: Option<BaseAddressV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作地址。
    pub work_address: Option<BaseAddressV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 备注。
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附件列表。
    pub attachment_list: Option<Vec<OfferAttachmentInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 自定义字段数据。
    pub customize_info_list: Option<Vec<ApplicationOfferCustomValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 创建时间戳（单位：毫秒）。
    pub create_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// Offer 薪酬信息（官方 `offer_salary_info_v2`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferSalaryInfoV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 薪酬 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 薪酬状态。
    pub salary_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 试用期百分比。
    pub probation_salary_percentage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 年终奖月数。
    pub award_salary_multiple: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 期权股数。
    pub option_shares: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 季度奖金额，单位元。
    pub quarterly_bonus: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 半年奖金额，单位元。
    pub half_year_bonus: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 年度现金总额（数量，非公式），单位元。
    pub total_annual_cash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 薪酬自定义字段。
    pub customize_info_list: Option<Vec<ApplicationOfferCustomValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 薪酬创建时间戳（单位：毫秒）。
    pub create_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递 Offer 信息（官方 `offer_info_v2`，正式 Offer）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferInfoV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 基本信息。
    pub offer_basic: Option<OfferBasicInfoV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 薪酬信息。
    pub offer_salary: Option<OfferSalaryInfoV2>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递员工入转离信息（官方 `employee_v2`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EmployeeV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 员工 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 入职状态：1 已入职 / 2 已离职。
    pub onboard_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 转正状态：1 未转正 / 2 已转正。
    pub conversion_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 实际入职时间戳（单位：毫秒）。
    pub onboard_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 预期转正时间戳（单位：毫秒）。
    pub expected_conversion_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 实际转正时间戳（单位：毫秒）。
    pub actual_conversion_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 离职时间戳（单位：毫秒）。
    pub overboard_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 离职原因。
    pub overboard_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 办公地点。
    pub onboard_city_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 入职部门 ID，与入参 `department_id_type` 类型一致。
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 直属上级 ID，与入参 `user_id_type` 类型一致。
    pub leader_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 序列 ID，与入参 `job_family_id_type` 类型一致。
    pub sequence_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职级 ID，与入参 `job_level_id_type` 类型一致。
    pub level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人员类型，与入参 `employee_type_id_type` 类型一致。
    pub employee_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘需求 ID。
    pub job_requirement_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 猎头基本信息（官方 `agency_basic_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencyBasicInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 猎头用户名。
    pub hunter_user_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 猎头公司名。
    pub hunter_company_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 猎头自定义数据（官方 `report_custom_data`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReportCustomData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 内容。
    pub value: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 描述。
    pub description: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递猎头推荐信息（官方 `agency_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencyInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 猎头基本信息。
    pub basic_info: Option<AgencyBasicInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 猎头评价信息。
    pub comment_info: Option<Vec<ReportCustomData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 薪酬信息。
    pub salary_info: Option<Vec<ReportCustomData>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 校招志愿信息（官方 `campus_volumnteer_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CampusVolunteerInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 志愿顺序。
    pub volunteer_seq: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递官网信息（官方 `application_detail_portal_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationDetailPortalInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 校招志愿信息。
    pub campus_volunteer_info: Option<CampusVolunteerInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 内推基本信息（官方 `referral_basic_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralBasicInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 内推 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 内推类型：1 意向推荐 / 2 职位推荐。
    pub referral_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 内推人信息。
    pub user_info: Option<BasicUserInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 内推创建时间戳（单位：毫秒）。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 内推方法：1 正常内推 / 2 内推码 / 3 特别内推码 / 4 H5 内推 / 5 HR 代为内推。
    pub referral_method: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 特殊关系（官方 `specific_relationship`）。
///
/// 注意：官方 `extra` 字段为 string 类型的附加信息，非扩展字段容器。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SpecificRelationship {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 与候选人的关系：1 前同事 / 2 朋友 / 3 其他。
    pub relation_with_candidate: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附加信息（官方字段名即 `extra`，string 类型）。
    pub extra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作能力熟悉程度：1 非常熟悉 / 2 比较熟悉 / 3 一般熟悉 / 4 不熟悉。
    pub work_ability_familiarity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 匹配度：1 非常匹配 / 2 比较匹配 / 3 一般匹配 / 4 不匹配。
    pub match_degree: Option<i32>,
}

/// 内推关联推荐信息（官方 `referral_recommend_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralRecommendInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 与内推人关系：0 无关系 / 1 直系亲属。
    pub relationship: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 熟悉程度：0 不熟悉 / 1 熟悉。
    pub familiarity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推荐语。
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 特殊关系。
    pub specific_relationship: Option<SpecificRelationship>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作能力熟悉程度。
    pub work_ability_familiarity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 匹配度。
    pub match_degree: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递内推信息（官方 `referral_info_v2`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralInfoV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 内推基本信息。
    pub basic_info: Option<ReferralBasicInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 内推关联推荐信息。
    pub recommend_info: Option<ReferralRecommendInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递详情（官方 `application_detail_info`，获取投递详情响应 data 下唯一容器）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationDetailInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递基本信息。
    pub basic_info: Option<ApplicationDetailBasicInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递职位信息。
    pub job: Option<ApplicationDetailJobInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递人才信息。
    pub talent: Option<TalentBasicInfoV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递评估信息列表。
    pub evaluations: Option<Vec<ApplicationEvaluation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递面试信息。
    pub interview_aggregation: Option<InterviewAggregation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 Offer 信息（正式 Offer）。
    pub offer: Option<OfferInfoV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递员工入转离信息。
    pub employee: Option<EmployeeV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递猎头推荐信息。
    pub agency: Option<AgencyInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递官网信息。
    pub portal: Option<ApplicationDetailPortalInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递内推信息。
    pub referral: Option<ReferralInfoV2>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取投递详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetDetailResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `application_detail` 字段（官方响应 data 下唯一容器）。
    pub application_detail: Option<ApplicationDetailInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取投递详情请求
#[derive(Debug, Clone)]
pub struct GetDetailRequest {
    /// 配置信息
    config: Config,
    application_id: String,
}

impl GetDetailRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
        }
    }

    /// 设置 `application_id`。
    pub fn application_id(mut self, application_id: String) -> Self {
        self.application_id = application_id;
        self
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
        validate_required!(self.application_id.trim(), "投递 ID 不能为空");

        let request = ApiRequest::<GetDetailResponse>::get(format!(
            "/open-apis/hire/v1/applications/{}/get_detail",
            self.application_id
        ));
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取投递详情响应数据为空",
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

    /// 端到端：GET /open-apis/hire/v1/applications/test001/get_detail
    #[tokio::test]
    async fn test_get_detail_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/applications/test001/get_detail"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "application_detail": {
                        "basic_info": {
                            "id": "6949805467799537964",
                            "job_id": "6843547872837273223",
                            "talent_id": "6843547872837273223",
                            "stage": {
                                "id": "614218419274131",
                                "zh_name": "面试",
                                "en_name": "Interview",
                                "type": 1
                            },
                            "active_status": 1,
                            "delivery_type": 1,
                            "resume_source_info": {
                                "id": "614218419274131",
                                "name": { "zh_cn": "内推", "en_us": "Referral" },
                                "resume_source_type": 10002
                            }
                        },
                        "job": {
                            "id": "6956499586395523359",
                            "name": "后端研发工程师",
                            "code": "A75256",
                            "description": "职位描述文本",
                            "requirement": "职位要求文本",
                            "recruitment_type": {
                                "id": "6001",
                                "name": { "zh_cn": "测试", "en_us": "test" },
                                "active_status": 1
                            },
                            "department": {
                                "id": "od-xxxx",
                                "name": { "zh_cn": "测试", "en_us": "test" }
                            },
                            "process_type": 1,
                            "process_id": "1",
                            "city_list": {
                                "code": "1111",
                                "name": { "zh_cn": "测试", "en_us": "test" }
                            }
                        },
                        "talent": {
                            "id": "6956499586395523359",
                            "name": "小明",
                            "mobile_code": "+86",
                            "mobile_number": "13563350751",
                            "email": "xxx@bytedance.com"
                        },
                        "evaluations": [
                            {
                                "id": "6875295756292425998",
                                "application_id": "6875569957036738823",
                                "stage_id": "6784315427607595268",
                                "creator_id": "ou_aaf83d1b2c856ead36aa9a38784b9a5c",
                                "evaluator_id": "ou_aaf83d1b2c856ead36aa9a38784b9a5c",
                                "commit_status": 1,
                                "conclusion": 1,
                                "content": "这位同学很优秀",
                                "create_time": "1600843767338",
                                "update_time": "1600843937733"
                            }
                        ],
                        "employee": {
                            "id": "123",
                            "application_id": "6990656563279480364",
                            "onboard_status": 1,
                            "conversion_status": 1
                        }
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

        let resp = GetDetailRequest::new(config)
            .application_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        // 官方响应 data.application_detail 容器（.md L49-499）
        let detail = resp
            .application_detail
            .as_ref()
            .expect("application_detail 应反序列化");
        let basic = detail.basic_info.as_ref().expect("basic_info 应反序列化");
        assert_eq!(basic.id.as_deref(), Some("6949805467799537964"));
        assert_eq!(basic.delivery_type, Some(1));
        assert_eq!(
            basic.stage.as_ref().and_then(|v| v.zh_name.as_deref()),
            Some("面试")
        );
        assert_eq!(
            basic
                .resume_source_info
                .as_ref()
                .and_then(|v| v.resume_source_type),
            Some(10002)
        );
        assert_eq!(
            detail.job.as_ref().and_then(|v| v.name.as_deref()),
            Some("后端研发工程师")
        );
        assert_eq!(
            detail.talent.as_ref().and_then(|v| v.name.as_deref()),
            Some("小明")
        );
        assert_eq!(
            detail
                .evaluations
                .as_ref()
                .and_then(|v| v.first())
                .and_then(|v| v.conclusion),
            Some(1)
        );
        assert_eq!(
            detail.employee.as_ref().and_then(|v| v.onboard_status),
            Some(1)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
