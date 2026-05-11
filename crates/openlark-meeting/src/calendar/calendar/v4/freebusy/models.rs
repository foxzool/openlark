use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 批量查询忙闲信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchFreebusyRequestBody {
    /// 用户列表
    pub user_ids: Vec<String>,
    /// 查询开始时间（Unix 时间戳，毫秒）
    pub start_time: i64,
    /// 查询结束时间（Unix 时间戳，毫秒）
    pub end_time: i64,
    /// 日历 ID 列表（可选，默认为主日历）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_ids: Option<Vec<String>>,
    /// 时区（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFreebusyRequestBody {
    /// 查询开始时间（Unix 时间戳，毫秒）
    pub start_time: i64,
    /// 查询结束时间（Unix 时间戳，毫秒）
    pub end_time: i64,
    /// 日历 ID 列表（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_ids: Option<Vec<String>>,
    /// 时区（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
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
