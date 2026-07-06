//! card.element
//!
//! 卡片组件相关 API（cardkit-v1）。

pub mod content;
pub mod create;
pub mod delete;
pub mod models;
pub mod patch;
pub mod update;

// content 模块显式导出

pub use content::{
    UpdateCardElementContentBody, UpdateCardElementContentRequest,
    UpdateCardElementContentRequestBuilder,
};
// create 模块显式导出
pub use create::{
    CreateCardElementBody, CreateCardElementRequest, CreateCardElementRequestBuilder,
};
// delete 模块显式导出
pub use delete::{
    DeleteCardElementBody, DeleteCardElementRequest, DeleteCardElementRequestBuilder,
};
// models 模块显式导出
pub use models::{
    CreateCardElementResponse, DeleteCardElementResponse, PatchCardElementResponse,
    UpdateCardElementContentResponse, UpdateCardElementResponse,
};
// patch 模块显式导出
pub use patch::{PatchCardElementBody, PatchCardElementRequest, PatchCardElementRequestBuilder};
// update 模块显式导出
pub use update::{
    UpdateCardElementBody, UpdateCardElementRequest, UpdateCardElementRequestBuilder,
};

#[cfg(test)]
mod tests {

    use serde_json;

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
