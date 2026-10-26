use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Base v2 自定义角色（AppRole）
///
/// docPath: /document/docs/bitable-v1/advanced-permission/app-role/list-2
/// doc: <https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/list-2>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRole {
    /// 自定义权限的名字
    pub role_name: String,
    /// 数据表权限配置列表
    ///
    /// 注意：该字段结构较复杂（包含记录规则、字段权限等），此处按 JSON 透传。
    #[serde(default)]
    pub table_roles: Vec<serde_json::Value>,
    /// 角色ID（以 "custom_" 开头）
    pub role_id: String,
    /// Block 权限配置列表（结构按 JSON 透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_roles: Option<Vec<serde_json::Value>>,
    /// Base 规则（结构按 JSON 透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_rule: Option<serde_json::Value>,
}

/// 记录筛选条件（rec_rule / other_rec_rule 的 conditions 成员项）
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/advanced-permission/base-v2/app-role/create
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct RecRuleCondition {
    /// 条件字段的名称。记录筛选条件为“创建人包含访问者本人”时，此参数值为 ""
    pub field_name: String,
    /// 条件运算符：is/isNot/contains/doesNotContain/isEmpty/isNotEmpty，默认 is
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// 条件的值，可以是单个值或多个值的数组
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// 记录筛选条件（table_role.rec_rule）
///
/// 当 `table_perm` 为 1 或 2 时生效，用于指定可编辑或可阅读的记录。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct RecRule {
    /// 记录筛选条件列表（最多 10 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RecRuleCondition>>,
    /// 多个筛选条件的关系：and/or，默认 and
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conjunction: Option<String>,
    /// 其他记录权限（0-禁止查看，1-仅可阅读），仅在 table_perm 为 2 时生效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_perm: Option<i32>,
}

/// 其余记录筛选条件（table_role.other_rec_rule）
///
/// 在 `rec_rule.other_perm` 为 0 时生效，用于指定未命中 `rec_rule` 记录的可阅读范围；
/// 仅高级权限为 v2 版本的多维表格支持。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct OtherRecRule {
    /// 记录筛选条件列表（最多 10 条）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RecRuleCondition>>,
    /// 多个筛选条件的关系：and/or，默认 and
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conjunction: Option<String>,
}

/// 针对数据表的权限设置（table_roles 成员项）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TableRole {
    /// 数据表权限：0-无权限，1-仅可阅读，2-可编辑，4-可管理
    pub table_perm: i32,
    /// 数据表名称（与 table_id 至少填写一项）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// 多维表格数据表的唯一标识（与 table_name 至少填写一项）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// 记录筛选条件，当 table_perm 为 1 或 2 时生效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_rule: Option<RecRule>,
    /// 其余记录筛选条件，在 rec_rule.other_perm 为 0 时生效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_rec_rule: Option<OtherRecRule>,
    /// 字段权限：key 为字段名称，value 为 1-可阅读 / 2-可添加 / 3-可编辑
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_perm: Option<HashMap<String, i32>>,
    /// 新增记录权限，仅在 table_perm 为 2 时生效，默认 true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_add_record: Option<bool>,
    /// 删除记录权限，仅在 table_perm 为 2 时生效，默认 true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_delete_record: Option<bool>,
    /// 视图权限：1-可阅读，2-可编辑，默认 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_perm: Option<i32>,
    /// 可读的视图集合：key 为视图 ID，value 为 0-无权限 / 1-可阅读
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_rules: Option<HashMap<String, i32>>,
    /// 字段点位权限（仅 v2 高级权限支持）：
    /// key 为 select_option_edit / attachment_export，
    /// value 为 {字段名称: 0-无权限 / 1-有权限}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_action_rules: Option<HashMap<String, HashMap<String, i32>>>,
}

/// 针对仪表盘的权限设置（block_roles 成员项）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct BlockRole {
    /// 多维表格仪表盘的唯一标识（以 blk 开头）
    pub block_id: String,
    /// 仪表盘权限：0-无权限，1-可阅读
    pub block_perm: i32,
}
