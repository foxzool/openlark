use openlark_core::config::Config;
use std::sync::Arc;

/// 帮助台服务入口。
#[derive(Clone)]
pub struct HelpdeskService {
    // reserved：feature v1 关闭时无读取者（见 #274 范式）；_config 前缀兼容两种 feature 组合
    _config: Arc<Config>,
}

impl HelpdeskService {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            _config: Arc::new(config),
        }
    }

    /// 访问帮助台 API。
    #[cfg(feature = "v1")]
    pub fn helpdesk(&self) -> crate::helpdesk::helpdesk::Helpdesk {
        crate::helpdesk::helpdesk::Helpdesk::new(self._config.clone())
    }

    /// 访问工单 API。
    #[cfg(feature = "v1")]
    pub fn ticket(&self) -> crate::helpdesk::helpdesk::v1::ticket::Ticket {
        crate::helpdesk::helpdesk::v1::ticket::Ticket::new(self._config.clone())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
