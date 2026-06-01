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
