#![allow(clippy::module_inception)]

/// 妙记服务模块
///
/// 提供飞书妙记的创建、查询、管理等功能。
/// docPath: <https://open.feishu.cn/document/server-docs/minutes-v1/minute/get>
pub mod minutes;

pub use minutes::{
    GetMinuteMediaRequest, GetMinuteMediaResponse, GetMinuteRequest, GetMinuteResponse,
    GetMinuteStatisticsRequest, GetMinuteStatisticsResponse, GetMinuteTranscriptRequest,
    MinuteInfo, MinuteMediaInfo, MinuteStatistics, ModelMinuteInfo, StatMinuteStatistics,
    StatUserViewDetail, UserIdType, UserViewDetail,
};
