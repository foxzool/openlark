//! Application API 模块

/// 应用管理 v1 版本 API。
pub mod v1;
/// 应用管理 v5 版本 API。
pub mod v5;
/// 应用管理 v7 版本 API。
/// 应用管理 v6 版本 API。
pub mod v6;
/// 待补充文档。
pub mod v7;

use openlark_core::config::Config;
use std::sync::Arc;

/// Application API 入口
#[derive(Clone)]
pub struct Application {
    config: Arc<Config>,
}

impl Application {
    /// 创建新的应用 API 入口。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 v1 版本 API。
    pub fn v1(&self) -> v1::ApplicationV1 {
        v1::ApplicationV1::new(self.config.clone())
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
