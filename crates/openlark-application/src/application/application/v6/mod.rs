/// 待补充文档。
pub mod app;

// app 模块显式导出

// app 模块类型经 app::* 访问

use openlark_core::config::Config;
use std::sync::Arc;

/// ApplicationV6：应用 API v6 访问入口
#[derive(Clone)]
pub struct ApplicationV6 {
    config: Arc<Config>,
}

impl ApplicationV6 {
    /// 待补充文档。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问应用资源
    pub fn app(&self) -> app::App {
        app::App::new(self.config.clone())
    }
}

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

pub mod app_badge;
pub mod app_recommend_rule;
pub mod app_usage;
pub mod app_version;
pub mod app_visibility;
pub mod application;
pub mod collaborator;
pub mod contacts_range;
pub mod feedback;
pub mod frequently_used;
pub mod management;
pub mod owner;
pub mod scope;
pub mod usage;
pub mod visibility;
