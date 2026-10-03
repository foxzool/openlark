/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 银行卡识别资源服务（对齐 URL /document_ai/v1/bank_card）。
#[derive(Debug, Clone)]
pub struct BankCardService {
    config: Arc<Config>,
}

impl BankCardService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 银行卡识别（对齐 URL /document_ai/v1/bank_card/recognize）。
    pub fn recognize(&self) -> recognize::BankCardRecognizeRequestBuilder {
        recognize::BankCardRecognizeRequestBuilder::new((*self.config).clone())
    }
}
