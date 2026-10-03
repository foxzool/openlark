/// field_extraction 模块。
pub mod field_extraction;

use openlark_core::config::Config;
use std::sync::Arc;

/// 合同字段提取资源服务（对齐 URL /document_ai/v1/contract）。
#[derive(Debug, Clone)]
pub struct ContractService {
    config: Arc<Config>,
}

impl ContractService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 合同字段提取（对齐 URL /document_ai/v1/contract/field_extraction）。
    pub fn field_extraction(&self) -> field_extraction::ContractFieldExtractionRequestBuilder {
        field_extraction::ContractFieldExtractionRequestBuilder::new((*self.config).clone())
    }
}
