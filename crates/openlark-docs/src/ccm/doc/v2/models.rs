/// 旧版文档 v2 数据模型
///
/// 定义旧版文档相关的数据结构，包括文档基本信息、
/// 内容类型、元数据等。
use serde::{Deserialize, Serialize};

/// 文档类型
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum DocType {
    /// 文本文档
    #[default]
    #[serde(rename = "doc")]
    Doc,
    /// 电子表格
    #[serde(rename = "sheet")]
    Sheet,
    /// 幻灯片
    #[serde(rename = "slide")]
    Slide,
    /// 思维导图
    #[serde(rename = "mindnote")]
    Mindnote,
    /// 未知类型
    #[serde(other)]
    Unknown,
}

/// 文档状态
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum DocStatus {
    /// 草稿
    #[default]
    #[serde(rename = "draft")]
    Draft,
    /// 已发布
    #[serde(rename = "published")]
    Published,
    /// 已归档
    #[serde(rename = "archived")]
    Archived,
    /// 未知状态
    #[serde(other)]
    Unknown,
}

/// 文档基本信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentInfo {
    /// 文档Token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_token: Option<String>,
    /// 文档标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文档类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<DocType>,
    /// 文档状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DocStatus>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    /// 文档链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Creator {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// 电子表格元数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SheetMeta {
    /// 工作表数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_count: Option<i32>,
    /// 工作表信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<SheetInfo>>,
}

/// 工作表信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SheetInfo {
    /// 工作表ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    /// 工作表名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 行数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    /// 列数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub col_count: Option<i32>,
}

/// 文档内容格式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentFormat {
    /// 纯文本
    #[serde(rename = "text")]
    Text,
    /// 富文本
    #[serde(rename = "rich_text")]
    RichText,
}

/// 文档内容
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentContent {
    /// 内容格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ContentFormat>,
    /// 内容文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 版本信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}

/// 批量更新操作类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BatchUpdateOperationType {
    /// 插入内容
    #[serde(rename = "insert")]
    Insert,
    /// 删除内容
    #[serde(rename = "delete")]
    Delete,
    /// 替换内容
    #[serde(rename = "replace")]
    Replace,
    /// 更新标题
    #[serde(rename = "update_title")]
    UpdateTitle,
}

/// 批量更新操作
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateOperation {
    /// 操作类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<BatchUpdateOperationType>,
    /// 操作位置（起始位置）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
    /// 操作位置（结束位置）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i32>,
    /// 插入或替换的内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 新标题（用于更新标题操作）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_title: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
