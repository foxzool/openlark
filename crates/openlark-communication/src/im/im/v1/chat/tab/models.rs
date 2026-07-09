//! 会话标签页相关模型（不算 API）

use serde::{Deserialize, Serialize};

/// 会话标签页 ID 列表请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TabIdsBody {
    /// 标签页 ID 列表。
    pub tab_ids: Vec<String>,
}
