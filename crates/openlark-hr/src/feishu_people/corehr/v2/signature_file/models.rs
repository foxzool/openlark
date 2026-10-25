//! 电子签文件（signature_file）共享模型
//!
//! query / list_by_biz_id 响应的元素官方类型均为 `signature_file`，
//! 与 list 的 9 字段结构相比多 `pre_hire_id` 与 `sign_url`，故单独建模；
//! i18n / enum 原语复用 [`super::list`] 的既有定义。

use super::list::{EnumValue, I18nText};
use serde::{Deserialize, Serialize};

/// 电子签文件（官方 signature_file 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureFile {
    /// 电子签文件 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_file_id: Option<String>,
    /// 名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nText>>,
    /// 创建时间，格式：YYYY-MM-DD 00:00:00
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间，格式：YYYY-MM-DD 00:00:00
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 员工雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 待入职 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire_id: Option<String>,
    /// 电子签文件状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_file_state: Option<EnumValue>,
    /// 供应商侧的合同编号，作为幂等 key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_code: Option<String>,
    /// 电子签文件生效日期，格式：yyyy-MM-dd
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 电子签模板 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 签署链接
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sign_url: Option<String>,
}
