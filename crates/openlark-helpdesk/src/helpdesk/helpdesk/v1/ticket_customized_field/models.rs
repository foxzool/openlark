//! 工单自定义字段共享模型

use serde::{Deserialize, Serialize};

/// 下拉选项集合。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TicketCustomizedFieldDropdownOptions {
    /// 下拉选项列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TicketCustomizedFieldDropdownOption>>,
}

/// 下拉选项。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TicketCustomizedFieldDropdownOption {
    /// 选项 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 选项展示名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 子选项，最多支持三级。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TicketCustomizedFieldDropdownOption>>,
}

/// 工单用户信息（ticket_user）。
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TicketUser {
    /// 用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 用户头像 url。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// 用户名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户邮箱。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// 工单自定义字段（官方 ticket_customized_field 完整模型，list/get 共用）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TicketCustomizedField {
    /// 工单自定义字段 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_customized_field_id: Option<String>,
    /// 服务台 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    /// 键名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    /// 名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 字段在后台管理列表中的位置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// 描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否可见。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    /// 是否可以修改。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    /// 是否必填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 创建时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// 创建用户。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<TicketUser>,
    /// 更新用户。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<TicketUser>,
    /// 下拉列表选项。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown_options: Option<TicketCustomizedFieldDropdownOptions>,
    /// 是否支持多选，仅在字段类型是 dropdown 的时候有效。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown_allow_multiple: Option<bool>,
}
