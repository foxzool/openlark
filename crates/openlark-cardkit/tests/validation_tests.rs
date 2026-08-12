//! CardKit 验证函数单元测试
//!
//! 测试卡片和组件相关的验证函数行为。

use openlark_cardkit::common::validation::{
    validate_card_id, validate_element_id, validate_id_list, validate_id_type, validate_sequence,
    validate_uuid,
};

/// validate_card_id 函数测试
#[cfg(test)]
mod validate_card_id_tests {
    use super::*;

    #[test]
    fn test_validate_card_id_valid() {
        // 有效的 card_id
        assert!(validate_card_id("card_123").is_ok());
        assert!(validate_card_id("valid_card_id").is_ok());
        assert!(validate_card_id("oc_xxx").is_ok());
    }

    #[test]
    fn test_validate_card_id_empty() {
        // 空 card_id
        let result = validate_card_id("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_card_id_whitespace() {
        // 纯空白字符的 card_id
        let result = validate_card_id("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_card_id_with_surrounding_whitespace() {
        // 带前后空格的有效 card_id（会被 trim）
        assert!(validate_card_id("  card_123  ").is_ok());
    }

    #[test]
    fn test_validate_card_id_special_characters() {
        // 包含特殊字符的 card_id
        assert!(validate_card_id("card-123_test").is_ok());
        assert!(validate_card_id("card:123").is_ok());
    }
}

/// validate_element_id 函数测试
#[cfg(test)]
mod validate_element_id_tests {
    use super::*;

    #[test]
    fn test_validate_element_id_valid() {
        // 有效的 element_id
        assert!(validate_element_id("elem_123").is_ok());
        assert!(validate_element_id("valid_element_id").is_ok());
    }

    #[test]
    fn test_validate_element_id_empty() {
        // 空 element_id
        let result = validate_element_id("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_element_id_whitespace() {
        // 纯空白字符的 element_id
        let result = validate_element_id("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_element_id_with_surrounding_whitespace() {
        // 带前后空格的有效 element_id（会被 trim）
        assert!(validate_element_id("  elem_123  ").is_ok());
    }

    #[test]
    fn test_validate_element_id_special_characters() {
        // 包含特殊字符的 element_id
        assert!(validate_element_id("elem-123_test").is_ok());
        assert!(validate_element_id("elem:123").is_ok());
    }
}

/// validate_id_type 函数测试
#[cfg(test)]
mod validate_id_type_tests {
    use super::*;

    #[test]
    fn test_validate_id_type_valid() {
        // 有效的 id_type
        assert!(validate_id_type("card_id", "卡片ID").is_ok());
        assert!(validate_id_type("open_card_id", "开放卡片ID").is_ok());
    }

    #[test]
    fn test_validate_id_type_empty() {
        // 空 id_type
        let result = validate_id_type("", "测试字段");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_id_type_whitespace() {
        // 纯空白字符的 id_type
        let result = validate_id_type("   ", "测试字段");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_id_type_field_name_in_error() {
        // 错误消息中应包含字段名称
        let result = validate_id_type("", "自定义字段名");
        assert!(result.is_err());
        let err = result.unwrap_err();
        // 验证错误消息包含字段名
        assert!(err.to_string().contains("自定义字段名"));
    }

    #[test]
    fn test_validate_id_type_with_surrounding_whitespace() {
        // 带前后空格的有效 id_type（会被 trim）
        assert!(validate_id_type("  card_id  ", "卡片ID").is_ok());
    }
}

/// validate_id_list 函数测试
#[cfg(test)]
mod validate_id_list_tests {
    use super::*;

    #[test]
    fn test_validate_id_list_valid() {
        // 有效的 ID 列表
        assert!(validate_id_list(&["id1".to_string()], "ID列表").is_ok());
        assert!(
            validate_id_list(
                &["id1".to_string(), "id2".to_string(), "id3".to_string()],
                "ID列表"
            )
            .is_ok()
        );
    }

    #[test]
    fn test_validate_id_list_empty() {
        // 空 ID 列表
        let result = validate_id_list(&[], "ID列表");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_id_list_field_name_in_error() {
        // 错误消息中应包含字段名称
        let result = validate_id_list(&[], "自定义ID列表");
        assert!(result.is_err());
        let err = result.unwrap_err();
        // 验证错误消息包含字段名
        assert!(err.to_string().contains("自定义ID列表"));
    }

    #[test]
    fn test_validate_id_list_single_element() {
        // 单个元素的列表
        assert!(validate_id_list(&["single_id".to_string()], "ID列表").is_ok());
    }

    #[test]
    fn test_validate_id_list_many_elements() {
        // 多个元素的列表
        let ids: Vec<String> = (0..100).map(|i| format!("id_{i}")).collect();
        assert!(validate_id_list(&ids, "ID列表").is_ok());
    }
}

/// validate_sequence / validate_uuid 测试
#[cfg(test)]
mod validate_sequence_uuid_tests {
    use super::*;

    #[test]
    fn test_validate_sequence_valid() {
        assert!(validate_sequence(1).is_ok());
        assert!(validate_sequence(1712578784).is_ok());
        assert!(validate_sequence(i32::MAX).is_ok());
    }

    #[test]
    fn test_validate_sequence_invalid() {
        assert!(validate_sequence(0).is_err());
        assert!(validate_sequence(-1).is_err());
    }

    #[test]
    fn test_validate_uuid_optional() {
        assert!(validate_uuid(&None).is_ok());
        assert!(validate_uuid(&Some("a0d69e20-1dd1-458b-k525-dfeca4015204".into())).is_ok());
        assert!(validate_uuid(&Some("".into())).is_err());
        assert!(validate_uuid(&Some("   ".into())).is_err());
        assert!(validate_uuid(&Some("x".repeat(65))).is_err());
    }
}

/// 验证函数组合测试
#[cfg(test)]
mod combined_validation_tests {
    use super::*;

    #[test]
    fn test_card_and_element_id_combination() {
        // 组合验证 card_id 和 element_id
        assert!(validate_card_id("card_123").is_ok());
        assert!(validate_element_id("elem_456").is_ok());
    }

    #[test]
    fn test_all_validations_pass() {
        // 所有验证都通过
        assert!(validate_card_id("card_123").is_ok());
        assert!(validate_element_id("elem_456").is_ok());
        assert!(validate_id_type("card_id", "源ID类型").is_ok());
        assert!(validate_id_list(&["id1".to_string(), "id2".to_string()], "卡片ID列表").is_ok());
    }

    #[test]
    fn test_all_validations_fail() {
        // 所有验证都失败
        assert!(validate_card_id("").is_err());
        assert!(validate_element_id("").is_err());
        assert!(validate_id_type("", "测试").is_err());
        assert!(validate_id_list(&[], "测试列表").is_err());
    }

    #[test]
    fn test_whitespace_edge_cases() {
        // 各种空白字符边界情况
        assert!(validate_card_id("\t").is_err());
        assert!(validate_card_id("\n").is_err());
        assert!(validate_element_id("\t\n ").is_err());
    }
}

/// 验证错误消息测试
#[cfg(test)]
mod validation_error_message_tests {
    use super::*;

    #[test]
    fn test_card_id_error_message_content() {
        let result = validate_card_id("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("card_id") || msg.contains("卡片"));
    }

    #[test]
    fn test_element_id_error_message_content() {
        let result = validate_element_id("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("element_id") || msg.contains("组件"));
    }

    #[test]
    fn test_id_type_error_message_with_custom_field() {
        let result = validate_id_type("", "自定义字段");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("自定义字段"));
    }

    #[test]
    fn test_id_list_error_message_with_custom_field() {
        let result = validate_id_list(&[], "自定义列表");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("自定义列表"));
    }
}
