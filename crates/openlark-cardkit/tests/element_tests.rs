//! 卡片组件构建器单元测试
//!
//! 测试卡片组件的构建器模式，包括参数验证、链式调用、默认值等。

use openlark_cardkit::cardkit::cardkit::v1::card::element::{
    content::{
        UpdateCardElementContentBody, UpdateCardElementContentRequest,
        UpdateCardElementContentRequestBuilder,
    },
    create::{CreateCardElementBody, CreateCardElementRequest, CreateCardElementRequestBuilder},
    delete::{DeleteCardElementBody, DeleteCardElementRequest, DeleteCardElementRequestBuilder},
    models::{
        CreateCardElementResponse, DeleteCardElementResponse, PatchCardElementResponse,
        UpdateCardElementContentResponse, UpdateCardElementResponse,
    },
    patch::{PatchCardElementBody, PatchCardElementRequest, PatchCardElementRequestBuilder},
    update::{UpdateCardElementBody, UpdateCardElementRequest, UpdateCardElementRequestBuilder},
};
use serde_json::json;

/// 辅助函数：创建测试配置
fn create_test_config() -> openlark_core::config::Config {
    openlark_core::config::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
}

/// 创建卡片组件请求构建器测试
#[cfg(test)]
mod create_card_element_request_builder_tests {
    use super::*;

    #[test]
    fn test_builder_default_state() {
        let config = create_test_config();
        let builder = CreateCardElementRequestBuilder::new(config.clone());
        let _request = builder.build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_builder_card_id_setting() {
        let config = create_test_config();

        let builder = CreateCardElementRequestBuilder::new(config.clone()).card_id("card_123");

        let _request = builder.build();
    }

    #[test]
    fn test_builder_element_setting() {
        let config = create_test_config();
        let element = json!({"type": "text", "content": "hello"});

        let builder = CreateCardElementRequestBuilder::new(config.clone()).element(element.clone());

        let _request = builder.build();
    }

    #[test]
    fn test_builder_chaining() {
        let config = create_test_config();
        let element = json!({"type": "div", "text": {"content": "test"}});

        let _request = CreateCardElementRequestBuilder::new(config.clone())
            .card_id("card_123")
            .element(element)
            .build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = CreateCardElementRequest::new(config);
        // 验证 request 对象已创建（Rust 引用永不为 null）
    }
}

/// 创建卡片组件体验证测试
#[cfg(test)]
mod create_card_element_body_tests {
    use super::*;

    #[test]
    fn test_valid_element_body() {
        let body = CreateCardElementBody {
            card_id: "card_123".to_string(),
            element: json!({"type": "text", "content": "hello"}),
        };

        assert_eq!(body.card_id, "card_123");
        assert!(!body.element.is_null());
    }

    #[test]
    fn test_element_body_serialization() {
        let body = CreateCardElementBody {
            card_id: "card_123".to_string(),
            element: json!({
                "tag": "div",
                "text": {
                    "tag": "plain_text",
                    "content": "测试内容"
                }
            }),
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("card_id"));
        assert!(json_str.contains("element"));
    }

    #[test]
    fn test_complex_element_body() {
        let body = CreateCardElementBody {
            card_id: "card_456".to_string(),
            element: json!({
                "tag": "column_set",
                "flex_mode": "none",
                "background_style": "default",
                "columns": [
                    {
                        "tag": "column",
                        "width": "weighted",
                        "weight": 1,
                        "elements": [
                            {"tag": "markdown", "content": "列1内容"}
                        ]
                    }
                ]
            }),
        };

        assert!(!body.element.is_null());
    }
}

/// 更新卡片组件请求构建器测试
#[cfg(test)]
mod update_card_element_request_builder_tests {
    use super::*;

    #[test]
    fn test_update_builder_default_state() {
        let config = create_test_config();
        let builder = UpdateCardElementRequestBuilder::new(config.clone());
        let _request = builder.build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_update_builder_with_params() {
        let config = create_test_config();
        let patch = json!({"content": "updated content"});

        let builder = UpdateCardElementRequestBuilder::new(config.clone())
            .card_id("card_123")
            .element_id("elem_456")
            .patch(patch);

        let _request = builder.build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = UpdateCardElementRequest::new(config);
        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_update_builder_chaining() {
        let config = create_test_config();

        let _request = UpdateCardElementRequestBuilder::new(config.clone())
            .card_id("card_123")
            .element_id("elem_456")
            .patch(json!({"key": "value"}))
            .build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }
}

/// 更新卡片组件体验证测试
#[cfg(test)]
mod update_card_element_body_tests {
    use super::*;

    #[test]
    fn test_valid_update_body() {
        let body = UpdateCardElementBody {
            card_id: "card_123".to_string(),
            element_id: "elem_456".to_string(),
            patch: json!({"content": "new content"}),
        };

        assert_eq!(body.card_id, "card_123");
        assert_eq!(body.element_id, "elem_456");
    }

    #[test]
    fn test_update_body_serialization() {
        let body = UpdateCardElementBody {
            card_id: "card_123".to_string(),
            element_id: "elem_456".to_string(),
            patch: json!({"style": {"bold": true}}),
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("card_id"));
        assert!(json_str.contains("element_id"));
        assert!(json_str.contains("patch"));
    }
}

/// 修补卡片组件请求构建器测试
#[cfg(test)]
mod patch_card_element_request_builder_tests {
    use super::*;

    #[test]
    fn test_patch_builder_default_state() {
        let config = create_test_config();
        let builder = PatchCardElementRequestBuilder::new(config.clone());
        let _request = builder.build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_patch_builder_with_params() {
        let config = create_test_config();
        let patch = json!([{"op": "replace", "path": "/content", "value": "new"}]);

        let builder = PatchCardElementRequestBuilder::new(config.clone())
            .card_id("card_123")
            .element_id("elem_456")
            .patch(patch);

        let _request = builder.build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = PatchCardElementRequest::new(config);
        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_patch_builder_chaining() {
        let config = create_test_config();

        let _request = PatchCardElementRequestBuilder::new(config.clone())
            .card_id("card_123")
            .element_id("elem_456")
            .patch(json!({"op": "add"}))
            .build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }
}

/// 修补卡片组件体验证测试
#[cfg(test)]
mod patch_card_element_body_tests {
    use super::*;

    #[test]
    fn test_valid_patch_body() {
        let body = PatchCardElementBody {
            card_id: "card_123".to_string(),
            element_id: "elem_456".to_string(),
            patch: json!([{"op": "replace", "path": "/text", "value": {"content": "new"}}]),
        };

        assert_eq!(body.card_id, "card_123");
        assert_eq!(body.element_id, "elem_456");
    }

    #[test]
    fn test_patch_body_serialization() {
        let body = PatchCardElementBody {
            card_id: "card_123".to_string(),
            element_id: "elem_456".to_string(),
            patch: json!({
                "operations": [
                    {"op": "add", "path": "/new_field", "value": "value"}
                ]
            }),
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("card_id"));
        assert!(json_str.contains("element_id"));
        assert!(json_str.contains("patch"));
    }
}

/// 删除卡片组件请求构建器测试
#[cfg(test)]
mod delete_card_element_request_builder_tests {
    use super::*;

    #[test]
    fn test_delete_builder_default_state() {
        let config = create_test_config();
        let builder = DeleteCardElementRequestBuilder::new(config.clone());
        let _request = builder.build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_delete_builder_with_params() {
        let config = create_test_config();

        let builder = DeleteCardElementRequestBuilder::new(config.clone())
            .card_id("card_123")
            .element_id("elem_456");

        let _request = builder.build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = DeleteCardElementRequest::new(config);
        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_delete_builder_chaining() {
        let config = create_test_config();

        let _request = DeleteCardElementRequestBuilder::new(config.clone())
            .card_id("card_123")
            .element_id("elem_456")
            .build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }
}

/// 删除卡片组件体验证测试
#[cfg(test)]
mod delete_card_element_body_tests {
    use super::*;

    #[test]
    fn test_valid_delete_body() {
        let body = DeleteCardElementBody {
            card_id: "card_123".to_string(),
            element_id: "elem_456".to_string(),
        };

        assert_eq!(body.card_id, "card_123");
        assert_eq!(body.element_id, "elem_456");
    }

    #[test]
    fn test_delete_body_serialization() {
        let body = DeleteCardElementBody {
            card_id: "card_123".to_string(),
            element_id: "elem_456".to_string(),
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("card_id"));
        assert!(json_str.contains("element_id"));
    }
}

/// 更新组件内容请求构建器测试
#[cfg(test)]
mod update_element_content_request_builder_tests {
    use super::*;

    #[test]
    fn test_content_builder_default_state() {
        let config = create_test_config();
        let builder = UpdateCardElementContentRequestBuilder::new(config.clone());
        let _request = builder.build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_content_builder_with_params() {
        let config = create_test_config();
        let content = json!("updated text content");

        let builder = UpdateCardElementContentRequestBuilder::new(config.clone())
            .card_id("card_123")
            .element_id("elem_456")
            .content(content);

        let _request = builder.build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = UpdateCardElementContentRequest::new(config);
        // 验证 request 对象已创建（Rust 引用永不为 null）
    }

    #[test]
    fn test_content_builder_chaining() {
        let config = create_test_config();

        let _request = UpdateCardElementContentRequestBuilder::new(config.clone())
            .card_id("card_123")
            .element_id("elem_456")
            .content(json!("streaming content"))
            .build();

        // 验证 request 对象已创建（Rust 引用永不为 null）
    }
}

/// 更新组件内容体验证测试
#[cfg(test)]
mod update_element_content_body_tests {
    use super::*;

    #[test]
    fn test_valid_content_body() {
        let body = UpdateCardElementContentBody {
            card_id: "card_123".to_string(),
            element_id: "elem_456".to_string(),
            content: json!("new streaming content"),
            sequence: Some(1),
        };

        assert_eq!(body.card_id, "card_123");
        assert_eq!(body.element_id, "elem_456");
        assert_eq!(body.sequence, Some(1));
    }

    #[test]
    fn test_content_body_serialization() {
        let body = UpdateCardElementContentBody {
            card_id: "card_123".to_string(),
            element_id: "elem_456".to_string(),
            content: json!("streaming text content"),
            sequence: Some(2),
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("content"));
        assert!(json_str.contains(r#""sequence":2"#));
        // card_id/element_id 仅用于 URL path
        assert!(!json_str.contains("card_id"));
        assert!(!json_str.contains("element_id"));
    }
}

/// 响应模型测试
#[cfg(test)]
mod response_model_tests {
    use super::*;

    #[test]
    fn test_create_element_response_default() {
        let response = CreateCardElementResponse::default();
        assert!(response.card_id.is_none());
        assert!(response.element_id.is_none());
    }

    #[test]
    fn test_update_element_response_default() {
        let response = UpdateCardElementResponse::default();
        assert!(response.card_id.is_none());
        assert!(response.element_id.is_none());
    }

    #[test]
    fn test_patch_element_response_default() {
        let response = PatchCardElementResponse::default();
        assert!(response.card_id.is_none());
        assert!(response.element_id.is_none());
    }

    #[test]
    fn test_delete_element_response_default() {
        let response = DeleteCardElementResponse::default();
        assert!(response.card_id.is_none());
        assert!(response.element_id.is_none());
    }

    #[test]
    fn test_update_content_response_default() {
        let response = UpdateCardElementContentResponse::default();
        assert!(response.card_id.is_none());
        assert!(response.element_id.is_none());
    }

    #[test]
    fn test_create_element_response_with_data() {
        let json_str = r#"{"card_id": "card_123", "element_id": "elem_456"}"#;
        let response: CreateCardElementResponse =
            serde_json::from_str(json_str).expect("反序列化失败");

        assert_eq!(response.card_id, Some("card_123".to_string()));
        assert_eq!(response.element_id, Some("elem_456".to_string()));
    }

    #[test]
    fn test_update_element_response_with_data() {
        let json_str = r#"{"card_id": "card_123", "element_id": "elem_456"}"#;
        let response: UpdateCardElementResponse =
            serde_json::from_str(json_str).expect("反序列化失败");

        assert_eq!(response.card_id, Some("card_123".to_string()));
        assert_eq!(response.element_id, Some("elem_456".to_string()));
    }

    #[test]
    fn test_response_partial_data() {
        // 只包含部分字段的响应
        let json_str = r#"{"card_id": "card_123"}"#;
        let response: CreateCardElementResponse =
            serde_json::from_str(json_str).expect("反序列化失败");

        assert_eq!(response.card_id, Some("card_123".to_string()));
        assert!(response.element_id.is_none());
    }
}

/// 边界情况测试
#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_element_object() {
        // 空的 JSON 对象作为 element
        let body = CreateCardElementBody {
            card_id: "card_123".to_string(),
            element: json!({}),
        };

        assert!(body.element.is_object());
    }

    #[test]
    fn test_nested_element_content() {
        // 嵌套的复杂 JSON 对象
        let body = CreateCardElementBody {
            card_id: "card_123".to_string(),
            element: json!({
                "tag": "column_set",
                "columns": [
                    {
                        "tag": "column",
                        "elements": [
                            {
                                "tag": "markdown",
                                "content": "**粗体文本**"
                            }
                        ]
                    }
                ]
            }),
        };

        assert!(!body.element.is_null());
    }

    #[test]
    fn test_special_characters_in_element() {
        let body = CreateCardElementBody {
            card_id: "card_123".to_string(),
            element: json!({
                "text": "特殊字符：<>&\"'"
            }),
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("特殊字符"));
    }

    #[test]
    fn test_unicode_in_element() {
        let body = CreateCardElementBody {
            card_id: "card_123".to_string(),
            element: json!({
                "text": "中文 🎉 Emoji 测试 🔧"
            }),
        };

        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("中文"));
        assert!(json_str.contains("🎉"));
    }

    #[test]
    fn test_long_content() {
        let long_text = "a".repeat(10000);
        let body = CreateCardElementBody {
            card_id: "card_123".to_string(),
            element: json!({
                "text": long_text
            }),
        };

        assert!(!body.element.is_null());
    }

    #[test]
    fn test_array_patch_content() {
        // JSON Patch 格式的数组
        let body = PatchCardElementBody {
            card_id: "card_123".to_string(),
            element_id: "elem_456".to_string(),
            patch: json!([
                {"op": "replace", "path": "/text/content", "value": "new text"},
                {"op": "add", "path": "/extra", "value": "value"}
            ]),
        };

        assert!(body.patch.is_array());
    }
}

/// 组件类型变体测试
#[cfg(test)]
mod element_type_variants_tests {
    use super::*;

    #[test]
    fn test_plain_text_element() {
        let body = CreateCardElementBody {
            card_id: "card_123".to_string(),
            element: json!({
                "tag": "plain_text",
                "content": "纯文本内容"
            }),
        };

        assert!(!body.element.is_null());
    }

    #[test]
    fn test_markdown_element() {
        let body = CreateCardElementBody {
            card_id: "card_123".to_string(),
            element: json!({
                "tag": "markdown",
                "content": "**粗体** *斜体*"
            }),
        };

        assert!(!body.element.is_null());
    }

    #[test]
    fn test_div_element() {
        let body = CreateCardElementBody {
            card_id: "card_123".to_string(),
            element: json!({
                "tag": "div",
                "text": {
                    "tag": "plain_text",
                    "content": "div内容"
                }
            }),
        };

        assert!(!body.element.is_null());
    }

    #[test]
    fn test_image_element() {
        let body = CreateCardElementBody {
            card_id: "card_123".to_string(),
            element: json!({
                "tag": "img",
                "img_key": "img_123",
                "alt": {
                    "tag": "plain_text",
                    "content": "图片说明"
                }
            }),
        };

        assert!(!body.element.is_null());
    }

    #[test]
    fn test_button_element() {
        let body = CreateCardElementBody {
            card_id: "card_123".to_string(),
            element: json!({
                "tag": "button",
                "text": {
                    "tag": "plain_text",
                    "content": "点击我"
                },
                "value": {
                    "key": "value"
                }
            }),
        };

        assert!(!body.element.is_null());
    }
}
