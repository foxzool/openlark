//! 被动解除补偿金的共享字段。
//!
//! 形状来自官方文档 `corehr-v2/severance_pay` 的 edit / query / submit。

use openlark_core::{CoreError, SDKResult, validate_required};
use serde::{Deserialize, Serialize};

/// 多语文本。官方 `i18n`：`lang` + `value`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct LangText {
    /// 语言编码，例如 `zh-CN`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 该语言下的文本。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 枚举字段。官方 `enum`：`enum_name` + 可选 `display`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct EnumValue {
    /// 枚举值。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举的多语展示。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<LangText>>,
}

/// 自定义字段键值。官方 `object_field_data`：`field_name` + `value`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ObjectFieldData {
    /// 字段 APIName。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// JSON 转义后的字段值。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 查询结果里的自定义字段。官方 `custom_field_data`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CustomFieldData {
    /// 自定义字段 APIName。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 自定义字段名称。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CustomFieldName>,
    /// 字段类型。1 文本，2 布尔，3 数字，4 枚举，5 Lookup，8 时间，9 附件。
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub field_type: Option<i32>,
    /// JSON 转义后的字段值。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义字段的中英文名称。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CustomFieldName {
    /// 中文名称。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文名称。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 发起流程时提交的补偿金信息。查询结果多出来的系数单位不在这里。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SubmitSeverancePayInfo {
    /// 补偿类型。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severance_pay_type: Option<EnumValue>,
    /// 标准补偿金额。用字符串避免精度问题。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_severance_pay_amount: Option<String>,
    /// 超额补偿金额。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceeding_severance_pay_amount: Option<String>,
    /// 币种。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// 补偿说明。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_explanation: Option<String>,
    /// 经济补偿金基数起始月，`YYYY-MM`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_compensation_start_month: Option<String>,
    /// 经济补偿金基数结束月，`YYYY-MM`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_compensation_end_month: Option<String>,
    /// 经济补偿金基数。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_compensation_base: Option<String>,
    /// 经济补偿系数。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_compensation_coefficient: Option<String>,
    /// 经济补偿金实际金额。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_standard_amount: Option<String>,
    /// 代通知金基数起始月，`YYYY-MM`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_compensation_start_month: Option<String>,
    /// 代通知金基数结束月，`YYYY-MM`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_compensation_end_month: Option<String>,
    /// 代通知金基数。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_compensation_base: Option<String>,
    /// 代通知金系数。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_compensation_coefficient: Option<String>,
    /// 代通知金测算金额。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_compensation_amount: Option<String>,
    /// 代通知金实际金额。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_notice_payment_amount: Option<String>,
    /// 超额补偿基数起始月，`YYYY-MM`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceeding_base_start_month: Option<String>,
    /// 超额补偿基数结束月，`YYYY-MM`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceeding_base_end_month: Option<String>,
    /// 超额补偿基数。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceeding_compensation_base: Option<String>,
    /// 超额补偿月数。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceeding_compensation_coefficient: Option<String>,
    /// 超额补偿实际金额。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_excess_amount: Option<String>,
    /// 其他浮动基数起始月，`YYYY-MM`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_floating_base_start_month: Option<String>,
    /// 其他浮动基数结束月，`YYYY-MM`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_floating_base_end_month: Option<String>,
    /// 其他浮动基数。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_floating_base: Option<String>,
    /// 其他浮动系数。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_floating_coefficient: Option<String>,
    /// 其他浮动测算金额。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_floating_amount: Option<String>,
    /// 其他浮动实际金额。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_other_floating_amount: Option<String>,
    /// 补偿金测算总额。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expect_severance_pay_amount: Option<String>,
    /// 实际补偿总额。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severance_pay_amount: Option<String>,
    /// 补偿金金额上限。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severance_pay_upper_limit: Option<String>,
    /// 补偿金信息说明。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severance_pay_calculate_explanation: Option<String>,
}

/// 查询结果中的补偿金信息。比发起流程多四个系数单位。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QuerySeverancePayInfo {
    /// 发起流程时也会提交的金额字段。
    #[serde(flatten)]
    pub pay: SubmitSeverancePayInfo,
    /// 经济补偿系数单位。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_compensation_unit: Option<EnumValue>,
    /// 代通知金系数单位。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_compensation_unit: Option<EnumValue>,
    /// 超额补偿系数单位。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceeding_compensation_unit: Option<EnumValue>,
    /// 其他浮动系数单位。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_floating_unit: Option<EnumValue>,
}

/// 一条被动解除补偿金记录。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SeverancePay {
    /// 补偿金业务单 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severance_pay_id: Option<String>,
    /// 关联离职单 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_id: Option<String>,
    /// 员工雇佣 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 补偿金状态。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EnumValue>,
    /// 测算状态。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calculate_status: Option<EnumValue>,
    /// 离职日期。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_date: Option<String>,
    /// 补偿金信息。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severance_pay_info: Option<QuerySeverancePayInfo>,
    /// 申请发起时间，Unix 秒级时间戳。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_initiating_time: Option<String>,
    /// BPM 流程 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    /// 离职原因。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<EnumValue>,
    /// 离职说明。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_explanation: Option<String>,
    /// 发起人雇佣 ID。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator_id: Option<String>,
    /// 解除申请结束时间，Unix 秒级时间戳。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_finish_time: Option<String>,
    /// 签署方式。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sign_type: Option<EnumValue>,
    /// 社保公积金最后缴纳月份。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_insurance_end_date: Option<String>,
    /// 公积金截止月。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provident_fund_end_date: Option<String>,
    /// 审批结果。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluation_result: Option<EnumValue>,
    /// 撤销原因。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdrawn_reason: Option<String>,
    /// 跟进人雇佣 ID 列表。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<String>>,
    /// 补偿金申请开始时间，Unix 秒级时间戳。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 更新时间，Unix 秒级时间戳。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    /// 自定义字段。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
    /// 签署文件 ID 列表。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_file_ids: Option<Vec<String>>,
    /// 从一级离职原因到当前 reason 的 APIName 列表。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_chain: Option<Vec<String>>,
}

pub(crate) fn take_required_text(
    value: Option<String>,
    message: &'static str,
) -> SDKResult<String> {
    let Some(text) = value else {
        return Err(CoreError::validation_msg(message));
    };
    validate_required!(text, message);
    Ok(text)
}

pub(crate) fn take_bounded_text(
    value: Option<String>,
    max_chars: usize,
    message: &'static str,
) -> SDKResult<String> {
    let text = take_required_text(value, message)?;
    if text.chars().count() > max_chars {
        return Err(CoreError::validation_msg(message));
    }
    Ok(text)
}

pub(crate) fn optional_text(
    value: &Option<String>,
    max_chars: usize,
    message: &'static str,
) -> SDKResult<()> {
    let Some(text) = value else {
        return Ok(());
    };
    validate_required!(text, message);
    if text.chars().count() > max_chars {
        return Err(CoreError::validation_msg(message));
    }
    Ok(())
}

pub(crate) fn optional_list(
    values: &Option<Vec<String>>,
    max_len: usize,
    message: &'static str,
) -> SDKResult<()> {
    if let Some(values) = values {
        openlark_core::validate_allow_empty_list!(values, max_len, message);
    }
    Ok(())
}

pub(crate) fn require_object_fields(items: &[ObjectFieldData]) -> SDKResult<()> {
    for item in items {
        let Some(field_name) = &item.field_name else {
            return Err(CoreError::validation_msg("field_name 不能为空"));
        };
        validate_required!(field_name, "field_name 不能为空");
        let Some(value) = &item.value else {
            return Err(CoreError::validation_msg("value 不能为空"));
        };
        validate_required!(value, "value 不能为空");
    }
    Ok(())
}

pub(crate) fn require_enum(value: &EnumValue) -> SDKResult<()> {
    let Some(enum_name) = &value.enum_name else {
        return Err(CoreError::validation_msg("enum_name 不能为空"));
    };
    validate_required!(enum_name, "enum_name 不能为空");
    if let Some(display) = &value.display {
        for text in display {
            let Some(lang) = &text.lang else {
                return Err(CoreError::validation_msg("lang 不能为空"));
            };
            validate_required!(lang, "lang 不能为空");
            let Some(value) = &text.value else {
                return Err(CoreError::validation_msg("value 不能为空"));
            };
            validate_required!(value, "value 不能为空");
        }
    }
    Ok(())
}
