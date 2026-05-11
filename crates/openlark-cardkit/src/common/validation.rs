//! CardKit 共享验证工具
//!
//! 提供卡片和组件相关的通用验证函数，消除 API 实现中的代码重复。

use openlark_core::{error, validate_required};

/// 验证卡片 ID 是否有效
///
/// # 参数
///
/// * `card_id` - 卡片 ID 字符串
///
/// # 返回
///
/// 成功返回 `Ok(())`，失败返回包含中文错误信息的 `CoreError`
///
/// # 示例
///
/// ```ignore
/// use openlark_cardkit::common::validate_card_id;
///
/// validate_card_id("card_123")?;  // Ok(())
/// validate_card_id("")?;          // Err(CoreError)
/// ```
pub fn validate_card_id(card_id: &str) -> Result<(), error::CoreError> {
    validate_required!(card_id, "card_id 不能为空");
    Ok(())
}

/// 验证组件 ID 是否有效
///
/// # 参数
///
/// * `element_id` - 组件 ID 字符串
///
/// # 返回
///
/// 成功返回 `Ok(())`，失败返回包含中文错误信息的 `CoreError`
///
/// # 示例
///
/// ```ignore
/// use openlark_cardkit::common::validate_element_id;
///
/// validate_element_id("elem_456")?;  // Ok(())
/// validate_element_id("")?;          // Err(CoreError)
/// ```
pub fn validate_element_id(element_id: &str) -> Result<(), error::CoreError> {
    validate_required!(element_id, "element_id 不能为空");
    Ok(())
}

/// 验证 ID 类型是否有效
///
/// # 参数
///
/// * `id_type` - ID 类型字符串
/// * `field_name` - 字段名称，用于生成错误信息
///
/// # 返回
///
/// 成功返回 `Ok(())`，失败返回包含中文错误信息的 `CoreError`
///
/// # 示例
///
/// ```ignore
/// use openlark_cardkit::common::validate_id_type;
///
/// validate_id_type("user_id", "用户ID")?;  // Ok(())
/// validate_id_type("", "用户ID")?;         // Err(CoreError)
/// ```
pub fn validate_id_type(id_type: &str, field_name: &str) -> Result<(), error::CoreError> {
    validate_required!(id_type, format!("{field_name} 不能为空"));
    Ok(())
}

/// 验证 ID 列表是否非空
///
/// # 参数
///
/// * `ids` - ID 列表
/// * `field_name` - 字段名称，用于生成错误信息
///
/// # 返回
///
/// 成功返回 `Ok(())`，失败返回包含中文错误信息的 `CoreError`
///
/// # 示例
///
/// ```ignore
/// use openlark_cardkit::common::validate_id_list;
///
/// validate_id_list(&["id1".to_string()], "用户ID列表")?;  // Ok(())
/// validate_id_list(&[], "用户ID列表")?;                   // Err(CoreError)
/// ```
pub fn validate_id_list(ids: &[String], field_name: &str) -> Result<(), error::CoreError> {
    if ids.is_empty() {
        Err(error::validation_error(
            format!("{field_name} 不能为空"),
            format!("{field_name} 必须包含至少一个 ID"),
        ))
    } else {
        Ok(())
    }
}
