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
