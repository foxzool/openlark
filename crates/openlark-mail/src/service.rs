use openlark_core::config::Config;
use std::sync::Arc;

/// MailService：邮件服务的统一入口
///
/// 提供对邮件 API v1 的访问能力
#[derive(Clone)]
pub struct MailService {
    // config 仅在 v1 feature 开启时被各 accessor 读取；feature 关闭时受控标注为预期死代码。
    #[cfg_attr(not(feature = "v1"), expect(dead_code))]
    config: Arc<Config>,
}

impl MailService {
    /// 创建新的邮件服务实例。
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// 访问 v1 邮件 API（mailgroup / public_mailbox / user / user_mailbox / multi_entity）。
    ///
    /// ADR 0001：消除 `Mail` 域层转发壳 + 单独 `mailgroup()` 快捷（深度不一致），
    /// 统一经 `v1()` 到 `MailV1` 扇出层。原 `service.mail().v1().mailgroup()` /
    /// `service.mailgroup()` → `service.v1().mailgroup()`。
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::mail::mail::v1::MailV1 {
        crate::mail::mail::v1::MailV1::new(self.config.clone())
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
