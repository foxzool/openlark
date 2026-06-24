use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
/// 待补充文档。
pub struct GetAppResponse {
    /// 待补充文档。
    pub app_id: String,
    /// 待补充文档。
    pub app_name: String,
    /// 待补充文档。
    pub app_type: String,
    /// 待补充文档。
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
