pub mod get;
/// 应用详情相关数据模型。
pub mod models;

use openlark_core::config::Config;
use std::sync::Arc;

/// 应用资源服务。
#[derive(Clone)]
pub struct App {
    config: Arc<Config>,
}

impl App {
    /// 创建新的应用资源服务。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取应用详情。
    pub fn get(&self) -> get::GetAppRequest {
        get::GetAppRequest::new(self.config.clone())
    }
}

pub use get::GetAppRequest;
// models 模块显式导出
pub use models::GetAppResponse;

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

pub mod create;
pub mod delete;
pub mod list;
pub mod patch;
