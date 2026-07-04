//! okr/v2 跨叶共享的 domain entity struct。
//!
//! 这些 struct 代表同一飞书实体，跨多个 API 叶子重复出现（已确认 byte-identical）。
//! 为避免逐字重复（#336），各只在此处定义一次，叶子通过 `use` 引用。

use serde::Deserialize;

/// OKR 目标。
#[derive(Debug, Clone, Deserialize)]
pub struct Objective {
    /// 目标的 ID。
    pub id: String,
    /// 目标的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 目标的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: ObjectiveOwner,
    /// 目标的用户周期 ID。
    pub cycle_id: String,
    /// 目标的序号：从 1 开始计数。
    pub position: i32,
    /// 目标的内容。
    // TODO: 飞书文档 block 深度嵌套结构暂留 Value，后续可单独抽取 typed 模型。
    #[serde(default)]
    pub content: Option<serde_json::Value>,
    /// 目标的分数：\[0,1\]，支持一位小数。
    #[serde(default)]
    pub score: Option<f64>,
    /// 目标的备注。
    // TODO: 飞书文档 block 深度嵌套结构暂留 Value，后续可单独抽取 typed 模型。
    #[serde(default)]
    pub notes: Option<serde_json::Value>,
    /// 目标的权重：\[0,1\]，支持三位小数。
    #[serde(default)]
    pub weight: Option<f64>,
    /// 目标的截止时间，毫秒级时间戳。
    #[serde(default)]
    pub deadline: Option<String>,
    /// 目标的分类 ID。
    #[serde(default)]
    pub category_id: Option<String>,
}

/// 目标所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct ObjectiveOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// 量化指标。
#[derive(Debug, Clone, Deserialize)]
pub struct Indicator {
    /// 指标的 ID。
    pub id: String,
    /// 指标的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 指标的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: IndicatorOwner,
    /// 指标所属的实体类型。
    pub entity_type: i32,
    /// 指标所属的实体 ID。
    pub entity_id: String,
    /// 指标的状态。
    pub indicator_status: i32,
    /// 指标的状态的计算方式。
    pub status_calculate_type: i32,
    /// 指标的起始值。
    #[serde(default)]
    pub start_value: Option<f64>,
    /// 指标的目标值。
    #[serde(default)]
    pub target_value: Option<f64>,
    /// 指标的当前值。
    #[serde(default)]
    pub current_value: Option<f64>,
    /// 指标的当前值的计算方式。
    #[serde(default)]
    pub current_value_calculate_type: Option<i32>,
    /// 指标的单位。
    #[serde(default)]
    pub unit: Option<IndicatorUnit>,
}

/// 指标所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct IndicatorOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// 指标单位。
#[derive(Debug, Clone, Deserialize)]
pub struct IndicatorUnit {
    /// 指标的单位类型。
    pub unit_type: i32,
    /// 指标单位的值。
    pub unit_value: String,
}

/// 关键结果。
#[derive(Debug, Clone, Deserialize)]
pub struct KeyResult {
    /// 关键结果的 ID。
    pub id: String,
    /// 关键结果的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 关键结果的修改时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: KeyResultOwner,
    /// 关键结果的目标 ID。
    pub objective_id: String,
    /// 关键结果的序号：从 1 开始计数。
    pub position: i32,
    /// 关键结果的内容。
    // TODO: 飞书文档 block 深度嵌套结构暂留 Value，后续可单独抽取 typed 模型。
    #[serde(default)]
    pub content: Option<serde_json::Value>,
    /// 关键结果的分数：\[0,1\]，支持一位小数。
    #[serde(default)]
    pub score: Option<f64>,
    /// 目标的权重：\[0,1\]，支持三位小数。
    #[serde(default)]
    pub weight: Option<f64>,
    /// 关键结果的截止时间，毫秒级时间戳。
    #[serde(default)]
    pub deadline: Option<String>,
}

/// 关键结果所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct KeyResultOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// OKR 对齐。
#[derive(Debug, Clone, Deserialize)]
pub struct Alignment {
    /// 对齐的 ID。
    pub id: String,
    /// 对齐的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 对齐的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 发起对齐的所有者。
    pub from_owner: AlignmentOwner,
    /// 被对齐的所有者。
    pub to_owner: AlignmentOwner,
    /// 发起对齐的实体类型。
    pub from_entity_type: i32,
    /// 发起对齐的实体 ID。
    pub from_entity_id: String,
    /// 被对齐的实体类型。
    pub to_entity_type: i32,
    /// 被对齐的实体 ID。
    pub to_entity_id: String,
}

/// 对齐所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct AlignmentOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}
