use openlark_core::config::Config;
use std::sync::Arc;

/// BotService：机器人服务的统一入口
///
/// 提供对机器人 API v4 的访问能力
#[derive(Clone)]
#[allow(dead_code)]
pub struct BotService {
    config: Arc<Config>,
}

impl BotService {
    /// 创建新的机器人服务实例。
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    #[cfg(feature = "v4")]
    /// 访问机器人 API。
    pub fn bot(&self) -> crate::bot::bot::Bot {
        crate::bot::bot::Bot::new(self.config.clone())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #[test]
    fn test_serialization_roundtrip() {
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }
}
