//! Hire 业务域专属响应模型。
//!
//! 域无关的 HR 原语（`I18nText` / `FlexibleText` / `IdNameObject` /
//! `CodeNameObject` / `PaginatedResponse` / `CatalogItem` / `LocalizedLabel`）
//! 已迁至 crate root：[`crate::common::shared_models`]（#473）。0.19 起本模块
//! 不再 re-export 这些共享原语（#556）；请改用
//! `openlark_hr::common::shared_models::*`。本模块仅保留 hire 专属摘要类型
//! （职位 / 投递 / 面试 / offer / 生态 / 猎头 等）。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 面试任务摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewTaskSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人才 ID。
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `interview_id` 字段。
    pub interview_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `interview_round_id` 字段。
    pub interview_round_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `interview_round_name` 字段。
    pub interview_round_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 任务状态：1 未开始 / 2 未评价 / 3 已评价 / 5 已终止。
    pub activity_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 操作人类型（当前只返回 `1`：员工）。
    pub operator_type: Option<i32>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附加信息 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 数据来源：1 官网 / 2 详情页。
    pub source_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间，毫秒时间戳。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 更新时间，毫秒时间戳。
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附加信息列表。
    pub di_data: Option<Vec<DiDataItem>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 多元化附加信息条目（官方 `di_data`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DiDataItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段值，格式随字段类型不同（单选为 `"1"`；多选为 JSON 数组字符串；日期范围为时间戳数组字符串等）。
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段属性。
    pub object_attribute: Option<DiDataObjectAttribute>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 多元化附加信息字段属性（官方 `object_attribute`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DiDataObjectAttribute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段名称。
    pub title: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段描述。
    pub description: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段类型：1 单选 / 2 多选 / 3 月份选择 / 4 年份选择 / 5 数字 / 6 文本 / 7 多行文本 / 8 日期范围。
    pub data_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段标签：1 性别 / 2 种族 / 3 身体情况。
    pub tags: Option<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否为 FCF 数据（新加坡公平考虑框架相关）。
    pub is_fcf_data: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否为 D&I 数据（多元化与包容性相关）。
    pub is_di_data: Option<bool>,
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

/// 外部投递信息（官方 `external_application`，创建外部投递响应容器）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalApplication {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部投递 ID（由飞书招聘系统生成）。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位招聘类型：1 社招 / 2 校招。
    pub job_recruitment_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位名称。
    pub job_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 简历来源。
    pub resume_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阶段名称。
    pub stage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人才 ID。
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 终止原因。
    pub termination_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递类型：1 HR 寻访 / 2 候选人主动投递 / 3 人才推荐 / 4 其他。
    pub delivery_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递在外部系统终止时间，毫秒时间戳。
    pub modify_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递在外部系统创建时间，毫秒时间戳。
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 终止类型。
    pub termination_type: Option<String>,
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

/// 外部通用附件（官方 `external_common_attachment`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalCommonAttachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附件 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附件名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附件大小（单位：字节）。
    pub size: Option<i64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部 Offer（官方 `external_offer`，创建外部 Offer 响应容器）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalOffer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部 Offer ID（由飞书招聘系统生成）。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部投递 ID。
    pub external_application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 创建时间，毫秒时间戳。
    pub biz_create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 负责人姓名。
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 状态。
    pub offer_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 详情附件列表。
    pub attachment_list: Option<Vec<ExternalCommonAttachment>>,
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

/// 招聘官网推广渠道（官方 `website_channel_info`：扁平 `{id, name, link, code}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteChannelInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推广渠道 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推广渠道名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推广渠道链接。
    pub link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推广渠道推广码。
    pub code: Option<String>,
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

/// 招聘官网投递信息（官方 `website_delivery_dto`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryDto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘投递 ID。
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网投递 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位广告 ID。
    pub job_post_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 简历 ID。
    pub portal_resume_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网用户 ID。
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人才 ID。
    pub talent_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘官网职位广告地址（官方 `common_address`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteJobPostAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位地址 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位地址名称。
    pub name: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 区域信息。
    pub district: Option<crate::common::shared_models::CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 城市信息。
    pub city: Option<crate::common::shared_models::CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 省信息。
    pub state: Option<crate::common::shared_models::CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 国家信息。
    pub country: Option<crate::common::shared_models::CodeNameObject>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘官网职位广告摘要（官方 `website_job_post`）。
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位广告 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位编码。
    pub job_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位过期时间，毫秒级时间戳，「null」代表「长期有效」。
    pub job_expire_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位状态：1 启用态 / 2 禁用态。
    pub job_active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位流程类型：1 社招 / 2 校招。
    pub job_process_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位雇佣类型。
    pub job_recruitment_type: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位部门。
    pub job_department: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别。
    pub job_type: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低职级。
    pub min_job_level: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高职级。
    pub max_job_level: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位地址（详情接口标记为废弃字段，请使用 `address_list`）。
    pub address: Option<WebsiteJobPostAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 月薪范围-最低薪资，单位：K。
    pub min_salary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 月薪范围-最高薪资，单位：K。
    pub max_salary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 学历要求。
    pub required_degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 经验要求。
    pub experience: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘数量。
    pub headcount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位亮点列表。
    pub high_light_list: Option<Vec<crate::common::shared_models::IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位描述。
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位要求。
    pub requirement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建人。
    pub creator: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间，毫秒级时间戳。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 修改时间，毫秒级时间戳。
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职能分类（详情接口返回）。
    pub job_function: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位项目（详情接口返回）。
    pub subject: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位地址列表。
    pub address_list: Option<Vec<WebsiteJobPostAddress>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 目标专业列表。
    pub target_major_list: Option<Vec<crate::common::shared_models::IdNameObject>>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 用户姓名。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部 ID（幂等字段）。
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 国家码（与 `mobile` 成对出现）。
    pub mobile_country_code: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 供应商联系人 ID，与入参 `user_id_type` 类型一致。
    pub contactor_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 供应商联系人名称。
    pub contactor_name: Option<crate::common::shared_models::I18nText>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 猎头 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 禁用原因，仅当 `status` 为 1（已禁用）时填充。
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 添加时间，毫秒时间戳。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 角色：0 管理员 / 1 顾问。
    pub role: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 猎头用户信息。
    pub user_info: Option<AgencyAccountUser>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 猎头用户信息（官方 `agency_account_user`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencyAccountUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 用户 ID，与入参 `user_id_type` 类型一致。
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 用户名称。
    pub name: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 用户邮箱（需 `hire:agency.email:readonly` 权限才返回）。
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 用户手机号（需 `hire:agency.mobile:readonly` 权限才返回）。
    pub mobile: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人才在外部系统的创建时间，毫秒时间戳。
    pub external_create_time: Option<String>,
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

/// 职位类别信息（官方 `job_type_info`，含 `parent_id`，超出 `IdNameObject` 范围）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobTypeInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别名称。
    pub name: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 父级职位类别 ID。
    pub parent_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘需求自定义字段选项（官方 `*_customized_option`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCustomizedOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项 ID。
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项名称。
    pub name: Option<crate::common::shared_models::I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘需求自定义字段时间段（官方 `*_customized_time_range`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCustomizedTimeRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 开始时间，毫秒时间戳。
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 结束时间，毫秒时间戳。
    pub end_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘需求自定义字段值（官方 `*_customized_value`，按字段类型从不同子字段取值）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCustomizedValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 单行文本、多行文本、模块、默认字段取值。
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 单选题取值。
    pub option: Option<JobCustomizedOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 多选题取值。
    pub option_list: Option<Vec<JobCustomizedOption>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 时间段取值。
    pub time_range: Option<JobCustomizedTimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 日期/月份/年份选择取值，毫秒级时间戳。
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 数字取值。
    pub number: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘需求自定义字段（官方 `job_requirement_customized_data_dto`，响应侧形态）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCustomizedData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 自定义字段 ID。
    pub object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段名称。
    pub name: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段类型：1 单行文本 … 11 模块。
    pub object_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 自定义字段值。
    pub value: Option<JobCustomizedValue>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘需求信息（官方 `job_requirement_dto`，创建招聘需求响应容器）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRequirementDto {
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
    /// 需求状态：1 未开始 / 2 进行中 / 3 已取消 / 4 已暂停 / 5 已完成 / 6 已超期。
    pub display_progress: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 需求人数。
    pub head_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位性质。
    pub recruitment_type: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 人员类型。
    pub employee_type: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高职级。
    pub max_level: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低职级。
    pub min_level: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列。
    pub sequence: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 需求类型：1 新增 / 2 替换。
    pub category: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 需求部门。
    pub department: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 需求负责人列表。
    pub recruiter_list: Option<Vec<crate::common::shared_models::IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 需求用人经理列表。
    pub jr_hiring_managers: Option<Vec<crate::common::shared_models::IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 直属上级列表。
    pub direct_leader_list: Option<Vec<crate::common::shared_models::IdNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 开始日期，毫秒时间戳。
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 预计完成日期，毫秒时间戳。
    pub deadline: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘优先级：1 高 / 2 中 / 3 低。
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 学历要求。
    pub required_degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高月薪，单位：K。
    pub max_salary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低月薪，单位：K。
    pub min_salary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作地址。
    pub address: Option<crate::common::shared_models::IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 需求描述。
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 自定义字段列表。
    pub customized_data_list: Option<Vec<JobCustomizedData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘类型：1 社招 / 2 校招。
    pub process_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别。
    pub job_type: Option<JobTypeInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间，毫秒时间戳。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建人 ID。
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 更新时间，毫秒时间戳。
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职务 ID。
    pub employment_job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 岗位 ID。
    pub position_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 完成时间，毫秒级时间戳。
    pub completion_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 审批状态：1 未发起 / 2 审批中 / 3 审批通过 / 4 已撤回 / 5 未通过。
    pub approval_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 关联职位 ID 列表。
    pub job_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘需求招聘进展统计数值。
    pub count_data: Option<JobRequirementCountData>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘需求招聘进展统计（官方 `jr_count_data_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRequirementCountData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Offer 沟通中的数量。
    pub offer_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 待入职中的数量。
    pub pre_hire_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 已完成的数量。
    pub complete_count: Option<i32>,
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

/// 外部背调附件（官方 `external_background_check_attachment`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalBackgroundCheckAttachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附件 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附件名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 附件大小（单位：字节）。
    pub size: Option<i64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部背调信息（官方 `external_background_check`，创建外部背调响应容器）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalBackgroundCheck {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部背调 ID（由飞书招聘系统生成）。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部投递 ID。
    pub external_application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 背调日期，毫秒时间戳。
    pub date: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 背调名称。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 背调结果。
    pub result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 背调附件列表。
    pub attachment_list: Option<Vec<ExternalBackgroundCheckAttachment>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部面试评价维度（官方 `external_interview_assessment_dimension`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalInterviewAssessmentDimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 打分题分数（维度类型为「打分题」时使用）。
    pub score: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 单选选项（维度类型为「单选题」时使用）。
    pub option: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 多选选项（维度类型为「多选题」时使用）。
    pub options: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 描述内容（维度类型为「描述题」时使用）。
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 维度类型：1 打分题 / 2 单选题 / 3 描述题 / 4 多选题。
    pub assessment_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 维度标题。
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 维度描述。
    pub description: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部面试面评（官方 `external_interview_assessment`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalInterviewAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部面评 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试官姓名。
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试结果：1 不通过 / 2 通过 / 3 待定。
    pub conclusion: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 评价维度列表。
    pub assessment_dimension_list: Option<Vec<ExternalInterviewAssessmentDimension>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 综合评价。
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部面试 ID。
    pub external_interview_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部面试信息（官方 `external_interview`，创建外部面试响应容器）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalInterview {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部投递 ID。
    pub external_application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 外部面试 ID（由飞书招聘系统生成）。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 参与状态：1 未参与 / 2 参与 / 3 爽约。
    pub participate_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 开始时间，毫秒时间戳。
    pub begin_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 结束时间，毫秒时间戳。
    pub end_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价列表。
    pub interview_assessments: Option<Vec<ExternalInterviewAssessment>>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 员工 ID（官方字段名为 `id`）。
    pub id: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 转正状态（1：未转正 2：已转正）。
    pub conversion_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 实际入职时间，毫秒时间戳。
    pub onboard_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 预期转正时间，毫秒时间戳。
    pub expected_conversion_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 实际转正时间，毫秒时间戳。
    pub actual_conversion_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 离职时间，毫秒时间戳。
    pub overboard_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 离职原因。
    pub overboard_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 办公地点 Code 码。
    pub onboard_city_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 入职部门 ID，与入参 `department_id_type` 类型一致。
    pub department: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 直属上级 ID，与入参 `user_id_type` 类型一致。
    pub leader: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 序列 ID，与入参 `job_family_id_type` 类型一致。
    pub sequence: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职级 ID，与入参 `job_level_id_type` 类型一致。
    pub level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 员工类型 ID，与入参 `employee_type_id_type` 类型一致。
    pub employee_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘需求 ID。
    pub job_requirement_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 飞书人事的雇佣 ID（`get_by_application` 响应返回）。
    pub external_employment_id: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试 ID（官方 `interview_extend.id`）。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试开始时间，毫秒时间戳。
    pub begin_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试结束时间，毫秒时间戳。
    pub end_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次。
    pub round: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价信息列表（官方 `interview_record`，结构与「批量获取面试评价详细信息」一致）。
    pub interview_record_list:
        Option<Vec<crate::hire::hire::v1::interview_record::list::InterviewRecordItem>>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 背调状态：2 已安排 / 3 已完成 / 4 已终止 / 5 审批中 / 6 审批已撤回 / 8 审批通过 / 9 审批未通过。
    pub order_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 供应商类型：1 八方锦程 / 2 i背调 / 3 轩渡 / 127 自定义供应商。
    pub account_third_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 背调套餐名称。
    pub package: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 背调名称（仅用户手动录入的背调结果支持返回该字段）。
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 背调报告列表，按照报告创建时间降序排列。
    pub feedback_info_list: Option<Vec<BackgroundCheckFeedbackInfo>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 背调报告信息（官方 `background_check_order_feedback_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BackgroundCheckFeedbackInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 背调报告 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 背调信息附件下载链接（与 `report_preview_url` 同一供应商只能支持 1 种），有效期 1 小时。
    pub attachment_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 背调预览链接（与 `attachment_url` 同一供应商只能支持 1 种），有效期由供应商控制。
    pub report_preview_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 背调结果（红灯、黄灯、蓝灯、绿灯，或更新背调进度时推送的状态）。
    pub result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 报告类型：1 阶段性报告 / 2 终版报告。
    pub report_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 报告名称。
    pub report_name: Option<String>,
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

/// 职位模板/招聘需求模板通用模块（官方 `common_schema`）。
///
/// 被 `job_schema/list`（职位模板）与 `job_requirement_schema/list`
/// （招聘需求模板）两个接口的 `object_list` 共用，字段按官方响应体字段表建模。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CommonSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块名称。
    pub name: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块描述。
    pub description: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块配置信息。
    pub setting: Option<CommonSchemaSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否是自定义模块。
    pub is_customized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否必填。
    pub is_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否启用（1：已启用 2：已停用）。
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块中的字段列表。
    pub children_list: Option<Vec<CommonSchemaChild>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 通用模板模块配置（官方 `common_schema_setting`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CommonSchemaSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 模块类型（11：模块；1-10、13：字段类型）。
    pub object_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 配置信息（模块级固定返回空，字段级可含选项）。
    pub config: Option<CommonSchemaConfig>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 通用模板模块配置项（官方 `common_schema_config`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CommonSchemaConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项信息，仅字段类型为单选（3）/多选（4）时有值。
    pub options: Option<Vec<CommonSchemaOption>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 通用模板选项（官方 `common_schema_option`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CommonSchemaOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项 ID。
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项名称。
    pub name: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项描述。
    pub description: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否启用（1：已启用 2：已停用）。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 通用模板模块中的字段（官方 `common_schema_child`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CommonSchemaChild {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段名称。
    pub name: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段描述。
    pub description: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段配置信息。
    pub setting: Option<CommonSchemaSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 所属模块 ID，对应外层 `object_list.id`。
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否是自定义字段。
    pub is_customized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否必填。
    pub is_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 是否启用（1：已启用 2：已停用）。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}
