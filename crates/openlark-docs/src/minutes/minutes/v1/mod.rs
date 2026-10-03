/// Minutes V1 API 模块
pub mod minute;

pub use minute::{
    GetMinuteMediaRequest, GetMinuteMediaResponse, GetMinuteRequest, GetMinuteResponse,
    GetMinuteStatisticsRequest, GetMinuteStatisticsResponse, GetMinuteTranscriptRequest,
    MinuteClipBody, MinuteClipRequest, MinuteClipResponse, MinuteInfo, MinuteMediaInfo,
    MinuteStatistics, MinuteTimeRange, MinuteUploadBody, MinuteUploadRequest, MinuteUploadResponse,
    ModelMinuteInfo, StatMinuteStatistics, StatUserViewDetail, UserIdType, UserViewDetail,
};
