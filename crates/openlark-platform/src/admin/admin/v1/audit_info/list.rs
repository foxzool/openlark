//! 获取审计日志列表 API
//!
//! API文档: https://open.feishu.cn/document/server-docs/security_and_compliance-v1/audit_log/audit_data_get
//! docPath: https://open.feishu.cn/document/server-docs/security_and_compliance-v1/audit_log/audit_data_get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct ListAuditInfoBuilder {
    start_time: String,
    end_time: String,
    page_size: Option<u32>,
    page_token: Option<String>,
    config: Config,
}

impl ListAuditInfoBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            start_time: String::new(),
            end_time: String::new(),
            page_size: None,
            page_token: None,
            config,
        }
    }

    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = start_time.into();
        self
    }

    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = end_time.into();
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListAuditInfoResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListAuditInfoResponse> {
        let mut url = format!("/open-apis/admin/v1/audit_infos?start_time={}&end_time={}", 
            self.start_time, self.end_time);
        
        if let Some(size) = self.page_size {
            url.push_str(&format!("&page_size={}", size));
        }
        if let Some(token) = self.page_token {
            url.push_str(&format!("&page_token={}", token));
        }

        let api_request: ApiRequest<ListAuditInfoResponse> = ApiRequest::get(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取审计日志列表", "响应数据为空")
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListAuditInfoResponse {
    pub items: Vec<AuditInfoItem>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditInfoItem {
    pub audit_id: String,
    pub operator_id: String,
    pub operation: String,
    pub resource: String,
    pub timestamp: String,
}

impl ApiResponseTrait for ListAuditInfoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder().app_id("test_app").app_secret("test_secret").build();
        let request = ListAuditInfoBuilder::new(config.clone())
            .start_time("test".to_string())
            .end_time("test".to_string());
        let _ = request;
    }
}
