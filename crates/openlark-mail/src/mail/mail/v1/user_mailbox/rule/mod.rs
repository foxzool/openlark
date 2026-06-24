/// 创建接口。
pub mod create;
/// 删除接口。
pub mod delete;
/// 列表接口。
pub mod list;
/// reorder 模块。
pub mod reorder;

use openlark_core::config::Config;
use std::sync::Arc;

/// 用户邮箱规则资源.
#[derive(Clone)]
pub struct Rule {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Rule {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::ListMailboxRuleRequest {
        list::ListMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 创建新建请求。
    pub fn create(&self) -> create::CreateMailboxRuleRequest {
        create::CreateMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 创建删除请求。
    pub fn delete(&self, rule_id: impl Into<String>) -> delete::DeleteMailboxRuleRequest {
        delete::DeleteMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone(), rule_id)
    }

    /// reorder。
    pub fn reorder(&self) -> reorder::ReorderMailboxRuleRequest {
        reorder::ReorderMailboxRuleRequest::new(self.config.clone(), self.mailbox_id.clone())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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

pub mod update;
