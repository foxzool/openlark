//! 会议机器人（bot）模块

pub mod events;
pub mod join;
pub mod leave;
pub mod message;
pub mod models;
pub mod user_active_meeting;

/// 获取会议事件请求。
pub use events::GetBotEventsRequest;
/// 加入会议请求与模型。
pub use join::{BotJoinedMeeting, JoinBotBody, JoinBotRequest, JoinBotResponse, JoinIdentify};
/// 离开会议请求与模型。
pub use leave::{LeaveBotBody, LeaveBotRequest, LeaveBotResponse};
/// 发送会中消息请求与模型。
pub use message::{SendBotMessageBody, SendBotMessageRequest, SendBotMessageResponse};
/// 会议机器人对应用户。
pub use models::BotMeetingUser;
/// 获取用户活跃会议请求。
pub use user_active_meeting::GetUserActiveMeetingRequest;
