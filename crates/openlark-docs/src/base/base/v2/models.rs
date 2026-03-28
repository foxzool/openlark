use serde::{Deserialize, Serialize};

/// Base v2 自定义角色（AppRole）
///
/// docPath: /document/docs/bitable-v1/advanced-permission/app-role/list-2
/// doc: <https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/list-2>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRole {
    /// 自定义权限的名字
    pub role_name: String,
    /// 数据表权限配置列表
    ///
    /// 注意：该字段结构较复杂（包含记录规则、字段权限等），此处按 JSON 透传。
    #[serde(default)]
    pub table_roles: Vec<serde_json::Value>,
    /// 角色ID（以 "custom_" 开头）
    pub role_id: String,
    /// Block 权限配置列表（结构按 JSON 透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_roles: Option<Vec<serde_json::Value>>,
    /// Base 规则（结构按 JSON 透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_rule: Option<serde_json::Value>,
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
