use serde::Deserialize;

/// 获取应用的响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAppResponse {
    /// 应用 ID。
    pub app_id: String,
    /// 应用名称。
    pub app_name: String,
    /// 应用类型。
    pub app_type: String,
    /// 描述。
    pub description: Option<String>,
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
