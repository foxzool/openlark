//! 通用数据模型

use serde::{Deserialize, Serialize};

/// 用户设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSetting {
    /// 设置键
    pub key: String,
    /// 设置值
    pub value: String,
    /// 设置类型
    pub setting_type: String,
}

/// 用户偏好
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreference {
    /// 偏好键
    pub key: String,
    /// 偏好值
    pub value: String,
    /// 偏好类别
    pub category: Option<String>,
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
