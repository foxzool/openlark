//! 会议机器人（bot）模块

pub mod events;
pub mod user_active_meeting;

/// 获取会议事件请求。
pub use events::GetBotEventsRequest;
/// 获取用户活跃会议请求。
pub use user_active_meeting::GetUserActiveMeetingRequest;
