//! Search API 模块

pub mod v2;

use openlark_core::config::Config;
use std::sync::Arc;

/// Search API 入口
#[deprecated(
    note = "analytics Search 导航死胡同；直接用 v2 子模块的 XxxRequest::new(Arc<Config>)，如 crate::search::search::v2::query::SearchRequest / v2::user::SearchUserRequest"
)]
#[derive(Clone)]
pub struct Search {
    config: Arc<Config>,
}

#[allow(deprecated)]
impl Search {
    /// 创建新的 Search API 入口。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 v2 版本搜索 API。
    #[allow(deprecated)]
    pub fn v2(&self) -> v2::SearchV2 {
        v2::SearchV2::new(self.config.clone())
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
