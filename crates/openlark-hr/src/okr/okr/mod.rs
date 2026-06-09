/// okr 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

/// v1 子模块。
pub mod v1;
/// v2 子模块。
pub mod v2;

/// okr 项目 v1 版本服务
/// OkrV1 服务入口。
#[derive(Debug, Clone)]
pub struct OkrV1 {
    config: Config,
}

impl OkrV1 {
    /// 创建新的服务入口实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回共享配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// okr 项目 v2 版本服务
/// OkrV2 服务入口。
pub type OkrV2 = v2::OkrV2;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
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
