//! 卡片实体构建器单元测试
//!
//! 测试卡片实体的构建器模式、字段序列化与校验。

use openlark_cardkit::cardkit::cardkit::v1::card::{
    UpdateCardPayload,
    batch_update::{BatchUpdateCardBody, BatchUpdateCardRequest, BatchUpdateCardRequestBuilder},
    create::{CreateCardBody, CreateCardRequest, CreateCardRequestBuilder},
    id_convert::{ConvertCardIdBody, ConvertCardIdRequest, ConvertCardIdRequestBuilder},
    settings::{
        UpdateCardSettingsBody, UpdateCardSettingsRequest, UpdateCardSettingsRequestBuilder,
    },
    update::{UpdateCardBody, UpdateCardRequest, UpdateCardRequestBuilder},
};

/// 辅助函数：创建测试配置
fn create_test_config() -> openlark_core::config::Config {
    openlark_core::config::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
}

/// 创建卡片请求构建器测试
#[cfg(test)]
mod create_card_request_builder_tests {
    use super::*;

    #[test]
    fn test_builder_default_state() {
        let config = create_test_config();
        let _request = CreateCardRequestBuilder::new(config).build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = CreateCardRequest::new(config);
    }
}

/// 创建卡片体验证测试
#[cfg(test)]
mod create_card_body_validation_tests {
    use super::*;

    #[test]
    fn test_valid_card_body() {
        let body = CreateCardBody {
            type_: "card_json".into(),
            data: r#"{"schema":"2.0","body":{"elements":[]}}"#.into(),
        };
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_empty_type_validation() {
        let body = CreateCardBody {
            type_: "   ".into(),
            data: r#"{"schema":"2.0"}"#.into(),
        };
        let result = body.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("type 不能为空"));
    }

    #[test]
    fn test_empty_data_validation() {
        let body = CreateCardBody {
            type_: "card_json".into(),
            data: "".into(),
        };
        let result = body.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("data 不能为空"));
    }

    #[test]
    fn test_template_type() {
        let body = CreateCardBody {
            type_: "template".into(),
            data: r#"{"template_id":"AAqxxxxx"}"#.into(),
        };
        assert!(body.validate().is_ok());
    }
}

/// 更新卡片请求构建器测试
#[cfg(test)]
mod update_card_request_builder_tests {
    use super::*;

    #[test]
    fn test_update_builder_settings() {
        let config = create_test_config();
        let _request = UpdateCardRequestBuilder::new(config)
            .card_id("card_123")
            .card(UpdateCardPayload {
                type_: "card_json".into(),
                data: r#"{"schema":"2.0"}"#.into(),
            })
            .sequence(1)
            .uuid("a0d69e20-1dd1-458b-k525-dfeca4015204")
            .build();
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = UpdateCardRequest::new(config);
    }
}

/// 批量更新卡片请求构建器测试
#[cfg(test)]
mod batch_update_card_request_builder_tests {
    use super::*;

    #[test]
    fn test_batch_update_builder_with_params() {
        let config = create_test_config();
        let _request = BatchUpdateCardRequestBuilder::new(config)
            .card_id("card_123")
            .actions(r#"[{"action":"delete_elements","params":{"element_ids":["text_1"]}}]"#)
            .sequence(2)
            .build();
    }

    #[test]
    fn test_batch_update_body_validation() {
        let valid_body = BatchUpdateCardBody {
            card_id: "card_123".into(),
            actions: r#"[{"action":"delete_elements","params":{"element_ids":["text_1"]}}]"#
                .into(),
            uuid: None,
            sequence: 1,
        };
        assert!(valid_body.validate().is_ok());

        let invalid = BatchUpdateCardBody {
            card_id: "card_123".into(),
            actions: "".into(),
            uuid: None,
            sequence: 1,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = BatchUpdateCardRequest::new(config);
    }
}

/// 更新卡片设置请求构建器测试
#[cfg(test)]
mod update_card_settings_request_builder_tests {
    use super::*;

    #[test]
    fn test_settings_builder_with_params() {
        let config = create_test_config();
        let _request = UpdateCardSettingsRequestBuilder::new(config)
            .card_id("card_123")
            .settings(r#"{"config":{"streaming_mode":true}}"#)
            .sequence(1)
            .build();
    }

    #[test]
    fn test_settings_body_creation() {
        let body = UpdateCardSettingsBody {
            card_id: "card_123".into(),
            settings: r#"{"config":{"streaming_mode":true}}"#.into(),
            uuid: None,
            sequence: 1,
        };
        assert_eq!(body.card_id, "card_123");
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = UpdateCardSettingsRequest::new(config);
    }
}

/// ID 转换请求构建器测试
#[cfg(test)]
mod id_convert_request_builder_tests {
    use super::*;

    #[test]
    fn test_id_convert_builder_with_params() {
        let config = create_test_config();
        let _request = ConvertCardIdRequestBuilder::new(config)
            .message_id("om_fbdf6ed2e17f1d98e78fb26c1370186e")
            .build();
    }

    #[test]
    fn test_convert_body_validation() {
        let body = ConvertCardIdBody {
            message_id: "om_fbdf6ed2e17f1d98e78fb26c1370186e".into(),
        };
        assert!(body.validate().is_ok());
        assert!(
            ConvertCardIdBody {
                message_id: "".into()
            }
            .validate()
            .is_err()
        );
    }

    #[test]
    fn test_request_new() {
        let config = create_test_config();
        let _request = ConvertCardIdRequest::new(config);
    }
}

/// Body 结构体序列化测试
#[cfg(test)]
mod body_serialization_tests {
    use super::*;

    #[test]
    fn test_create_card_body_serialization() {
        let body = CreateCardBody {
            type_: "card_json".into(),
            data: r#"{"schema":"2.0"}"#.into(),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains(r#""type":"card_json""#));
        assert!(json_str.contains(r#""data":"{\"schema\":\"2.0\"}""#));
        assert!(!json_str.contains("card_content"));
    }

    #[test]
    fn test_update_card_body_serialization_skips_path_params() {
        let body = UpdateCardBody {
            card_id: "card_123".into(),
            card: UpdateCardPayload {
                type_: "card_json".into(),
                data: r#"{"schema":"2.0"}"#.into(),
            },
            uuid: Some("uuid-1".into()),
            sequence: 3,
        };
        let value = serde_json::to_value(&body).expect("序列化失败");
        assert!(value.get("card_id").is_none());
        assert_eq!(value["card"]["type"], "card_json");
        assert_eq!(value["sequence"], 3);
        assert_eq!(value["uuid"], "uuid-1");
    }

    #[test]
    fn test_batch_update_card_body_serialization() {
        let body = BatchUpdateCardBody {
            card_id: "card_123".into(),
            actions: r#"[{"action":"delete_elements"}]"#.into(),
            uuid: None,
            sequence: 1,
        };
        let value = serde_json::to_value(&body).expect("序列化失败");
        assert!(value.get("card_id").is_none());
        assert!(value.get("operations").is_none());
        assert!(value["actions"].as_str().unwrap().contains("delete_elements"));
    }

    #[test]
    fn test_settings_body_serialization() {
        let body = UpdateCardSettingsBody {
            card_id: "card_123".into(),
            settings: r#"{"config":{"enable_forward":true}}"#.into(),
            uuid: None,
            sequence: 1,
        };
        let value = serde_json::to_value(&body).expect("序列化失败");
        assert!(value.get("card_id").is_none());
        assert!(value["settings"].is_string());
    }

    #[test]
    fn test_id_convert_body_serialization() {
        let body = ConvertCardIdBody {
            message_id: "om_xxx".into(),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("message_id"));
        assert!(!json_str.contains("card_ids"));
        assert!(!json_str.contains("source_id_type"));
    }
}

/// sequence 校验测试
#[cfg(test)]
mod sequence_validation_tests {
    use super::*;

    #[test]
    fn test_sequence_must_be_positive() {
        let body = UpdateCardBody {
            card_id: "card_123".into(),
            card: UpdateCardPayload {
                type_: "card_json".into(),
                data: r#"{"schema":"2.0"}"#.into(),
            },
            uuid: None,
            sequence: 0,
        };
        assert!(body.validate().is_err());
    }
}
