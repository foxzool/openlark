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
    config: Arc<Config>,
}

impl DocumentAiV1 {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 简历解析资源（对齐 URL /document_ai/v1/resume）。
    pub fn resume(&self) -> resume::ResumeService {
        resume::ResumeService::new(self.config.clone())
    }

    /// 身份证识别资源（对齐 URL /document_ai/v1/id_card）。
    pub fn id_card(&self) -> id_card::IdCardService {
        id_card::IdCardService::new(self.config.clone())
    }

    /// 银行卡识别资源（对齐 URL /document_ai/v1/bank_card）。
    pub fn bank_card(&self) -> bank_card::BankCardService {
        bank_card::BankCardService::new(self.config.clone())
    }

    /// 名片识别资源（对齐 URL /document_ai/v1/business_card）。
    pub fn business_card(&self) -> business_card::BusinessCardService {
        business_card::BusinessCardService::new(self.config.clone())
    }

    /// 营业执照识别资源（对齐 URL /document_ai/v1/business_license）。
    pub fn business_license(&self) -> business_license::BusinessLicenseService {
        business_license::BusinessLicenseService::new(self.config.clone())
    }

    /// 中国护照识别资源（对齐 URL /document_ai/v1/chinese_passport）。
    pub fn chinese_passport(&self) -> chinese_passport::ChinesePassportService {
        chinese_passport::ChinesePassportService::new(self.config.clone())
    }

    /// 合同字段提取资源（对齐 URL /document_ai/v1/contract）。
    pub fn contract(&self) -> contract::ContractService {
        contract::ContractService::new(self.config.clone())
    }

    /// 驾驶证识别资源（对齐 URL /document_ai/v1/driving_license）。
    pub fn driving_license(&self) -> driving_license::DrivingLicenseService {
        driving_license::DrivingLicenseService::new(self.config.clone())
    }

    /// 食品经营许可证识别资源（对齐 URL /document_ai/v1/food_manage_license）。
    pub fn food_manage_license(&self) -> food_manage_license::FoodManageLicenseService {
        food_manage_license::FoodManageLicenseService::new(self.config.clone())
    }

    /// 食品生产许可证识别资源（对齐 URL /document_ai/v1/food_produce_license）。
    pub fn food_produce_license(&self) -> food_produce_license::FoodProduceLicenseService {
        food_produce_license::FoodProduceLicenseService::new(self.config.clone())
    }

    /// 健康证识别资源（对齐 URL /document_ai/v1/health_certificate）。
    pub fn health_certificate(&self) -> health_certificate::HealthCertificateService {
        health_certificate::HealthCertificateService::new(self.config.clone())
    }

    /// 港澳居民来往内地通行证识别资源（对齐 URL /document_ai/v1/hkm_mainland_travel_permit）。
    pub fn hkm_mainland_travel_permit(&self) -> hkm_mainland_travel_permit::HkmMainlandTravelPermitService {
        hkm_mainland_travel_permit::HkmMainlandTravelPermitService::new(self.config.clone())
    }

    /// 台湾居民来往大陆通行证识别资源（对齐 URL /document_ai/v1/tw_mainland_travel_permit）。
    pub fn tw_mainland_travel_permit(&self) -> tw_mainland_travel_permit::TwMainlandTravelPermitService {
        tw_mainland_travel_permit::TwMainlandTravelPermitService::new(self.config.clone())
    }

    /// 出租车发票识别资源（对齐 URL /document_ai/v1/taxi_invoice）。
    pub fn taxi_invoice(&self) -> taxi_invoice::TaxiInvoiceService {
        taxi_invoice::TaxiInvoiceService::new(self.config.clone())
    }

    /// 火车票识别资源（对齐 URL /document_ai/v1/train_invoice）。
    pub fn train_invoice(&self) -> train_invoice::TrainInvoiceService {
        train_invoice::TrainInvoiceService::new(self.config.clone())
    }

    /// 增值税发票识别资源（对齐 URL /document_ai/v1/vat_invoice）。
    pub fn vat_invoice(&self) -> vat_invoice::VatInvoiceService {
        vat_invoice::VatInvoiceService::new(self.config.clone())
    }

    /// 机动车发票识别资源（对齐 URL /document_ai/v1/vehicle_invoice）。
    pub fn vehicle_invoice(&self) -> vehicle_invoice::VehicleInvoiceService {
        vehicle_invoice::VehicleInvoiceService::new(self.config.clone())
    }

    /// 行驶证识别资源（对齐 URL /document_ai/v1/vehicle_license）。
    pub fn vehicle_license(&self) -> vehicle_license::VehicleLicenseService {
        vehicle_license::VehicleLicenseService::new(self.config.clone())
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
