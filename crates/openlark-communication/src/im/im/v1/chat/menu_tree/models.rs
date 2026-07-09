//! 群菜单相关模型（不算 API）

use serde::{Deserialize, Serialize};

/// 群菜单一级菜单 ID 列表请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMenuTopLevelIdsBody {
    /// 一级菜单 ID 列表。
    pub chat_menu_top_level_ids: Vec<String>,
}
