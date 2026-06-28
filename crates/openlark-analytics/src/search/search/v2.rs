//! 搜索服务 V2 API
//!
//! 提供搜索服务 V2 版本的 API 访问

use crate::AnalyticsConfig;
use std::sync::Arc;

/// 搜索服务 V2 API
#[derive(Debug, Clone)]
pub struct SearchV2 {
    // reserved：query()/user() deprecated 存根移除后无读取者（analytics search 导航缺口，后续补访问器）
    _config: Arc<AnalyticsConfig>,
}

impl SearchV2 {
    /// 创建新的搜索服务 V2 实例
    pub fn new(config: Arc<AnalyticsConfig>) -> Self {
        Self { _config: config }
    }
}

pub mod app;
pub mod data_source;
pub mod doc_wiki;
pub mod message;
pub mod query;
pub mod schema;
pub mod user;

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
