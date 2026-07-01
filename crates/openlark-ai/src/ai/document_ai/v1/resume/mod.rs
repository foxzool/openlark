/// parse 模块。
pub mod parse;

use openlark_core::config::Config;
use std::sync::Arc;

/// 简历解析资源服务（对齐 URL /document_ai/v1/resume）。
#[derive(Debug, Clone)]
pub struct ResumeService {
    config: Arc<Config>,
}

impl ResumeService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 简历解析（对齐 URL /document_ai/v1/resume/parse）。
    pub fn parse(&self) -> parse::ResumeParseRequestBuilder {
        parse::ResumeParseRequestBuilder::new((*self.config).clone())
    }
}
