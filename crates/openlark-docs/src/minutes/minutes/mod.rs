/// Minutes API 模块
pub mod v1;

pub use v1::{
    GetMinuteMediaRequest, GetMinuteMediaResponse, GetMinuteRequest, GetMinuteResponse,
    GetMinuteStatisticsRequest, GetMinuteStatisticsResponse, GetMinuteTranscriptRequest,
    MinuteClipBody, MinuteClipRequest, MinuteClipResponse, MinuteInfo, MinuteMediaInfo,
    MinuteStatistics, MinuteTimeRange, MinuteUploadBody, MinuteUploadRequest, MinuteUploadResponse,
    ModelMinuteInfo, StatMinuteStatistics, StatUserViewDetail, UserIdType, UserViewDetail,
};
