//! Meeting 资源模块

/// recording 模块。
pub mod recording;
/// search 模块。
pub mod search;
/// 订阅会议变更事件接口。
pub mod subscription;
/// 取消订阅会议变更事件接口。
pub mod unsubscription;

pub use subscription::SubscribeMeetingRequest;
pub use unsubscription::UnsubscribeMeetingRequest;

/// end 模块。
pub mod end;
/// get 模块。
pub mod get;
/// invite 模块。
pub mod invite;
/// kickout 模块。
pub mod kickout;
/// list_by_no 模块。
pub mod list_by_no;
/// set_host 模块。
pub mod set_host;
