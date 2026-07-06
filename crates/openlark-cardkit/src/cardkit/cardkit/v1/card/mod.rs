//! card
//!
//! 卡片实体相关 API（cardkit-v1）。

pub mod batch_update;
pub mod create;
pub mod id_convert;
pub mod models;

/// 重新导出模型类型
pub use self::models::*;
pub mod settings;
pub mod update;

pub mod element;

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
