/// 表单字段相关模型
use serde::{Deserialize, Serialize};

/// 更新表单字段问题请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchFormFieldRequest {
    /// 前置字段 ID（用于调整问题顺序）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_field_id: Option<String>,
    /// 问题标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 问题描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 是否可见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

impl PatchFormFieldRequest {
    /// 创建空的更新请求
    pub fn new() -> Self {
        Self {
            pre_field_id: None,
            title: None,
            description: None,
            required: None,
            visible: None,
        }
    }

    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        // 至少要有一个字段需要更新
        if self.pre_field_id.is_none()
            && self.title.is_none()
            && self.description.is_none()
            && self.required.is_none()
            && self.visible.is_none()
        {
            return Err(openlark_core::CoreError::validation_msg("至少需要提供一个要更新的字段"));
        }

        // 如果提供了标题，不能为空
        if let Some(ref title) = self.title
        {
            validate_required!(title, "问题标题不能为空");
        }

        Ok(())
    }
}

impl Default for PatchFormFieldRequest {
    fn default() -> Self {
        Self::new()
    }
}

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
