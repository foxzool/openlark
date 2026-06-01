/// 获取会议纪要详情接口。
pub mod get;
/// 获取会议纪要媒体接口。
pub mod media;
/// 会议纪要模型模块。
pub mod models;
/// 会议纪要统计接口。
pub mod statistics;
/// 订阅妙记变更事件接口。
pub mod subscription;
/// 会议纪要转写接口。
pub mod transcript;
/// 取消订阅妙记变更事件接口。
pub mod unsubscription;

/// 重新导出会议纪要详情类型。
pub use get::{GetMinuteRequest, GetMinuteResponse, MinuteInfo};
/// 重新导出会议纪要媒体类型。
pub use media::{GetMinuteMediaRequest, GetMinuteMediaResponse};
/// 重新导出会议纪要模型。
pub use models::{
    MinuteInfo as ModelMinuteInfo, MinuteMediaInfo, MinuteStatistics, UserIdType, UserViewDetail,
};
/// 重新导出会议纪要统计类型。
pub use statistics::{
    GetMinuteStatisticsRequest, GetMinuteStatisticsResponse,
    MinuteStatistics as StatMinuteStatistics, UserViewDetail as StatUserViewDetail,
};
/// 重新导出订阅妙记变更事件请求类型。
pub use subscription::SubscribeMinuteRequest;
/// 重新导出会议纪要转写请求类型。
pub use transcript::GetMinuteTranscriptRequest;
/// 重新导出取消订阅妙记变更事件请求类型。
pub use unsubscription::UnsubscribeMinuteRequest;
/// artifacts 模块。
pub mod artifacts;
/// search 模块。
pub mod search;
