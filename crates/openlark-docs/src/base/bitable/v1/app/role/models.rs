use serde::{Deserialize, Serialize};

/// 数据表权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableRole {
    /// 数据表权限
    pub table_perm: i32,
    /// 数据表名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// 数据表 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// 记录筛选条件（当 `table_perm` 为 1 或 2 时生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_rule: Option<serde_json::Value>,
    /// 字段权限（仅在 `table_perm` 为 2 时生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_perm: Option<serde_json::Value>,
    /// 新增记录权限（仅在 `table_perm` 为 2 时生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_add_record: Option<bool>,
    /// 删除记录权限（仅在 `table_perm` 为 2 时生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_delete_record: Option<bool>,
}

impl TableRole {
    /// 创建仅包含权限级别的表角色配置。
    pub fn new(table_perm: i32) -> Self {
        Self {
            table_perm,
            table_name: None,
            table_id: None,
            rec_rule: None,
            field_perm: None,
            allow_add_record: None,
            allow_delete_record: None,
        }
    }
}

/// 仪表盘权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockRole {
    /// 仪表盘 ID
    pub block_id: String,
    /// 仪表盘权限
    pub block_perm: i32,
}

/// 自定义角色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Role {
    /// 自定义角色名称
    pub role_name: String,
    /// 自定义角色 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 针对数据表的权限设置
    pub table_roles: Vec<TableRole>,
    /// 针对仪表盘的权限设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_roles: Option<Vec<BlockRole>>,
}
