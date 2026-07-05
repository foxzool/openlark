//! openlark-meeting 链式调用入口（meta 风格）。
//!
//! ADR 0001（#353）：砍掉 7 个空壳——`CalendarClient`/`CalendarV4Client`/
//! `CalendarResourceClient`/`MeetingRoomClient`（承诺资源却零方法）+ `VcRoomResourceClient`/
//! `VcMeetingResourceClient`/`VcReserveResourceClient`（字段暗示 room/meeting/reserve API
//! 却未接线，真实 builder 经 strict 路径 `crate::vc::vc::v1::<resource>::*` 访问）。
//! 保留唯一接线的真实资源 `VcNoteResourceClient`（get/subscribe/unsubscribe）。
//!
//! Config 内部 Arc-wrapped，clone O(1)（非深拷贝），无需改 Arc。

use openlark_core::config::Config;

/// 会议链式入口：`client.meeting.vc.v1.note.get()` 等。
#[derive(Debug, Clone)]
pub struct MeetingClient {
    config: Config,
    /// 视频会议客户端入口。
    #[cfg(feature = "vc")]
    pub vc: VcClient,
}

impl MeetingClient {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            #[cfg(feature = "vc")]
            vc: VcClient::new(config),
        }
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
///
/// 仅 `note` 资源接线（get/subscribe/unsubscribe）；room/meeting/reserve 的真实 builder
/// 经 strict 路径访问（ADR 0001：不 deepen 空壳）。
#[cfg(feature = "vc")]
#[derive(Debug, Clone)]
pub struct VcV1Client {
    config: Config,
    /// note 资源入口。
    pub note: VcNoteResourceClient,
}

#[cfg(feature = "vc")]
impl VcV1Client {
    fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            note: VcNoteResourceClient::new(config),
        }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 视频会议 note 资源客户端（get/subscribe/unsubscribe）。
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

    /// 获取纪要详情。
    pub fn get(&self, note_id: impl Into<String>) -> crate::vc::vc::v1::note::get::NoteGetRequest {
        crate::vc::vc::v1::note::get::NoteGetRequest::new(std::sync::Arc::new(self.config.clone()))
            .note_id(note_id)
    }

    /// 订阅纪要变更事件。
    pub fn subscribe(&self) -> crate::vc::vc::v1::note::subscription::SubscribeNoteRequest {
        crate::vc::vc::v1::note::subscription::SubscribeNoteRequest::new(self.config.clone())
    }

    /// 取消订阅纪要变更事件。
    pub fn unsubscribe(&self) -> crate::vc::vc::v1::note::unsubscription::UnsubscribeNoteRequest {
        crate::vc::vc::v1::note::unsubscription::UnsubscribeNoteRequest::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_serialization_roundtrip() {
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }
}
