//! 卡片实体构建器单元测试
//!
//! 测试卡片实体的构建器模式，包括参数验证、链式调用、默认值等。

use openlark_cardkit::cardkit::cardkit::v1::card::{
    batch_update::{BatchUpdateCardBody, BatchUpdateCardRequest, BatchUpdateCardRequestBuilder},
    create::{CreateCardBody, CreateCardRequest},
    id_convert::{ConvertCardIdBody, ConvertCardIdRequest, ConvertCardIdRequestBuilder},
    settings::{
        UpdateCardSettingsBody, UpdateCardSettingsRequest, UpdateCardSettingsRequestBuilder,
    },
    update::{UpdateCardBody, UpdateCardRequest, UpdateCardRequestBuilder},
};
use serde_json::json;

/// 辅助函数：创建测试配置
fn create_test_config() -> openlark_core::config::Config {
    openlark_core::config::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
}

/// 更新卡片请求构建器测试
#[cfg(test)]
mod update_card_request_builder_tests {
    use super::*;

    #[test]
    fn test_update_builder_default_state() {
        let config = create_test_config();
        let builder = UpdateCardRequestBuilder::new(config.clone());
        let _request = builder.build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_update_builder_settings() {
        let config = create_test_config();

        let builder = UpdateCardRequestBuilder::new(config.clone())
            .card_id("card_123")
            .card(json!({"type": "card_json", "data": "{\"updated\":true}"}))
            .sequence(1);

        let _request = builder.build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = UpdateCardRequest::new(config);
        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_update_builder_chaining() {
        let config = create_test_config();

        let _request = UpdateCardRequestBuilder::new(config.clone())
            .card_id("card_123")
            .card(json!({"type": "card_json", "data": "{\"key\":\"value\"}"}))
            .uuid("uuid_1")
            .sequence(2)
            .build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }
}

/// 批量更新卡片请求构建器测试
#[cfg(test)]
mod batch_update_card_request_builder_tests {
    use super::*;

    #[test]
    fn test_batch_update_builder_default_state() {
        let config = create_test_config();
        let builder = BatchUpdateCardRequestBuilder::new(config.clone());
        let _request = builder.build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_batch_update_builder_with_params() {
        let config = create_test_config();

        let operations = vec![
            json!({"operation": "add", "path": "/elements", "value": {}}),
            json!({"operation": "replace", "path": "/header", "value": {}}),
        ];

        let builder = BatchUpdateCardRequestBuilder::new(config.clone())
            .card_id("card_123")
            .operations(operations);

        let _request = builder.build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = BatchUpdateCardRequest::new(config);
        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_batch_update_body_validation() {
        // 有效请求体
        let valid_body = BatchUpdateCardBody {
            card_id: "card_123".to_string(),
            operations: vec![json!({"op": "add"})],
        };
        assert!(!valid_body.card_id.is_empty());
        assert!(!valid_body.operations.is_empty());
    }
}

/// 更新卡片设置请求构建器测试
#[cfg(test)]
mod update_card_settings_request_builder_tests {
    use super::*;

    #[test]
    fn test_settings_builder_default_state() {
        let config = create_test_config();
        let builder = UpdateCardSettingsRequestBuilder::new(config.clone());
        let _request = builder.build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_settings_builder_with_params() {
        let config = create_test_config();

        let builder = UpdateCardSettingsRequestBuilder::new(config.clone())
            .card_id("card_123")
            .settings(r#"{"auto_submit":true,"allow_forward":false}"#);

        let _request = builder.build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = UpdateCardSettingsRequest::new(config);
        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_settings_body_creation() {
        let body = UpdateCardSettingsBody {
            card_id: "card_123".to_string(),
            settings: r#"{"key":"value"}"#.to_string(),
            uuid: None,
            sequence: Some(1),
        };

        assert_eq!(body.card_id, "card_123");
        assert!(!body.settings.is_empty());
    }
}

/// ID 转换请求构建器测试
#[cfg(test)]
mod id_convert_request_builder_tests {
    use super::*;

    #[test]
    fn test_id_convert_builder_default_state() {
        let config = create_test_config();
        let builder = ConvertCardIdRequestBuilder::new(config.clone());
        let _request = builder.build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_id_convert_builder_with_params() {
        let config = create_test_config();

        let builder = ConvertCardIdRequestBuilder::new(config.clone())
            .source_id_type("card_id")
            .target_id_type("open_card_id")
            .card_ids(vec!["card_1".to_string(), "card_2".to_string()]);

        let _request = builder.build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = ConvertCardIdRequest::new(config);
        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_convert_body_validation() {
        let body = ConvertCardIdBody {
            source_id_type: "card_id".to_string(),
            target_id_type: "open_card_id".to_string(),
            card_ids: vec!["card_1".to_string(), "card_2".to_string()],
        };

        assert_eq!(body.source_id_type, "card_id");
        assert_eq!(body.target_id_type, "open_card_id");
        assert_eq!(body.card_ids.len(), 2);
    }
}

/// Body 结构体序列化测试
#[cfg(test)]
mod body_serialization_tests {
    use super::*;

    #[test]
    fn test_create_card_body_serialization() {
        let body = CreateCardBody {
            r#type: "card_json".into(),
            data: r#"{"schema":"2.0"}"#.into(),
        };

        let v = serde_json::to_value(&body).expect("序列化失败");
        assert_eq!(v["type"], "card_json");
        assert_eq!(v["data"], r#"{"schema":"2.0"}"#);
        assert!(v.get("card_content").is_none(), "官方平铺格式不应有 card_content");
    }

    #[test]
    fn test_update_card_body_serialization() {
        let body = UpdateCardBody {
            card_id: "card_123".to_string(),
            card: json!({"type": "card_json", "data": "{\"updated\":true}"}),
            uuid: None,
            sequence: Some(3),
        };

        let v = serde_json::to_value(&body).expect("序列化失败");
        assert_eq!(v["card"]["type"], "card_json");
        assert_eq!(v["sequence"], 3);
        assert!(v.get("card_id").is_none(), "card_id 仅用于 URL path");
        assert!(v.get("card_content").is_none());
    }

    #[test]
    fn test_batch_update_card_body_serialization() {
        let body = BatchUpdateCardBody {
            card_id: "card_123".to_string(),
            operations: vec![json!({"op": "add"})],
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("card_id"));
        assert!(json_str.contains("operations"));
    }

    #[test]
    fn test_settings_body_serialization() {
        let body = UpdateCardSettingsBody {
            card_id: "card_123".to_string(),
            settings: r#"{"auto_submit":true}"#.to_string(),
            uuid: None,
            sequence: Some(1),
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("settings"));
        assert!(json_str.contains(r#"\"auto_submit\":true"#));
        assert!(json_str.contains(r#"\"sequence\":1"#));
        assert!(!json_str.contains("card_id"), "card_id 仅用于 URL path");
    }

    #[test]
    fn test_id_convert_body_serialization() {
        let body = ConvertCardIdBody {
            source_id_type: "card_id".to_string(),
            target_id_type: "open_card_id".to_string(),
            card_ids: vec!["card_1".to_string()],
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("source_id_type"));
        assert!(json_str.contains("target_id_type"));
        assert!(json_str.contains("card_ids"));
    }
}

/// 边界情况测试
#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_card_content_object() {
        // 空的 JSON 字符串（data）也有效（校验仅要求非空）
        let body = CreateCardBody {
            r#type: "card_json".into(),
            data: "{}".into(),
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_nested_card_content() {
        // 嵌套的复杂 JSON 以字符串形式放在 data
        let body = CreateCardBody {
            r#type: "card_json".into(),
            data: r#"{"header":{"title":{"tag":"plain_text","content":"标题"}},"elements":[{"tag":"div"}]}"#.into(),
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_special_characters_in_strings() {
        let body = CreateCardBody {
            r#type: "card_json".into(),
            data: r#"{"text":"特殊字符：!@#$%^&*()_+-=[]{}|;':\",./<>?"}"#.into(),
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_unicode_content() {
        let body = CreateCardBody {
            r#type: "card_json".into(),
            data: r#"{"text":"中文内容 🎉 Emoji 测试"}"#.into(),
        };

        assert!(body.validate().is_ok());
    }
}
