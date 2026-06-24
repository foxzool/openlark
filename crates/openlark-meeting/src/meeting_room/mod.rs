//! meeting_room default（历史版本资源）

pub mod building;
/// country 模块。
pub mod country;
/// district 模块。
pub mod district;
/// freebusy 模块。
pub mod freebusy;
/// instance 模块。
pub mod instance;
/// room 模块。
pub mod room;
/// service 模块。
pub mod service;
/// summary 模块。
pub mod summary;

pub use service::MeetingRoomService;

/// responses 模块。
pub mod responses;
