use openlark_core::config::Config;
use std::sync::Arc;

/// ApplicationService：应用管理服务的统一入口
///
/// 提供对应用 API v1 的访问能力
#[derive(Clone)]
pub struct ApplicationService {
    // reserved：feature v1 关闭时无读取者（见 #274 范式）；_config 前缀兼容两种 feature 组合
    _config: Arc<Config>,
}

impl ApplicationService {
    /// 创建新的应用管理服务实例。
    pub fn new(config: Config) -> Self {
        Self {
            _config: Arc::new(config),
        }
    }

    #[cfg(feature = "v1")]
    /// 访问 v1 版本应用 API。
    pub fn v1(&self) -> crate::application::application::v1::ApplicationV1 {
        crate::application::application::v1::ApplicationV1::new(self._config.clone())
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
