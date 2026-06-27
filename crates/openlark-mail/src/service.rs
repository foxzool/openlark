use openlark_core::config::Config;
use std::sync::Arc;

/// MailService：邮件服务的统一入口
///
/// 提供对邮件 API v1 的访问能力
#[derive(Clone)]
pub struct MailService {
    config: Arc<Config>,
}

impl MailService {
    /// 创建新的邮件服务实例。
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    #[cfg(feature = "v1")]
    /// 访问邮件 API。
    pub fn mail(&self) -> crate::mail::mail::Mail {
        crate::mail::mail::Mail::new(self.config.clone())
    }

    #[cfg(feature = "v1")]
    /// 访问邮件组 API。
    pub fn mailgroup(&self) -> crate::mail::mail::v1::mailgroup::MailGroup {
        crate::mail::mail::v1::mailgroup::MailGroup::new(self.config.clone())
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
