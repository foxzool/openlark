/// get 模块。
pub mod get;
/// subscription 模块。
pub mod subscription;
/// unsubscription 模块。
pub mod unsubscription;

/// 订阅纪要变更事件请求。
pub use subscription::SubscribeNoteRequest;
/// 取消订阅纪要变更事件请求。
pub use unsubscription::UnsubscribeNoteRequest;
