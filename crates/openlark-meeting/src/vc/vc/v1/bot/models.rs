//! 会议机器人共享模型。

use serde::{Deserialize, Serialize};

/// 会议中的机器人对应用户。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotMeetingUser {
    /// 用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 用户类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
}
