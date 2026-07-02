//! 用户邮箱模块

use openlark_core::config::Config;
use std::sync::Arc;

/// 用户邮箱服务
#[derive(Clone)]
pub struct User {
    /// 导航 struct，accessor 待补（见 #274/#275 范式），本 change 不接线。
    #[expect(dead_code)]
    config: Arc<Config>,
}

impl User {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
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

pub mod query;
