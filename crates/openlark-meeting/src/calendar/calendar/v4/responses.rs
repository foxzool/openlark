//! 日历相关响应结构
//!
//! 定义日历 API 的响应数据类型。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 创建共享日历响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCalendarResponse {
    /// 日历 ID
    pub calendar_id: String,
}

impl ApiResponseTrait for CreateCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取日历列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCalendarResponse {
    /// 日历列表
    pub calendars: Vec<CalendarInfo>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页 token
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 日历信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarInfo {
    /// 日历 ID
    pub calendar_id: String,
    /// 日历名称
    pub name: String,
    /// 日历类型
    pub calendar_type: String,
    /// 描述
    pub description: Option<String>,
}

/// 删除日历响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCalendarResponse {}

impl ApiResponseTrait for DeleteCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取忙闲信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFreebusyResponse {
    /// 忙闲数据
    pub data: Vec<FreebusyItem>,
}

impl ApiResponseTrait for ListFreebusyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 忙闲数据项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreebusyItem {
    /// 用户 ID
    pub user_id: String,
    /// 忙闲时间段
    pub time_ranges: Vec<TimeRange>,
}

/// 时间范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// 开始时间
    pub start: String,
    /// 结束时间
    pub end: String,
}

/// 批量获取忙闲信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetFreebusyResponse {
    /// 忙闲数据
    pub data: Vec<FreebusyItem>,
}

impl ApiResponseTrait for BatchGetFreebusyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建 Exchange 绑定响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExchangeBindingResponse {
    /// 绑定 ID
    pub binding_id: String,
}

impl ApiResponseTrait for CreateExchangeBindingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除 Exchange 绑定响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteExchangeBindingResponse {}

impl ApiResponseTrait for DeleteExchangeBindingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取 Exchange 绑定响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExchangeBindingResponse {
    /// 绑定信息
    pub binding: ExchangeBindingInfo,
}

impl ApiResponseTrait for GetExchangeBindingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Exchange 绑定信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeBindingInfo {
    /// 绑定 ID
    pub binding_id: String,
    /// 用户邮箱
    pub email: String,
    /// 绑定状态
    pub status: String,
}
