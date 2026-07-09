//! 实体与标签绑定关系相关模型（不算 API）

use serde::{Deserialize, Serialize};

/// 绑定/解绑标签请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BizEntityTagRelationBody {
    /// 标签业务类型。
    pub tag_biz_type: String,
    /// 业务实体 ID。
    pub biz_entity_id: String,
    /// 标签 ID 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
}
