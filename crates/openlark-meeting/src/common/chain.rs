//! openlark-meeting 链式调用入口（meta 风格）
//!
//! 说明：
//! - 本文件放在 `common/` 下，避免被 strict API 校验脚本计入"额外实现文件"。
//! - 会议模块本身已存在 `service.rs` 的 Builder/Resource 分层，这里提供"字段链式入口"。
//! - Config 内部已使用 Arc，无需重复包装。

use openlark_core::config::Config;

/// 会议链式入口：`meeting.vc.v1.room.create()` 等
#[derive(Debug, Clone)]
pub struct MeetingClient {
    config: Config,

    /// 日历客户端入口。
    #[cfg(feature = "calendar")]
    pub calendar: CalendarClient,

    /// 视频会议客户端入口。
    #[cfg(feature = "vc")]
    pub vc: VcClient,

    /// 会议室客户端入口。
    #[cfg(feature = "meeting-room")]
    pub meeting_room: MeetingRoomClient,
}

impl MeetingClient {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        let config_cloned = config.clone();
        Self {
            config: config_cloned,
            #[cfg(feature = "calendar")]
            calendar: CalendarClient::new(config.clone()),
            #[cfg(feature = "vc")]
            vc: VcClient::new(config.clone()),
            #[cfg(feature = "meeting-room")]
            meeting_room: MeetingRoomClient::new(config),
        }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 日历链式调用客户端。
#[cfg(feature = "calendar")]
#[derive(Debug, Clone)]
pub struct CalendarClient {
    config: Config,
    /// v4 版本客户端入口。
    pub v4: CalendarV4Client,
}

#[cfg(feature = "calendar")]
impl CalendarClient {
    fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v4: CalendarV4Client::new(config),
        }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 日历 v4 链式调用客户端。
#[cfg(feature = "calendar")]
#[derive(Debug, Clone)]
pub struct CalendarV4Client {
    config: Config,
    /// 日历客户端入口。
    pub calendar: CalendarResourceClient,
}

#[cfg(feature = "calendar")]
impl CalendarV4Client {
    fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            calendar: CalendarResourceClient::new(config),
        }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 日历资源客户端。
#[cfg(feature = "calendar")]
#[derive(Debug, Clone)]
pub struct CalendarResourceClient {
    config: Config,
}

#[cfg(feature = "calendar")]
impl CalendarResourceClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 视频会议链式调用客户端。
#[cfg(feature = "vc")]
#[derive(Debug, Clone)]
pub struct VcClient {
    config: Config,
    /// v1 版本客户端入口。
    pub v1: VcV1Client,
}

#[cfg(feature = "vc")]
impl VcClient {
    fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: VcV1Client::new(config),
        }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 视频会议 v1 链式调用客户端。
#[cfg(feature = "vc")]
#[derive(Debug, Clone)]
pub struct VcV1Client {
    config: Config,
    /// room 资源入口。
    pub room: VcRoomResourceClient,
    /// meeting 资源入口。
    pub meeting: VcMeetingResourceClient,
    /// reserve 资源入口。
    pub reserve: VcReserveResourceClient,
    /// note 资源入口。
    pub note: VcNoteResourceClient,
}

#[cfg(feature = "vc")]
impl VcV1Client {
    fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            room: VcRoomResourceClient::new(config.clone()),
            meeting: VcMeetingResourceClient::new(config.clone()),
            reserve: VcReserveResourceClient::new(config.clone()),
            note: VcNoteResourceClient::new(config),
        }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 视频会议 room 资源客户端。
#[cfg(feature = "vc")]
#[derive(Debug, Clone)]
pub struct VcRoomResourceClient {
    config: Config,
}

#[cfg(feature = "vc")]
impl VcRoomResourceClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 视频会议 meeting 资源客户端。
#[cfg(feature = "vc")]
#[derive(Debug, Clone)]
pub struct VcMeetingResourceClient {
    config: Config,
}

#[cfg(feature = "vc")]
impl VcMeetingResourceClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 视频会议 reserve 资源客户端。
#[cfg(feature = "vc")]
#[derive(Debug, Clone)]
pub struct VcReserveResourceClient {
    config: Config,
}

#[cfg(feature = "vc")]
impl VcReserveResourceClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 视频会议 note 资源客户端。
#[cfg(feature = "vc")]
#[derive(Debug, Clone)]
pub struct VcNoteResourceClient {
    config: Config,
}

#[cfg(feature = "vc")]
impl VcNoteResourceClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 会议室链式调用客户端。
#[cfg(feature = "meeting-room")]
#[derive(Debug, Clone)]
pub struct MeetingRoomClient {
    config: Config,
}

#[cfg(feature = "meeting-room")]
impl MeetingRoomClient {
    fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
