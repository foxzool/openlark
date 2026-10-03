//! 日历 v4 响应结构
//!
//! 定义日历 API (v4) 的响应数据类型。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

// ==================== Calendar 资源 ====================

/// 日历数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarData {
    /// 日历 ID
    pub calendar_id: String,
    /// 日历摘要
    pub summary: String,
    /// 日历描述
    pub description: Option<String>,
    /// 日历颜色
    pub color: Option<String>,
    /// 权限
    pub permissions: Option<CalendarPermissions>,
    /// 是否为主日历
    pub primary: Option<bool>,
    /// 日历类型
    pub calendar_type: Option<String>,
}

/// 日历权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarPermissions {
    /// 是否可读
    pub is_readable: Option<bool>,
    /// 是否可写
    pub is_writable: Option<bool>,
}

/// 日历列表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarListData {
    /// 日历列表
    pub calendar: Vec<CalendarData>,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

// ==================== Calendar 响应 ====================

/// 获取日历信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetCalendarResponse {
    /// 日历数据。
    pub calendar: CalendarData,
}

/// 获取日历列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCalendarResponse {
    /// 响应数据。
    pub data: CalendarListData,
}

/// 创建日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCalendarResponse {
    /// 日历数据。
    pub calendar: CalendarData,
}

/// 删除日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteCalendarResponse {}

/// 更新日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchCalendarResponse {
    /// 日历数据。
    pub calendar: CalendarData,
}

/// 搜索日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchCalendarResponse {
    /// 响应数据。
    pub data: CalendarListData,
}

/// 批量获取日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MgetCalendarResponse {
    /// 响应数据。
    pub data: CalendarListData,
}

/// 设置主日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimaryCalendarResponse {
    /// 日历数据。
    pub calendar: CalendarData,
}

/// 获取主日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPrimaryCalendarResponse {
    /// 日历数据。
    pub calendar: CalendarData,
}

/// 订阅日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscribeCalendarResponse {}

/// 取消订阅日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnsubscribeCalendarResponse {}

// ==================== Event 资源 ====================

/// 日程数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventData {
    /// 日程 ID
    pub event_id: String,
    /// 日历 ID
    pub calendar_id: String,
    /// 日程摘要
    pub summary: String,
    /// 日程描述
    pub description: Option<String>,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
    /// 地点
    pub location: Option<String>,
    /// 是否全天
    pub is_all_day: Option<bool>,
    /// 状态
    pub status: Option<String>,
}

/// 参会人数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttendeeData {
    /// 参会人 ID
    pub attendee_id: String,
    /// 参会人类型
    pub attendee_type: Option<String>,
    /// 参会状态
    pub status: Option<String>,
}

// ==================== Event 响应 ====================

/// 获取日程信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetEventResponse {
    /// 日程数据。
    pub event: EventData,
}

/// 获取日程列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListEventResponse {
    /// 响应数据。
    pub data: EventListData,
}

/// 日程列表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventListData {
    /// 日程列表
    pub event: Vec<EventData>,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

/// 创建日程响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateEventResponse {
    /// 日程数据。
    pub event: EventData,
}

/// 更新日程响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchEventResponse {
    /// 日程数据。
    pub event: EventData,
}

/// 删除日程响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteEventResponse {}

/// 批量获取日程响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MgetEventResponse {
    /// 响应数据。
    pub data: EventListData,
}

/// 搜索日程响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchEventResponse {
    /// 响应数据。
    pub data: EventListData,
}

/// 获取日程实例视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventInstanceViewResponse {}

/// 获取日程实例响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventInstancesResponse {
    /// 响应数据。
    pub data: EventListData,
}

/// 回复日程响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplyEventResponse {}

// ==================== Event Attendee 响应 ====================

/// 获取参会人列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListAttendeeResponse {
    /// 响应数据。
    pub data: AttendeeListData,
}

/// 参会人列表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttendeeListData {
    /// 参会人列表
    pub attendees: Vec<AttendeeData>,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

/// 创建参会人响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAttendeeResponse {
    /// 参会人数据。
    pub attendee: AttendeeData,
}

/// 删除参会人响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteAttendeeResponse {}

/// 批量删除参会人响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteAttendeeResponse {}

/// 获取会议群成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttendeeChatMemberListResponse {
    /// 响应数据。
    pub data: ChatMemberListData,
}

/// 会议群成员列表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberListData {
    /// 成员列表
    pub members: Vec<ChatMemberData>,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

/// 会议群成员数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberData {
    /// 成员 ID
    pub user_id: String,
    /// 成员类型
    pub member_type: Option<String>,
}

// ==================== Freebusy 响应 ====================

/// 忙闲数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FreebusyData {
    /// 日历 ID
    pub calendar_id: String,
    /// 忙闲时间列表
    pub busy_times: Vec<BusyTime>,
}

/// 忙闲时间
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusyTime {
    /// 开始时间
    pub start: String,
    /// 结束时间
    pub end: String,
}

/// 查询忙闲响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryFreebusyResponse {
    /// 响应数据。
    pub data: FreebusyListData,
}

/// 忙闲列表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FreebusyListData {
    /// 忙闲列表
    pub freebusy: Vec<FreebusyData>,
}

/// 批量查询忙闲响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchFreebusyResponse {
    /// 响应数据。
    pub data: FreebusyListData,
}

// ==================== Setting 响应 ====================

/// 日历设置数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarSettingsData {
    /// 首选工作日
    pub preferred_work_days: Option<Vec<i32>>,
    /// 首选工作时间
    pub preferred_working_hours: Option<WorkingHours>,
}

/// 工作时间
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkingHours {
    /// 开始时间
    pub start: String,
    /// 结束时间
    pub end: String,
}

/// 获取日历设置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetCalendarSettingsResponse {
    /// 响应数据。
    pub data: CalendarSettingsData,
}

/// 更新日历设置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateCalendarSettingsResponse {
    /// 响应数据。
    pub data: CalendarSettingsData,
}

/// 生成 CalDAV 配置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenerateCaldavConfResponse {
    /// 响应数据。
    pub data: CaldavConfData,
}

/// CalDAV 配置数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CaldavConfData {
    /// CalDAV URL
    pub caldav_url: String,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

// ==================== Timeoff Event 响应 ====================

/// 请假日程数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeoffEventData {
    /// 请假日程 ID
    pub event_id: String,
    /// 日程摘要
    pub summary: String,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
    /// 请假类型
    pub timeoff_type: Option<String>,
}

/// 创建请假日程响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTimeoffEventResponse {
    /// 日程数据。
    pub event: TimeoffEventData,
}

/// 获取请假日程列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListTimeoffEventResponse {
    /// 响应数据。
    pub data: TimeoffEventListData,
}

/// 请假日程列表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeoffEventListData {
    /// 请假日程列表
    pub timeoff_events: Vec<TimeoffEventData>,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

/// 删除请假日程响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteTimeoffEventResponse {}

// ==================== Exchange Binding 响应 ====================

/// Exchange 绑定数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExchangeBindingData {
    /// 绑定 ID
    pub exchange_id: String,
    /// Exchange 邮箱
    pub email: String,
    /// 绑定状态
    pub status: Option<String>,
}

/// 获取 Exchange 绑定响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetExchangeBindingResponse {
    /// 响应数据。
    pub data: ExchangeBindingListData,
}

/// Exchange 绑定列表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExchangeBindingListData {
    /// 绑定列表
    pub exchange_bindings: Vec<ExchangeBindingData>,
}

/// 创建 Exchange 绑定响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateExchangeBindingResponse {
    /// exchange_binding 字段。
    pub exchange_binding: ExchangeBindingData,
}

/// 删除 Exchange 绑定响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteExchangeBindingResponse {}

// ==================== ACL 响应 ====================

/// ACL 规则数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AclData {
    /// ACL 类型
    pub acl_type: String,
    /// 规则 ID
    pub rule_id: String,
    /// 权限
    pub permission: String,
    /// 权限详情
    pub scope: Option<String>,
}

/// 获取 ACL 列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListAclResponse {
    /// 响应数据。
    pub data: AclListData,
}

/// ACL 列表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AclListData {
    /// ACL 列表
    pub acl_rules: Vec<AclData>,
}

/// 创建 ACL 响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAclResponse {
    /// 访问控制规则。
    pub acl: AclData,
}

/// 删除 ACL 响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteAclResponse {}

// ==================== Meeting Chat 响应 ====================

/// 获取会议群响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetMeetingChatResponse {
    /// 响应数据。
    pub data: MeetingChatData,
}

/// 会议群数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeetingChatData {
    /// 群 ID
    pub chat_id: String,
    /// 群名称
    pub name: String,
}

// ==================== Meeting Minute 响应 ====================

/// 会议纪要数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MeetingMinuteData {
    /// 纪要 ID
    pub minute_id: String,
    /// 纪要内容
    pub content: String,
    /// 创建时间
    pub create_time: String,
}

/// 获取会议纪要响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetMeetingMinuteResponse {
    /// 响应数据。
    pub data: MeetingMinuteData,
}

/// 更新会议纪要响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchMeetingMinuteResponse {
    /// 响应数据。
    pub data: MeetingMinuteData,
}

// ==================== ApiResponseTrait 实现 ====================

impl ApiResponseTrait for GetCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ListCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for PatchCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for SearchCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for MgetCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for PrimaryCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetPrimaryCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for SubscribeCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UnsubscribeCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ListEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for PatchEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for MgetEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for SearchEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for EventInstanceViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for EventInstancesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ReplyEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ListAttendeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateAttendeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteAttendeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for BatchDeleteAttendeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for AttendeeChatMemberListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for QueryFreebusyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for BatchFreebusyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetCalendarSettingsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateCalendarSettingsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GenerateCaldavConfResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateTimeoffEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ListTimeoffEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteTimeoffEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetExchangeBindingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateExchangeBindingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteExchangeBindingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ListAclResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateAclResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteAclResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetMeetingChatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetMeetingMinuteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for PatchMeetingMinuteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
