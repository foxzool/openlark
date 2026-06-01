//! API端点定义（类型安全枚举系统）
//!
//! 本模块提供基于枚举的 API 端点定义，用于生产代码中的类型安全调用。
//!
//! # 使用场景
//!
//! ## 生产代码（推荐）
//! 使用枚举端点获得编译时类型检查和动态 URL 生成能力：
//! ```rust
//! use openlark_meeting::common::api_endpoints::VcApiV1;
//!
//! let room_id = "room_123".to_string();
//! let endpoint = VcApiV1::RoomGet(room_id);
//! let url = endpoint.to_url(); // 类型安全，动态生成
//! assert!(url.contains("/open-apis/vc/v1/rooms/"));
//! ```
//!
//! # 特性
//! - ✅ **类型安全**：编译时验证参数
//! - ✅ **动态生成**：支持参数化 URL
//! - ✅ **易于维护**：集中管理端点定义
//! - ✅ **避免错误**：消除字符串拼接错误

// ==================== Calendar（日历）v4 ====================

/// Calendar API V4 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum CalendarApiV4 {
    // ========== Calendar 资源 ==========
    /// 获取日历信息
    /// GET /open-apis/calendar/v4/calendars/:calendar_id
    CalendarGet(String),

    /// 创建日历
    /// POST /open-apis/calendar/v4/calendars
    CalendarCreate,

    /// 删除日历
    /// DELETE /open-apis/calendar/v4/calendars/:calendar_id
    CalendarDelete(String),

    /// 更新日历
    /// PATCH /open-apis/calendar/v4/calendars/:calendar_id
    CalendarPatch(String),

    /// 获取日历列表
    /// GET /open-apis/calendar/v4/calendars
    CalendarList,

    /// 批量获取日历信息
    /// POST /open-apis/calendar/v4/calendars/batch_get
    CalendarBatchGet,

    /// 搜索日历
    /// POST /open-apis/calendar/v4/calendars/search
    CalendarSearch,

    /// 批量获取日历
    /// POST /open-apis/calendar/v4/calendars/mget
    CalendarMget,

    /// 订阅日历变更事件
    /// POST /open-apis/calendar/v4/calendars/subscription
    CalendarSubscription,

    /// 设置主日历
    /// PUT /open-apis/calendar/v4/calendars/primary
    CalendarPrimary,

    /// 获取主日历
    /// GET /open-apis/calendar/v4/calendars/primary
    CalendarPrimaryGet,

    // ========== Event 日程管理 ==========
    /// 获取日程信息
    /// GET /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id
    EventGet(String, String),

    /// 创建日程
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/events
    EventCreate(String),

    /// 删除日程
    /// DELETE /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id
    EventDelete(String, String),

    /// 更新日程
    /// PATCH /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id
    EventPatch(String, String),

    /// 获取日程列表
    /// GET /open-apis/calendar/v4/calendars/:calendar_id/events
    EventList(String),

    /// 搜索日程
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/events/search
    EventSearch(String),

    /// 批量获取日程信息
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/events/batch_get
    EventBatchGet(String),

    /// 回复日程
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/reply
    EventReply(String, String),

    // ========== ACL 访问控制 ==========
    /// 创建访问控制
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/acls
    CalendarAclCreate(String),

    /// 订阅日历访问控制变更事件
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/acls/subscription
    CalendarAclSubscription(String),

    /// 取消订阅日历访问控制变更事件
    /// POST /open-apis/calendar/v4/calendars/:calendar_id/acls/unsubscription
    CalendarAclUnsubscription(String),

    // ========== Exchange 绑定 ==========
    /// 将 Exchange 账户绑定到飞书账户
    /// POST /open-apis/calendar/v4/exchange_bindings
    ExchangeBindingCreate,

    // ========== 请假日程 ==========
    /// 创建请假日程
    /// POST /open-apis/calendar/v4/timeoff_events
    TimeoffEventCreate,

    // ========== 忙闲查询 ==========
    /// 批量查询主日历日程忙闲信息
    /// POST /open-apis/calendar/v4/freebusy/batch
    FreebusyBatch,
}

impl CalendarApiV4 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // Calendar 资源
            CalendarApiV4::CalendarGet(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}")
            }
            CalendarApiV4::CalendarCreate => "/open-apis/calendar/v4/calendars".to_string(),
            CalendarApiV4::CalendarDelete(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}")
            }
            CalendarApiV4::CalendarPatch(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}")
            }
            CalendarApiV4::CalendarList => "/open-apis/calendar/v4/calendars".to_string(),
            CalendarApiV4::CalendarBatchGet => {
                "/open-apis/calendar/v4/calendars/batch_get".to_string()
            }
            CalendarApiV4::CalendarSearch => "/open-apis/calendar/v4/calendars/search".to_string(),
            CalendarApiV4::CalendarMget => "/open-apis/calendar/v4/calendars/mget".to_string(),
            CalendarApiV4::CalendarSubscription => {
                "/open-apis/calendar/v4/calendars/subscription".to_string()
            }
            CalendarApiV4::CalendarPrimary => {
                "/open-apis/calendar/v4/calendars/primary".to_string()
            }
            CalendarApiV4::CalendarPrimaryGet => {
                "/open-apis/calendar/v4/calendars/primary".to_string()
            }

            // Event
            CalendarApiV4::EventGet(calendar_id, event_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}")
            }
            CalendarApiV4::EventCreate(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}/events")
            }
            CalendarApiV4::EventDelete(calendar_id, event_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}")
            }
            CalendarApiV4::EventPatch(calendar_id, event_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}")
            }
            CalendarApiV4::EventList(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}/events")
            }
            CalendarApiV4::EventSearch(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/search")
            }
            CalendarApiV4::EventBatchGet(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/batch_get")
            }
            CalendarApiV4::EventReply(calendar_id, event_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/reply")
            }

            // ACL
            CalendarApiV4::CalendarAclCreate(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}/acls")
            }
            CalendarApiV4::CalendarAclSubscription(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}/acls/subscription")
            }
            CalendarApiV4::CalendarAclUnsubscription(calendar_id) => {
                format!("/open-apis/calendar/v4/calendars/{calendar_id}/acls/unsubscription")
            }

            // Exchange Binding
            CalendarApiV4::ExchangeBindingCreate => {
                "/open-apis/calendar/v4/exchange_bindings".to_string()
            }

            // Timeoff Event
            CalendarApiV4::TimeoffEventCreate => {
                "/open-apis/calendar/v4/timeoff_events".to_string()
            }

            // Freebusy
            CalendarApiV4::FreebusyBatch => "/open-apis/calendar/v4/freebusy/batch".to_string(),
        }
    }
}

// ==================== VC（视频会议）v1 ====================

/// VC API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum VcApiV1 {
    // ========== Room 会议室 ==========
    /// 创建会议室
    /// POST /open-apis/vc/v1/rooms
    RoomCreate,

    /// 获取会议室
    /// GET /open-apis/vc/v1/rooms/:room_id
    RoomGet(String),

    /// 删除会议室
    /// DELETE /open-apis/vc/v1/rooms/:room_id
    RoomDelete(String),

    /// 获取会议室列表
    /// GET /open-apis/vc/v1/rooms
    RoomList,

    /// 批量获取会议室
    /// POST /open-apis/vc/v1/rooms/batch_get
    RoomBatchGet,

    /// 搜索会议室
    /// POST /open-apis/vc/v1/rooms/search
    RoomSearch,

    /// 更新会议室
    /// PATCH /open-apis/vc/v1/rooms/:room_id
    RoomPatch(String),

    // ========== Meeting 会议 ==========
    /// 获取会议
    /// GET /open-apis/vc/v1/meetings/:meeting_id
    MeetingGet(String),

    /// 邀请会议
    /// POST /open-apis/vc/v1/meetings/:meeting_id/invite
    MeetingInvite(String),

    /// 踢出会议
    /// POST /open-apis/vc/v1/meetings/:meeting_id/kickout
    MeetingKickout(String),

    /// 结束会议
    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/end
    MeetingEnd(String),

    /// 设置主持人
    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/set_host
    MeetingSetHost(String),

    /// 获取会议列表
    /// GET /open-apis/vc/v1/meetings
    MeetingList,

    /// 订阅会议变更事件
    /// POST /open-apis/vc/v1/meetings/subscription
    MeetingSubscription,

    /// 取消订阅会议变更事件
    /// POST /open-apis/vc/v1/meetings/unsubscription
    MeetingUnsubscription,

    /// 获取会议录制
    /// GET /open-apis/vc/v1/meetings/:meeting_id/recordings/:recording_id
    MeetingRecordingGet(String, String),

    /// 获取会议录制列表
    /// GET /open-apis/vc/v1/meetings/:meeting_id/recordings
    MeetingRecordingList(String),

    /// 开始录制
    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/recording/start
    MeetingRecordingStart(String),

    /// 停止录制
    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/recording/stop
    MeetingRecordingStop(String),

    /// 设置录制权限
    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/recording/set_permission
    MeetingRecordingSetPermission(String),

    // ========== Reserve 预约 ==========
    /// 预约会议
    /// POST /open-apis/vc/v1/reserves
    ReserveCreate,

    /// 删除预约
    /// DELETE /open-apis/vc/v1/reserves/:reserve_id
    ReserveDelete(String),

    /// 获取预约
    /// GET /open-apis/vc/v1/reserves/:reserve_id
    ReserveGet(String),

    /// 获取预约活跃会议
    /// GET /open-apis/vc/v1/reserves/:reserve_id/active_meeting
    ReserveGetActiveMeeting(String),

    /// 更新预约
    /// PATCH /open-apis/vc/v1/reserves/:reserve_id
    ReservePatch(String),

    // ========== Report 报表 ==========
    /// 获取日报表
    /// GET /open-apis/vc/v1/reports/daily
    ReportDailyGet,

    /// 获取 Top 用户
    /// GET /open-apis/vc/v1/reports/top_user
    ReportTopUserGet,

    // ========== Export 导出 ==========
    /// 获取导出
    /// GET /open-apis/vc/v1/exports/:export_id
    ExportGet(String),

    /// 下载导出
    /// GET /open-apis/vc/v1/exports/:export_id/download
    ExportDownload(String),

    /// 获取会议列表导出
    /// GET /open-apis/vc/v1/exports/meeting_list
    ExportMeetingList,

    /// 获取参会人列表导出
    /// GET /open-apis/vc/v1/exports/participant_list
    ExportParticipantList,

    /// 获取参会人质量列表导出
    /// GET /open-apis/vc/v1/exports/participant_quality_list
    ExportParticipantQualityList,

    /// 获取资源预约列表导出
    /// GET /open-apis/vc/v1/exports/resource_reservation_list
    ExportResourceReservationList,

    // ========== Room Level 会议室层级 ==========
    /// 创建会议室层级
    /// POST /open-apis/vc/v1/room_levels
    RoomLevelCreate,

    /// 删除会议室层级
    /// DELETE /open-apis/vc/v1/room_levels/:room_level_id
    RoomLevelDelete(String),

    /// 获取会议室层级
    /// GET /open-apis/vc/v1/room_levels/:room_level_id
    RoomLevelGet(String),

    /// 获取会议室层级列表
    /// GET /open-apis/vc/v1/room_levels
    RoomLevelList,

    /// 更新会议室层级
    /// PATCH /open-apis/vc/v1/room_levels/:room_level_id
    RoomLevelPatch(String),

    // ========== Room Config 会议室配置 ==========
    /// 创建会议室配置
    /// POST /open-apis/vc/v1/room_configs
    RoomConfigCreate,

    /// 删除会议室配置
    /// DELETE /open-apis/vc/v1/room_configs/:room_config_id
    RoomConfigDelete(String),

    /// 获取会议室配置
    /// GET /open-apis/vc/v1/room_configs/:room_config_id
    RoomConfigGet(String),

    /// 获取会议室配置列表
    /// GET /open-apis/vc/v1/room_configs
    RoomConfigList,

    /// 更新会议室配置
    /// PATCH /open-apis/vc/v1/room_configs/:room_config_id
    RoomConfigPatch(String),

    /// 创建会议室部署码
    /// POST /open-apis/vc/v1/room_configs/set_room_access_code
    RoomConfigSetRoomAccessCode,

    // ========== Reserve Config 预约配置 ==========
    /// 创建预约配置
    /// POST /open-apis/vc/v1/reserve_configs
    ReserveConfigCreate,

    /// 删除预约配置
    /// DELETE /open-apis/vc/v1/reserve_configs/:reserve_config_id
    ReserveConfigDelete(String),

    /// 获取预约配置
    /// GET /open-apis/vc/v1/reserve_configs/:reserve_config_id
    ReserveConfigGet(String),

    /// 获取预约配置列表
    /// GET /open-apis/vc/v1/reserve_configs
    ReserveConfigList,

    /// 更新预约配置
    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id
    ReserveConfigPatch(String),

    /// 更新预约配置管理员
    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id/admin
    ReserveConfigAdminPatch(String),

    /// 更新会议室预定表单
    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id/form
    ReserveConfigFormPatch(String),

    /// 更新禁用状态变更通知
    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform
    ReserveConfigDisableInformPatch(String),

    // ========== Scope Config 范围配置 ==========
    /// 创建范围配置
    /// POST /open-apis/vc/v1/scope_config
    ScopeConfigCreate,

    /// 删除范围配置
    /// DELETE /open-apis/vc/v1/scope_config/:scope_config_id
    ScopeConfigDelete(String),

    /// 获取范围配置
    /// GET /open-apis/vc/v1/scope_config/:scope_config_id
    ScopeConfigGet(String),

    /// 获取范围配置列表
    /// GET /open-apis/vc/v1/scope_config
    ScopeConfigList,

    /// 更新范围配置
    /// PATCH /open-apis/vc/v1/scope_config/:scope_config_id
    ScopeConfigPatch(String),

    // ========== Alert 告警 ==========
    /// 获取告警列表
    /// GET /open-apis/vc/v1/alerts
    AlertList,

    /// 获取告警
    /// GET /open-apis/vc/v1/alerts/:alert_id
    AlertGet(String),

    // ========== List 列表 ==========
    /// 获取会议列表
    /// GET /open-apis/vc/v1/meeting_list
    MeetingListList,

    /// 获取参会人列表
    /// GET /open-apis/vc/v1/participant_list
    ParticipantListList,

    /// 获取参会人质量列表
    /// GET /open-apis/vc/v1/participant_quality_list
    ParticipantQualityListList,

    /// 获取资源预约列表
    /// GET /open-apis/vc/v1/resource_reservation_list
    ResourceReservationListList,
}

impl VcApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // Room
            VcApiV1::RoomCreate => "/open-apis/vc/v1/rooms".to_string(),
            VcApiV1::RoomGet(room_id) => format!("/open-apis/vc/v1/rooms/{room_id}"),
            VcApiV1::RoomDelete(room_id) => format!("/open-apis/vc/v1/rooms/{room_id}"),
            VcApiV1::RoomList => "/open-apis/vc/v1/rooms".to_string(),
            VcApiV1::RoomBatchGet => "/open-apis/vc/v1/rooms/batch_get".to_string(),
            VcApiV1::RoomSearch => "/open-apis/vc/v1/rooms/search".to_string(),
            VcApiV1::RoomPatch(room_id) => format!("/open-apis/vc/v1/rooms/{room_id}"),

            // Meeting
            VcApiV1::MeetingGet(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{meeting_id}")
            }
            VcApiV1::MeetingInvite(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{meeting_id}/invite")
            }
            VcApiV1::MeetingKickout(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{meeting_id}/kickout")
            }
            VcApiV1::MeetingEnd(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{meeting_id}/end")
            }
            VcApiV1::MeetingSetHost(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{meeting_id}/set_host")
            }
            VcApiV1::MeetingList => "/open-apis/vc/v1/meetings".to_string(),
            VcApiV1::MeetingSubscription => "/open-apis/vc/v1/meetings/subscription".to_string(),
            VcApiV1::MeetingUnsubscription => {
                "/open-apis/vc/v1/meetings/unsubscription".to_string()
            }
            VcApiV1::MeetingRecordingGet(meeting_id, recording_id) => {
                format!("/open-apis/vc/v1/meetings/{meeting_id}/recordings/{recording_id}")
            }
            VcApiV1::MeetingRecordingList(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{meeting_id}/recordings")
            }
            VcApiV1::MeetingRecordingStart(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{meeting_id}/recording/start")
            }
            VcApiV1::MeetingRecordingStop(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{meeting_id}/recording/stop")
            }
            VcApiV1::MeetingRecordingSetPermission(meeting_id) => {
                format!("/open-apis/vc/v1/meetings/{meeting_id}/recording/set_permission")
            }

            // Reserve
            VcApiV1::ReserveCreate => "/open-apis/vc/v1/reserves".to_string(),
            VcApiV1::ReserveDelete(reserve_id) => {
                format!("/open-apis/vc/v1/reserves/{reserve_id}")
            }
            VcApiV1::ReserveGet(reserve_id) => {
                format!("/open-apis/vc/v1/reserves/{reserve_id}")
            }
            VcApiV1::ReserveGetActiveMeeting(reserve_id) => {
                format!("/open-apis/vc/v1/reserves/{reserve_id}/active_meeting")
            }
            VcApiV1::ReservePatch(reserve_id) => {
                format!("/open-apis/vc/v1/reserves/{reserve_id}")
            }

            // Report
            VcApiV1::ReportDailyGet => "/open-apis/vc/v1/reports/daily".to_string(),
            VcApiV1::ReportTopUserGet => "/open-apis/vc/v1/reports/top_user".to_string(),

            // Export
            VcApiV1::ExportGet(export_id) => {
                format!("/open-apis/vc/v1/exports/{export_id}")
            }
            VcApiV1::ExportDownload(export_id) => {
                format!("/open-apis/vc/v1/exports/{export_id}/download")
            }
            VcApiV1::ExportMeetingList => "/open-apis/vc/v1/exports/meeting_list".to_string(),
            VcApiV1::ExportParticipantList => {
                "/open-apis/vc/v1/exports/participant_list".to_string()
            }
            VcApiV1::ExportParticipantQualityList => {
                "/open-apis/vc/v1/exports/participant_quality_list".to_string()
            }
            VcApiV1::ExportResourceReservationList => {
                "/open-apis/vc/v1/exports/resource_reservation_list".to_string()
            }

            // Room Level
            VcApiV1::RoomLevelCreate => "/open-apis/vc/v1/room_levels".to_string(),
            VcApiV1::RoomLevelDelete(room_level_id) => {
                format!("/open-apis/vc/v1/room_levels/{room_level_id}")
            }
            VcApiV1::RoomLevelGet(room_level_id) => {
                format!("/open-apis/vc/v1/room_levels/{room_level_id}")
            }
            VcApiV1::RoomLevelList => "/open-apis/vc/v1/room_levels".to_string(),
            VcApiV1::RoomLevelPatch(room_level_id) => {
                format!("/open-apis/vc/v1/room_levels/{room_level_id}")
            }

            // Room Config
            VcApiV1::RoomConfigCreate => "/open-apis/vc/v1/room_configs".to_string(),
            VcApiV1::RoomConfigDelete(room_config_id) => {
                format!("/open-apis/vc/v1/room_configs/{room_config_id}")
            }
            VcApiV1::RoomConfigGet(room_config_id) => {
                format!("/open-apis/vc/v1/room_configs/{room_config_id}")
            }
            VcApiV1::RoomConfigList => "/open-apis/vc/v1/room_configs".to_string(),
            VcApiV1::RoomConfigPatch(room_config_id) => {
                format!("/open-apis/vc/v1/room_configs/{room_config_id}")
            }
            VcApiV1::RoomConfigSetRoomAccessCode => {
                "/open-apis/vc/v1/room_configs/set_room_access_code".to_string()
            }

            // Reserve Config
            VcApiV1::ReserveConfigCreate => "/open-apis/vc/v1/reserve_configs".to_string(),
            VcApiV1::ReserveConfigDelete(reserve_config_id) => {
                format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}")
            }
            VcApiV1::ReserveConfigGet(reserve_config_id) => {
                format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}")
            }
            VcApiV1::ReserveConfigList => "/open-apis/vc/v1/reserve_configs".to_string(),
            VcApiV1::ReserveConfigPatch(reserve_config_id) => {
                format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}")
            }
            VcApiV1::ReserveConfigAdminPatch(reserve_config_id) => {
                format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/admin")
            }
            VcApiV1::ReserveConfigFormPatch(reserve_config_id) => {
                format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/form")
            }
            VcApiV1::ReserveConfigDisableInformPatch(reserve_config_id) => {
                format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/disable_inform")
            }

            // Scope Config
            VcApiV1::ScopeConfigCreate => "/open-apis/vc/v1/scope_config".to_string(),
            VcApiV1::ScopeConfigDelete(scope_config_id) => {
                format!("/open-apis/vc/v1/scope_config/{scope_config_id}")
            }
            VcApiV1::ScopeConfigGet(scope_config_id) => {
                format!("/open-apis/vc/v1/scope_config/{scope_config_id}")
            }
            VcApiV1::ScopeConfigList => "/open-apis/vc/v1/scope_config".to_string(),
            VcApiV1::ScopeConfigPatch(scope_config_id) => {
                format!("/open-apis/vc/v1/scope_config/{scope_config_id}")
            }

            // Alert
            VcApiV1::AlertList => "/open-apis/vc/v1/alerts".to_string(),
            VcApiV1::AlertGet(alert_id) => format!("/open-apis/vc/v1/alerts/{alert_id}"),

            // List
            VcApiV1::MeetingListList => "/open-apis/vc/v1/meeting_list".to_string(),
            VcApiV1::ParticipantListList => "/open-apis/vc/v1/participant_list".to_string(),
            VcApiV1::ParticipantQualityListList => {
                "/open-apis/vc/v1/participant_quality_list".to_string()
            }
            VcApiV1::ResourceReservationListList => {
                "/open-apis/vc/v1/resource_reservation_list".to_string()
            }
        }
    }
}

// ==================== Meeting Room（会议室）old ====================

/// Meeting Room API 端点枚举（历史版本）
#[derive(Debug, Clone, PartialEq)]
pub enum MeetingRoomApi {
    // ========== Building 建筑物 ==========
    /// 创建建筑物
    /// POST /open-apis/meeting_room/buildings
    BuildingCreate,

    /// 删除建筑物
    /// DELETE /open-apis/meeting_room/buildings/:building_id
    BuildingDelete(String),

    /// 获取建筑物
    /// GET /open-apis/meeting_room/buildings/:building_id
    BuildingGet(String),

    /// 获取建筑物列表
    /// GET /open-apis/meeting_room/buildings
    BuildingList,

    /// 更新建筑物
    /// PATCH /open-apis/meeting_room/buildings/:building_id
    BuildingPatch(String),

    /// 批量获取建筑物
    /// POST /open-apis/meeting_room/buildings/batch_get
    BuildingBatchGet,

    /// 批量获取建筑物 ID
    /// POST /open-apis/meeting_room/buildings/batch_get_id
    BuildingBatchGetId,

    // ========== Room 会议室 ==========
    /// 创建会议室
    /// POST /open-apis/meeting_room/rooms
    RoomCreate,

    /// 删除会议室
    /// DELETE /open-apis/meeting_room/rooms/:room_id
    RoomDelete(String),

    /// 获取会议室
    /// GET /open-apis/meeting_room/rooms/:room_id
    RoomGet(String),

    /// 获取会议室列表
    /// GET /open-apis/meeting_room/rooms
    RoomList,

    /// 更新会议室
    /// PATCH /open-apis/meeting_room/rooms/:room_id
    RoomPatch(String),

    /// 批量获取会议室
    /// POST /open-apis/meeting_room/rooms/batch_get
    RoomBatchGet,

    /// 批量获取会议室 ID
    /// POST /open-apis/meeting_room/rooms/batch_get_id
    RoomBatchGetId,

    // ========== Instance 实例 ==========
    /// 批量获取忙碌情况
    /// POST /open-apis/meeting_room/instances/batch_get_freebusy
    InstanceBatchGetFreebusy,

    /// 回复实例
    /// POST /open-apis/meeting_room/instances/:instance_id/reply
    InstanceReply(String),

    // ========== 其他 ==========
    /// 获取国家列表
    /// GET /open-apis/meeting_room/countries
    CountryList,

    /// 获取地区列表
    /// GET /open-apis/meeting_room/districts
    DistrictList,

    /// 批量获取摘要
    /// POST /open-apis/meeting_room/rooms/batch_get_summary
    RoomBatchGetSummary,

    /// 更新会议室（旧版本 API）
    /// POST /open-apis/meeting_room/room/update
    RoomUpdate,

    /// 回复会议室日程实例（旧版本 API）
    /// POST /open-apis/meeting_room/instance/reply
    InstanceReplyOld,
}

impl MeetingRoomApi {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // Building
            MeetingRoomApi::BuildingCreate => "/open-apis/meeting_room/buildings".to_string(),
            MeetingRoomApi::BuildingDelete(building_id) => {
                format!("/open-apis/meeting_room/buildings/{building_id}")
            }
            MeetingRoomApi::BuildingGet(building_id) => {
                format!("/open-apis/meeting_room/buildings/{building_id}")
            }
            MeetingRoomApi::BuildingList => "/open-apis/meeting_room/buildings".to_string(),
            MeetingRoomApi::BuildingPatch(building_id) => {
                format!("/open-apis/meeting_room/buildings/{building_id}")
            }
            MeetingRoomApi::BuildingBatchGet => {
                "/open-apis/meeting_room/buildings/batch_get".to_string()
            }
            MeetingRoomApi::BuildingBatchGetId => {
                "/open-apis/meeting_room/buildings/batch_get_id".to_string()
            }

            // Room
            MeetingRoomApi::RoomCreate => "/open-apis/meeting_room/rooms".to_string(),
            MeetingRoomApi::RoomDelete(room_id) => {
                format!("/open-apis/meeting_room/rooms/{room_id}")
            }
            MeetingRoomApi::RoomGet(room_id) => {
                format!("/open-apis/meeting_room/rooms/{room_id}")
            }
            MeetingRoomApi::RoomList => "/open-apis/meeting_room/rooms".to_string(),
            MeetingRoomApi::RoomPatch(room_id) => {
                format!("/open-apis/meeting_room/rooms/{room_id}")
            }
            MeetingRoomApi::RoomBatchGet => "/open-apis/meeting_room/rooms/batch_get".to_string(),
            MeetingRoomApi::RoomBatchGetId => {
                "/open-apis/meeting_room/rooms/batch_get_id".to_string()
            }

            // Instance
            MeetingRoomApi::InstanceBatchGetFreebusy => {
                "/open-apis/meeting_room/instances/batch_get_freebusy".to_string()
            }
            MeetingRoomApi::InstanceReply(instance_id) => {
                format!("/open-apis/meeting_room/instances/{instance_id}/reply")
            }

            // 其他
            MeetingRoomApi::CountryList => "/open-apis/meeting_room/countries".to_string(),
            MeetingRoomApi::DistrictList => "/open-apis/meeting_room/districts".to_string(),
            MeetingRoomApi::RoomBatchGetSummary => {
                "/open-apis/meeting_room/rooms/batch_get_summary".to_string()
            }
            MeetingRoomApi::RoomUpdate => "/open-apis/meeting_room/room/update".to_string(),
            MeetingRoomApi::InstanceReplyOld => {
                "/open-apis/meeting_room/instance/reply".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendarapiv4_to_url_coverage() {
        let urls = vec![
            CalendarApiV4::CalendarGet("id1".to_string()).to_url(),
            CalendarApiV4::CalendarCreate.to_url(),
            CalendarApiV4::CalendarDelete("id1".to_string()).to_url(),
            CalendarApiV4::CalendarPatch("id1".to_string()).to_url(),
            CalendarApiV4::CalendarList.to_url(),
            CalendarApiV4::CalendarBatchGet.to_url(),
            CalendarApiV4::CalendarSearch.to_url(),
            CalendarApiV4::CalendarMget.to_url(),
            CalendarApiV4::CalendarSubscription.to_url(),
            CalendarApiV4::CalendarPrimary.to_url(),
            CalendarApiV4::CalendarPrimaryGet.to_url(),
            CalendarApiV4::EventGet("id1".to_string(), "id2".to_string()).to_url(),
            CalendarApiV4::EventCreate("id1".to_string()).to_url(),
            CalendarApiV4::EventDelete("id1".to_string(), "id2".to_string()).to_url(),
            CalendarApiV4::EventPatch("id1".to_string(), "id2".to_string()).to_url(),
            CalendarApiV4::EventList("id1".to_string()).to_url(),
            CalendarApiV4::EventSearch("id1".to_string()).to_url(),
            CalendarApiV4::EventBatchGet("id1".to_string()).to_url(),
            CalendarApiV4::EventReply("id1".to_string(), "id2".to_string()).to_url(),
            CalendarApiV4::CalendarAclCreate("id1".to_string()).to_url(),
            CalendarApiV4::CalendarAclSubscription("id1".to_string()).to_url(),
            CalendarApiV4::CalendarAclUnsubscription("id1".to_string()).to_url(),
            CalendarApiV4::ExchangeBindingCreate.to_url(),
            CalendarApiV4::TimeoffEventCreate.to_url(),
            CalendarApiV4::FreebusyBatch.to_url(),
        ];
        assert!(urls.iter().all(|url| url.starts_with("/open-apis/")));
    }

    #[test]
    fn test_vcapiv1_to_url_coverage() {
        let urls = vec![
            VcApiV1::RoomCreate.to_url(),
            VcApiV1::RoomGet("id1".to_string()).to_url(),
            VcApiV1::RoomDelete("id1".to_string()).to_url(),
            VcApiV1::RoomList.to_url(),
            VcApiV1::RoomBatchGet.to_url(),
            VcApiV1::RoomSearch.to_url(),
            VcApiV1::RoomPatch("id1".to_string()).to_url(),
            VcApiV1::MeetingGet("id1".to_string()).to_url(),
            VcApiV1::MeetingInvite("id1".to_string()).to_url(),
            VcApiV1::MeetingKickout("id1".to_string()).to_url(),
            VcApiV1::MeetingEnd("id1".to_string()).to_url(),
            VcApiV1::MeetingSetHost("id1".to_string()).to_url(),
            VcApiV1::MeetingList.to_url(),
            VcApiV1::MeetingSubscription.to_url(),
            VcApiV1::MeetingUnsubscription.to_url(),
            VcApiV1::MeetingRecordingGet("id1".to_string(), "id2".to_string()).to_url(),
            VcApiV1::MeetingRecordingList("id1".to_string()).to_url(),
            VcApiV1::MeetingRecordingStart("id1".to_string()).to_url(),
            VcApiV1::MeetingRecordingStop("id1".to_string()).to_url(),
            VcApiV1::MeetingRecordingSetPermission("id1".to_string()).to_url(),
            VcApiV1::ReserveCreate.to_url(),
            VcApiV1::ReserveDelete("id1".to_string()).to_url(),
            VcApiV1::ReserveGet("id1".to_string()).to_url(),
            VcApiV1::ReserveGetActiveMeeting("id1".to_string()).to_url(),
            VcApiV1::ReservePatch("id1".to_string()).to_url(),
            VcApiV1::ReportDailyGet.to_url(),
            VcApiV1::ReportTopUserGet.to_url(),
            VcApiV1::ExportGet("id1".to_string()).to_url(),
            VcApiV1::ExportDownload("id1".to_string()).to_url(),
            VcApiV1::ExportMeetingList.to_url(),
            VcApiV1::ExportParticipantList.to_url(),
            VcApiV1::ExportParticipantQualityList.to_url(),
            VcApiV1::ExportResourceReservationList.to_url(),
            VcApiV1::RoomLevelCreate.to_url(),
            VcApiV1::RoomLevelDelete("id1".to_string()).to_url(),
            VcApiV1::RoomLevelGet("id1".to_string()).to_url(),
            VcApiV1::RoomLevelList.to_url(),
            VcApiV1::RoomLevelPatch("id1".to_string()).to_url(),
            VcApiV1::RoomConfigCreate.to_url(),
            VcApiV1::RoomConfigDelete("id1".to_string()).to_url(),
            VcApiV1::RoomConfigGet("id1".to_string()).to_url(),
            VcApiV1::RoomConfigList.to_url(),
            VcApiV1::RoomConfigPatch("id1".to_string()).to_url(),
            VcApiV1::RoomConfigSetRoomAccessCode.to_url(),
            VcApiV1::ReserveConfigCreate.to_url(),
            VcApiV1::ReserveConfigDelete("id1".to_string()).to_url(),
            VcApiV1::ReserveConfigGet("id1".to_string()).to_url(),
            VcApiV1::ReserveConfigList.to_url(),
            VcApiV1::ReserveConfigPatch("id1".to_string()).to_url(),
            VcApiV1::ReserveConfigAdminPatch("id1".to_string()).to_url(),
            VcApiV1::ReserveConfigFormPatch("id1".to_string()).to_url(),
            VcApiV1::ReserveConfigDisableInformPatch("id1".to_string()).to_url(),
            VcApiV1::ScopeConfigCreate.to_url(),
            VcApiV1::ScopeConfigDelete("id1".to_string()).to_url(),
            VcApiV1::ScopeConfigGet("id1".to_string()).to_url(),
            VcApiV1::ScopeConfigList.to_url(),
            VcApiV1::ScopeConfigPatch("id1".to_string()).to_url(),
            VcApiV1::AlertList.to_url(),
            VcApiV1::AlertGet("id1".to_string()).to_url(),
            VcApiV1::MeetingListList.to_url(),
            VcApiV1::ParticipantListList.to_url(),
            VcApiV1::ParticipantQualityListList.to_url(),
            VcApiV1::ResourceReservationListList.to_url(),
        ];
        assert!(urls.iter().all(|url| url.starts_with("/open-apis/")));
    }

    #[test]
    fn test_meetingroomapi_to_url_coverage() {
        let urls = vec![
            MeetingRoomApi::BuildingCreate.to_url(),
            MeetingRoomApi::BuildingDelete("id1".to_string()).to_url(),
            MeetingRoomApi::BuildingGet("id1".to_string()).to_url(),
            MeetingRoomApi::BuildingList.to_url(),
            MeetingRoomApi::BuildingPatch("id1".to_string()).to_url(),
            MeetingRoomApi::BuildingBatchGet.to_url(),
            MeetingRoomApi::BuildingBatchGetId.to_url(),
            MeetingRoomApi::RoomCreate.to_url(),
            MeetingRoomApi::RoomDelete("id1".to_string()).to_url(),
            MeetingRoomApi::RoomGet("id1".to_string()).to_url(),
            MeetingRoomApi::RoomList.to_url(),
            MeetingRoomApi::RoomPatch("id1".to_string()).to_url(),
            MeetingRoomApi::RoomBatchGet.to_url(),
            MeetingRoomApi::RoomBatchGetId.to_url(),
            MeetingRoomApi::InstanceBatchGetFreebusy.to_url(),
            MeetingRoomApi::InstanceReply("id1".to_string()).to_url(),
            MeetingRoomApi::CountryList.to_url(),
            MeetingRoomApi::DistrictList.to_url(),
            MeetingRoomApi::RoomBatchGetSummary.to_url(),
            MeetingRoomApi::RoomUpdate.to_url(),
            MeetingRoomApi::InstanceReplyOld.to_url(),
        ];
        assert!(urls.iter().all(|url| url.starts_with("/open-apis/")));
    }
}
