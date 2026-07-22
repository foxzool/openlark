//! Hire 业务域专属响应模型。
//!
//! 域无关的 HR 原语（`I18nText` / `FlexibleText` / `IdNameObject` /
//! `CodeNameObject` / `PaginatedResponse` / `CatalogItem` / `LocalizedLabel`）
//! 已迁至 crate root：[`crate::common::shared_models`]。本模块保留 hire 专属
//! 摘要类型（职位 / 投递 / 面试 / offer / 生态 / 猎头 等），并经 deprecated
//! re-export 再导出已迁移的原语，为既有全路径 import 留一个过渡周期（#473）。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

// 向后兼容 re-export：这 7 个域无关原语已迁至 crate root
// （`crate::common::shared_models`），此处按名再导出保留一个过渡周期，将在下个
// breaking 版本移除（#473）。功能上保证既有全路径 import 仍可解析（不破坏编译）。
// 注意：rustc 当前不会为 `pub use` 重导出在消费端触发 deprecation 告警——这里的
// `#[deprecated]` 主要作 rustdoc 标记 + 未来 Rust 版本转发兼容；alias 的实际移除
// 以下个 breaking 窗口的 grep（`common_models::(I18nText|FlexibleText|...)`）为准。
#[deprecated(note = "用 crate::common::shared_models，将在下个 breaking 版本移除")]
pub use crate::common::shared_models::{
    CatalogItem, CodeNameObject, FlexibleText, I18nText, IdNameObject, LocalizedLabel,
    PaginatedResponse,
};

/// 面试任务摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewTaskSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_id` 字段。
    pub interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_round_id` 字段。
    pub interview_round_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_round_name` 字段。
    pub interview_round_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interviewer` 字段。
    pub interviewer: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 候选人操作日志摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentOperationLogEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `operator` 字段。
    pub operator: Option<crate::common::shared_models::IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `operation_type` 字段。
    pub operation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `operation_time` 字段。
    pub operation_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 多元化附加信息摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DiversityInclusionRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 性别。
    pub gender: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位广告发布记录摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobPublishRecordSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `channel_id` 字段。
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `publish_status` 字段。
    pub publish_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 长尾接口常见的简单操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GenericOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `task_id` 字段。
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `publish_id` 字段。
    pub publish_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `exam_id` 字段。
    pub exam_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
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

/// 奖励金额。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BonusAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `point_bonus` 字段。
    pub point_bonus: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `cash_bonus` 字段。
    pub cash_bonus: Option<Vec<CashAmount>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 现金奖励金额。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CashAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `currency_type` 字段。
    pub currency_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `amount` 字段。
    pub amount: Option<f64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 附件元数据。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AttachmentMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `file_id` 字段。
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `file_name` 字段。
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `content_type` 字段。
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `file_size` 字段。
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `create_time` 字段。
    pub create_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 分数信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ScoreInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 分数。
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `total_score` 字段。
    pub total_score: Option<f64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘备注。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NoteRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `is_private` 字段。
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `create_time` 字段。
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `modify_time` 字段。
    pub modify_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `creator_id` 字段。
    pub creator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 内容。
    pub content: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘附件/文件句柄。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct HireAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `url` 字段。
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `mime` 字段。
    pub mime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `create_time` 字段。
    pub create_time: Option<i64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递所关联的职位摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationJobInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_name` 字段。
    pub job_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递所关联的候选人摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationTalentInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `talent_name` 字段。
    pub talent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `mobile` 字段。
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 邮箱地址。
    pub email: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递关联的 offer 摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationOfferInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_id` 字段。
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_status` 字段。
    pub offer_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递摘要信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_status` 字段。
    pub application_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_id` 字段。
    pub stage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_name` 字段。
    pub stage_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_info` 字段。
    pub job_info: Option<ApplicationJobInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递关联的面试记录摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationInterviewRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_id` 字段。
    pub interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_round_id` 字段。
    pub interview_round_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_round_name` 字段。
    pub interview_round_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interviewer` 字段。
    pub interviewer: Option<crate::common::shared_models::IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 分数。
    pub score: Option<ScoreInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位上的招聘相关人员。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRecruiterRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `recruiter_id` 字段。
    pub recruiter_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `manager_id` 字段。
    pub manager_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `user_id` 字段。
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `role` 字段。
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `role_type` 字段。
    pub role_type: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobSummary {
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
    /// `job_description` 字段。
    pub job_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `recruiters` 字段。
    pub recruiters: Option<Vec<JobRecruiterRecord>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位设置摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobConfigInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `process_id` 字段。
    pub process_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `process_name` 字段。
    pub process_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_requirement_schema_id` 字段。
    pub job_requirement_schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_registration_schema_id` 字段。
    pub interview_registration_schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_application_form_id` 字段。
    pub offer_application_form_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部投递摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalApplicationSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_application_id` 字段。
    pub external_application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `source_name` 字段。
    pub source_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部 Offer 摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalOfferSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_offer_id` 字段。
    pub external_offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_id` 字段。
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘官网推广渠道摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteChannelSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `channel_id` 字段。
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `website_id` 字段。
    pub website_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `code` 字段。
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘官网投递任务结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryTaskResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `delivery_task_id` 字段。
    pub delivery_task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `error_message` 字段。
    pub error_message: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘官网职位广告摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteJobPostSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_post_id` 字段。
    pub job_post_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `website_id` 字段。
    pub website_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标题。
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_channel_id` 字段。
    pub job_channel_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘官网站点用户摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteSiteUserSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `site_user_id` 字段。
    pub site_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `website_id` 字段。
    pub website_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `user_id` 字段。
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 邮箱地址。
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `mobile` 字段。
    pub mobile: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 猎头供应商摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencySummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `agency_id` 字段。
    pub agency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `code` 字段。
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 猎头供应商下的猎头账号摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencyAccountSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `agency_account_id` 字段。
    pub agency_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `agency_id` 字段。
    pub agency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `user_id` 字段。
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 猎头保护期摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencyProtectionSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `protection_id` 字段。
    pub protection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `agency_id` 字段。
    pub agency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `expiration_time` 字段。
    pub expiration_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 人才外部信息摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentExternalInfoRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_info_id` 字段。
    pub external_info_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `source_name` 字段。
    pub source_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_id` 字段。
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘需求摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRequirementSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_requirement_id` 字段。
    pub job_requirement_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标题。
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递操作结果摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_id` 字段。
    pub stage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_name` 字段。
    pub stage_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_id` 字段。
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `employee_id` 字段。
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `onboard_status` 字段。
    pub onboard_status: Option<i32>,
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

/// 外部背调摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalBackgroundCheckSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_background_check_id` 字段。
    pub external_background_check_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `vendor_name` 字段。
    pub vendor_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部面试摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalInterviewSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_interview_id` 字段。
    pub external_interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_round_name` 字段。
    pub interview_round_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 开始时间。
    pub start_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 三方协议摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TripartiteAgreementSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `agreement_id` 字段。
    pub agreement_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `sign_status` 字段。
    pub sign_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 生态自定义字段摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoCustomFieldSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `custom_field_id` 字段。
    pub custom_field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `code` 字段。
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 生态背调套餐摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoBackgroundCheckPackageSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `package_id` 字段。
    pub package_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 生态考试试卷摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoExamPaperSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `paper_id` 字段。
    pub paper_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 生态流程操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `custom_field_id` 字段。
    pub custom_field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `package_id` 字段。
    pub package_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `paper_id` 字段。
    pub paper_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `background_check_id` 字段。
    pub background_check_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `order_id` 字段。
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `progress` 字段。
    pub progress: Option<i32>,
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

/// 入职员工摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EmployeeSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `employee_id` 字段。
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `onboard_status` 字段。
    pub onboard_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 内推账户操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralAccountOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `account_id` 字段。
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
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

/// 外部面评操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalInterviewAssessmentResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_interview_assessment_id` 字段。
    pub external_interview_assessment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_interview_id` 字段。
    pub external_interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
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

/// 面试摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_id` 字段。
    pub interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_round_name` 字段。
    pub interview_round_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 开始时间。
    pub start_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 背调订单摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BackgroundCheckOrderSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `order_id` 字段。
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `vendor_name` 字段。
    pub vendor_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部内推奖励操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalReferralRewardResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_referral_reward_id` 字段。
    pub external_referral_reward_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `account_id` 字段。
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `amount` 字段。
    pub amount: Option<f64>,
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

/// 人才库操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentPoolOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `talent_pool_id` 字段。
    pub talent_pool_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
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

/// 生态考试操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoExamOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `exam_id` 字段。
    pub exam_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
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

/// Offer 自定义字段操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferCustomFieldOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_custom_field_id` 字段。
    pub offer_custom_field_id: Option<String>,
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

/// 面试官更新结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewerOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interviewer_id` 字段。
    pub interviewer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `user_id` 字段。
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `verify_status` 字段。
    pub verify_status: Option<i32>,
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

/// 职位 manager 操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobManagerOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `manager_id` 字段。
    pub manager_id: Option<String>,
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
