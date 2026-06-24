/// 应用资源接口。
pub mod app;

// app 模块显式导出
pub use app::{GetAppRequest, GetAppResponse};

use openlark_core::config::Config;
use std::sync::Arc;

/// ApplicationV1：应用 API v1 访问入口
#[derive(Clone)]
pub struct ApplicationV1 {
    config: Arc<Config>,
}

impl ApplicationV1 {
    /// 创建新的 ApplicationV1 实例。
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
pub mod usage;
pub mod visibility;
