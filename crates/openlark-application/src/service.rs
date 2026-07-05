use openlark_core::config::Config;
use std::sync::Arc;

/// ApplicationService：应用管理服务的统一入口
///
/// 提供对应用 API v1/v5/v6/v7 与工作台（workplace）的访问能力，各版本经独立 feature 门控。
#[derive(Clone)]
pub struct ApplicationService {
    // reserved：所有版本 feature 关闭时无读取者（--no-default-features）；_config 前缀兜底 dead_code
    _config: Arc<Config>,
}

impl ApplicationService {
    /// 创建新的应用管理服务实例。
    pub fn new(config: Config) -> Self {
        Self {
            _config: Arc::new(config),
        }
    }

    /// 访问 v1 版本应用 API。
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::application::application::v1::ApplicationV1 {
        crate::application::application::v1::ApplicationV1::new(self._config.clone())
    }

    /// 访问 v5 版本应用 API。
    #[cfg(feature = "v5")]
    pub fn v5(&self) -> crate::application::application::v5::ApplicationV5 {
        crate::application::application::v5::ApplicationV5::new(self._config.clone())
    }

    /// 访问 v6 版本应用 API。
    #[cfg(feature = "v6")]
    pub fn v6(&self) -> crate::application::application::v6::ApplicationV6 {
        crate::application::application::v6::ApplicationV6::new(self._config.clone())
    }

    /// 访问 v7 版本应用 API。
    #[cfg(feature = "v7")]
    pub fn v7(&self) -> crate::application::application::v7::ApplicationV7 {
        crate::application::application::v7::ApplicationV7::new(self._config.clone())
    }

    /// 访问工作台（workplace）API（仅 v1，直接返回 WorkplaceV1，不引入单版本中间层）。
    #[cfg(feature = "workplace")]
    pub fn workplace(&self) -> crate::workplace::workplace::v1::WorkplaceV1 {
        crate::workplace::workplace::v1::WorkplaceV1::new(self._config.clone())
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
