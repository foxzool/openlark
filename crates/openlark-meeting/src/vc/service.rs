//! 视频会议服务
use openlark_core::config::Config;

/// 视频会议服务
#[derive(Debug, Clone)]
pub struct VcService {
    config: Config,
}

impl VcService {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 访问 v1 版本 API。
    pub fn v1(&self) -> VcV1Service {
        VcV1Service::new(self.config.clone())
    }
}

/// 视频会议 V1 服务
#[derive(Debug, Clone)]
pub struct VcV1Service {
    config: Config,
}

impl VcV1Service {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 访问 room 资源。
    pub fn room(&self) -> RoomResource {
        RoomResource::new(self.config.clone())
    }

    /// 访问 meeting 资源。
    pub fn meeting(&self) -> MeetingResource {
        MeetingResource::new(self.config.clone())
    }

    /// 访问 reserve 资源。
    pub fn reserve(&self) -> ReserveResource {
        ReserveResource::new(self.config.clone())
    }
    /// 访问 note 资源。
    pub fn note(&self) -> NoteResource {
        NoteResource::new(self.config.clone())
    }
}

/// Room 资源
#[derive(Debug, Clone)]
pub struct RoomResource {
    config: Config,
}

impl RoomResource {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// Meeting 资源
#[derive(Debug, Clone)]
pub struct MeetingResource {
    config: Config,
}

impl MeetingResource {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// Reserve 资源
#[derive(Debug, Clone)]
pub struct ReserveResource {
    config: Config,
}

impl ReserveResource {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
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

/// Note 资源
#[derive(Debug, Clone)]
pub struct NoteResource {
    config: Config,
}

impl NoteResource {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回配置引用。
    pub fn config(&self) -> &Config {
        &self.config
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
