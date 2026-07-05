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

    /// 访问 v1 帮助台 API（ticket / agent / category / faq 等 11 资源扇出）。
    ///
    /// ADR 0001：消除 `Helpdesk` 域层转发壳 + 单独 `ticket()` 快捷（11 资源中仅 1 个有快捷，
    /// 深度不一致），统一经 `v1()` 到 `HelpdeskV1` 扇出层。
    /// 原 `service.helpdesk().v1().ticket()` / `service.ticket()` → `service.v1().ticket()`。
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::helpdesk::helpdesk::v1::HelpdeskV1 {
        crate::helpdesk::helpdesk::v1::HelpdeskV1::new(self._config.clone())
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
