//! 卡片组件构建器单元测试
//!
//! 测试卡片组件的构建器模式、字段序列化与校验。

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

/// 辅助函数：创建测试配置
fn create_test_config() -> openlark_core::config::Config {
    openlark_core::config::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
}

#[cfg(test)]
mod create_card_element_tests {
    use super::*;

    #[test]
    fn test_builder_chaining() {
        let config = create_test_config();
        let _request = CreateCardElementRequestBuilder::new(config)
            .card_id("card_123")
            .type_("append")
            .elements(r#"[{"tag":"markdown","id":"md_1","content":"hi"}]"#)
            .sequence(1)
            .build();
    }

    #[test]
    fn test_valid_element_body() {
        let body = CreateCardElementBody {
            card_id: "card_123".into(),
            type_: "insert_before".into(),
            target_element_id: Some("elem_1".into()),
            uuid: None,
            sequence: 1,
            elements: r#"[{"tag":"markdown","id":"md_1","content":"hello"}]"#.into(),
        };
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_element_body_serialization_skips_path_params() {
        let body = CreateCardElementBody {
            card_id: "card_123".into(),
            type_: "append".into(),
            target_element_id: None,
            uuid: None,
            sequence: 1,
            elements: r#"[{"tag":"div"}]"#.into(),
        };
        let value = serde_json::to_value(&body).expect("序列化失败");
        assert!(value.get("card_id").is_none());
        assert!(value.get("element").is_none());
        assert_eq!(value["type"], "append");
        assert_eq!(value["sequence"], 1);
    }

    #[test]
    fn test_request_new() {
        let _request = CreateCardElementRequest::new(create_test_config());
    }
}

#[cfg(test)]
mod update_card_element_tests {
    use super::*;

    #[test]
    fn test_update_builder_with_params() {
        let _request = UpdateCardElementRequestBuilder::new(create_test_config())
            .card_id("card_123")
            .element_id("elem_456")
            .element(r#"{"tag":"markdown","id":"md_1","content":"普通文本"}"#)
            .sequence(1)
            .build();
    }

    #[test]
    fn test_valid_update_body() {
        let body = UpdateCardElementBody {
            card_id: "card_123".into(),
            element_id: "elem_456".into(),
            element: r#"{"tag":"markdown","content":"new"}"#.into(),
            sequence: 1,
            uuid: None,
        };
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_update_body_serialization() {
        let body = UpdateCardElementBody {
            card_id: "card_123".into(),
            element_id: "elem_456".into(),
            element: r#"{"tag":"markdown"}"#.into(),
            sequence: 2,
            uuid: None,
        };
        let value = serde_json::to_value(&body).expect("序列化失败");
        assert!(value.get("card_id").is_none());
        assert!(value.get("element_id").is_none());
        assert!(value.get("patch").is_none());
        assert!(value["element"].is_string());
        assert_eq!(value["sequence"], 2);
    }

    #[test]
    fn test_request_new() {
        let _request = UpdateCardElementRequest::new(create_test_config());
    }
}

#[cfg(test)]
mod patch_card_element_tests {
    use super::*;

    #[test]
    fn test_patch_builder_with_params() {
        let _request = PatchCardElementRequestBuilder::new(create_test_config())
            .card_id("card_123")
            .element_id("elem_456")
            .partial_element(r#"{"content":"Updated text"}"#)
            .sequence(1)
            .build();
    }

    #[test]
    fn test_patch_body_serialization() {
        let body = PatchCardElementBody {
            card_id: "card_123".into(),
            element_id: "elem_456".into(),
            partial_element: r#"{"content":"Updated text"}"#.into(),
            sequence: 1,
            uuid: None,
        };
        let value = serde_json::to_value(&body).expect("序列化失败");
        assert!(value.get("patch").is_none());
        assert_eq!(value["partial_element"], r#"{"content":"Updated text"}"#);
        assert_eq!(value["sequence"], 1);
    }

    #[test]
    fn test_request_new() {
        let _request = PatchCardElementRequest::new(create_test_config());
    }
}

#[cfg(test)]
mod delete_card_element_tests {
    use super::*;

    #[test]
    fn test_delete_builder_with_params() {
        let _request = DeleteCardElementRequestBuilder::new(create_test_config())
            .card_id("card_123")
            .element_id("elem_456")
            .sequence(1)
            .uuid("uuid-1")
            .build();
    }

    #[test]
    fn test_delete_body_requires_sequence() {
        let body = DeleteCardElementBody {
            card_id: "card_123".into(),
            element_id: "elem_456".into(),
            sequence: 1,
            uuid: None,
        };
        assert!(body.validate().is_ok());
        assert!(
            DeleteCardElementBody {
                card_id: "card_123".into(),
                element_id: "elem_456".into(),
                sequence: 0,
                uuid: None,
            }
            .validate()
            .is_err()
        );
    }

    #[test]
    fn test_request_new() {
        let _request = DeleteCardElementRequest::new(create_test_config());
    }
}

#[cfg(test)]
mod content_card_element_tests {
    use super::*;

    #[test]
    fn test_content_builder_with_params() {
        let _request = UpdateCardElementContentRequestBuilder::new(create_test_config())
            .card_id("card_123")
            .element_id("elem_456")
            .content("updated text")
            .sequence(1)
            .build();
    }

    #[test]
    fn test_content_body_serialization() {
        let body = UpdateCardElementContentBody {
            card_id: "card_123".into(),
            element_id: "elem_456".into(),
            content: "updated text".into(),
            sequence: 1,
            uuid: None,
        };
        let value = serde_json::to_value(&body).expect("序列化失败");
        assert!(value.get("card_id").is_none());
        assert!(value.get("element_id").is_none());
        assert_eq!(value["content"], "updated text");
        assert_eq!(value["sequence"], 1);
    }

    #[test]
    fn test_content_must_be_string_not_object() {
        // 官方文档 content 为 string，不是 JSON 对象
        let body = UpdateCardElementContentBody {
            card_id: "card_123".into(),
            element_id: "elem_456".into(),
            content: "plain text".into(),
            sequence: 1,
            uuid: None,
        };
        assert!(body.validate().is_ok());
        let value = serde_json::to_value(&body).unwrap();
        assert!(value["content"].is_string());
    }

    #[test]
    fn test_request_new() {
        let _request = UpdateCardElementContentRequest::new(create_test_config());
    }
}

#[cfg(test)]
mod response_models_tests {
    use super::*;

    #[test]
    fn test_empty_response_defaults() {
        let _ = CreateCardElementResponse::default();
        let _ = UpdateCardElementResponse::default();
        let _ = PatchCardElementResponse::default();
        let _ = UpdateCardElementContentResponse::default();
        let _ = DeleteCardElementResponse::default();
    }
}
