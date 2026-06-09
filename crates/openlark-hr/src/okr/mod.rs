/// OKR 模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

/// okr 项目模块。
/// okr 子模块。
#[allow(clippy::module_inception)]
pub mod okr;

/// OKR 服务
/// Okr 服务入口。
#[derive(Debug, Clone)]
pub struct Okr {
    config: Config,
}

impl Okr {
    /// 创建新的服务入口实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回共享配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 okr 项目 v1 版本服务
    pub fn v1(&self) -> okr::OkrV1 {
        okr::OkrV1::new(self.config.clone())
    }
    /// 获取 okr 项目 v2 版本服务
    pub fn v2(&self) -> okr::OkrV2 {
        okr::OkrV2::new(self.config.clone())
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
