//! Document AI 集成测试（第二代 ai/ 下实现）

use openlark_ai::ai::document_ai::v1::{
    bank_card::recognize::BankCardRecognizeRequestBuilder,
    business_license::recognize::BusinessLicenseRecognizeRequestBuilder,
    id_card::recognize::IdCardRecognizeRequestBuilder, resume::parse::ResumeParseRequestBuilder,
    vat_invoice::recognize::VatInvoiceRecognizeRequestBuilder,
};
use openlark_ai::prelude::*;

/// 测试简历解析请求构建器
#[test]
fn test_resume_parse_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let request = ResumeParseRequestBuilder::new(config).file_token("test_file_token");

    // 验证构建器创建成功 - body() 方法返回请求体
    let body = request.body();
    assert_eq!(body.file_token, "test_file_token");
}

/// 测试身份证识别请求构建器
#[test]
fn test_id_card_recognize_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let request = IdCardRecognizeRequestBuilder::new(config).file_token("test_file_token");

    let body = request.body();
    assert_eq!(body.file_token, "test_file_token");
}

/// 测试银行卡识别请求构建器
#[test]
fn test_bank_card_recognize_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let request = BankCardRecognizeRequestBuilder::new(config)
        .file(vec![1, 2, 3])
        .file_name("test.jpg");

    let body = request.body();
    assert_eq!(body.file_name, Some("test.jpg".to_string()));
}

/// 测试营业执照识别请求构建器
#[test]
fn test_business_license_recognize_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let request = BusinessLicenseRecognizeRequestBuilder::new(config).file_token("test_file_token");

    let body = request.body();
    assert_eq!(body.file_token, "test_file_token");
}

/// 测试增值税发票识别请求构建器
#[test]
fn test_vat_invoice_recognize_builder() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build();

    let request = VatInvoiceRecognizeRequestBuilder::new(config).file_token("test_file_token");

    let body = request.body();
    assert_eq!(body.file_token, "test_file_token");
}
