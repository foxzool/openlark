//! Document AI V1 模块

/// bank_card 模块。
pub mod bank_card;
/// business_card 模块。
pub mod business_card;
/// business_license 模块。
pub mod business_license;
/// chinese_passport 模块。
pub mod chinese_passport;
/// contract 模块。
pub mod contract;
/// driving_license 模块。
pub mod driving_license;
/// food_manage_license 模块。
pub mod food_manage_license;
/// food_produce_license 模块。
pub mod food_produce_license;
/// health_certificate 模块。
pub mod health_certificate;
/// hkm_mainland_travel_permit 模块。
pub mod hkm_mainland_travel_permit;
/// id_card 模块。
pub mod id_card;
/// resume 模块。
pub mod resume;
/// taxi_invoice 模块。
pub mod taxi_invoice;
/// train_invoice 模块。
pub mod train_invoice;
/// tw_mainland_travel_permit 模块。
pub mod tw_mainland_travel_permit;
/// vat_invoice 模块。
pub mod vat_invoice;
/// vehicle_invoice 模块。
pub mod vehicle_invoice;
/// vehicle_license 模块。
pub mod vehicle_license;

use openlark_core::config::Config;
use std::sync::Arc;

/// Document AI V1 API
#[derive(Clone)]
pub struct DocumentAiV1 {
    // reserved：待装访问器/execute（见 #274，不完整脚手架）
    _config: Arc<Config>,
}

impl DocumentAiV1 {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { _config: config }
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
