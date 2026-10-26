use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 批量查询忙闲信息请求体
///
/// time_min/time_max/user_ids 必填；time_min 与 time_max 间隔不能超过 2 周。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchFreebusyRequestBody {
    /// 获取忙闲信息的开始时间（RFC 3339 date_time 格式），与 time_max 间隔不能超过 2 周
    pub time_min: String,
    /// 获取忙闲信息的结束时间（RFC 3339 date_time 格式），与 time_min 间隔不能超过 2 周
    pub time_max: String,
    /// 用户 ID 列表（与查询参数 user_id_type 匹配），1 ～ 10 个
    pub user_ids: Vec<String>,
    /// 是否包含用户绑定的三方日历中的日程忙闲（默认 true）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_external_calendar: Option<bool>,
    /// 是否只查询忙碌日程信息（默认 true）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_busy: Option<bool>,
    /// 是否需要 RSVP 状态信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_rsvp_status: Option<bool>,
}

/// 忙闲时间段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreebusyItem {
    /// 开始时间（Unix 时间戳，毫秒）
    pub start_time: i64,
    /// 结束时间（Unix 时间戳，毫秒）
    pub end_time: i64,
    /// 忙闲状态：0-空闲，1-忙碌
    pub status: i32,
    /// 日程标题（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// 用户忙闲信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFreebusy {
    /// 用户 ID
    pub user_id: String,
    /// 忙闲时间段列表
    pub freebusy_list: Vec<FreebusyItem>,
}

/// 批量查询忙闲信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchFreebusyResponse {
    /// 用户忙闲信息列表
    pub freebusy_list: Vec<UserFreebusy>,
}

/// 查询忙闲信息请求体（单用户）
///
/// time_min/time_max 必填；user_id 与 room_id 二选一传入，同时传入只生效 user_id。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFreebusyRequestBody {
    /// 查询时段开始时间（RFC 3339 date_time 格式），与 time_max 间隔不能大于 90 天
    pub time_min: String,
    /// 查询时段结束时间（RFC 3339 date_time 格式），与 time_min 间隔不能大于 90 天
    pub time_max: String,
    /// 用户 ID（与查询参数 user_id_type 匹配；user_id 与 room_id 二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 会议室 room_id（user_id 与 room_id 二选一，同时传入只生效 user_id）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    /// 是否包含绑定的三方日历中的日程（默认 true）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_external_calendar: Option<bool>,
    /// 是否只查询忙碌日程信息（默认 true）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_busy: Option<bool>,
    /// 是否需要 RSVP 状态信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_rsvp_status: Option<bool>,
}

/// 查询忙闲信息响应（单用户）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFreebusyResponse {
    /// 忙闲时间段列表
    pub freebusy_list: Vec<FreebusyItem>,
}

impl ApiResponseTrait for BatchFreebusyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ListFreebusyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
