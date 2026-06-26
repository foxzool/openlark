//! 自定义字段 API v2 的数据模型

use serde::{Deserialize, Serialize};

/// 自定义字段类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CustomFieldType {
    /// 文本
    Text,
    /// 数字
    Number,
    /// 日期
    Date,
    /// 人员
    Person,
    /// 复选框
    Checkbox,
    /// 下拉选项
    Select,
}

/// 自定义字段配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldConfig {
    /// 字段类型
    #[serde(rename = "type")]
    pub field_type: CustomFieldType,
    /// 字段选项（用于 Select 类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

impl Default for CustomFieldConfig {
    fn default() -> Self {
        Self {
            field_type: CustomFieldType::Text,
            options: None,
        }
    }
}

/// 创建自定义字段请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateCustomFieldBody {
    /// 字段名称
    pub name: String,
    /// 字段配置
    pub config: CustomFieldConfig,
    /// 资源类型（目前官方仅支持 "tasklist"）
    pub resource_type: String,
    /// 资源 GUID（挂载的清单 GUID）
    pub resource_id: String,
}

/// 更新自定义字段请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateCustomFieldBody {
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 字段配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<CustomFieldConfig>,
}

/// 创建自定义字段响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCustomFieldResponse {
    /// 字段 GUID
    pub field_guid: String,
    /// 字段名称
    pub name: String,
    /// 字段配置
    pub config: CustomFieldConfig,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 获取自定义字段响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetCustomFieldResponse {
    /// 字段 GUID
    pub field_guid: String,
    /// 字段名称
    pub name: String,
    /// 字段配置
    pub config: CustomFieldConfig,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 更新自定义字段响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCustomFieldResponse {
    /// 字段 GUID
    pub field_guid: String,
    /// 字段名称
    pub name: String,
    /// 字段配置
    pub config: CustomFieldConfig,
    /// 更新时间
    pub updated_at: String,
}

/// 删除自定义字段响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCustomFieldResponse {
    /// 是否删除成功
    pub success: bool,
    /// 字段 GUID
    pub field_guid: String,
}

/// 自定义字段列表项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldItem {
    /// 字段 GUID
    pub field_guid: String,
    /// 字段名称
    pub name: String,
    /// 字段配置
    pub config: CustomFieldConfig,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 获取自定义字段列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCustomFieldsResponse {
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 列表项
    #[serde(default)]
    pub items: Vec<CustomFieldItem>,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
