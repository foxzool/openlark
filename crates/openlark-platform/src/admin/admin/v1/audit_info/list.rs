//! 获取审计日志列表 API
//!
//! API文档: https://open.feishu.cn/document/server-docs/security_and_compliance-v1/audit_log/audit_data_get
//! docPath: https://open.feishu.cn/document/server-docs/security_and_compliance-v1/audit_log/audit_data_get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取审计日志列表请求 Builder。
pub struct ListAuditInfoRequestBuilder {
    /// 查询起始时间。
    start_time: String,
    /// 查询结束时间。
    end_time: String,
    /// 分页大小。
    page_size: Option<u32>,
    /// 分页标记。
    page_token: Option<String>,
    /// 配置信息。
    config: Config,
}

impl ListAuditInfoRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            start_time: String::new(),
            end_time: String::new(),
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置查询起始时间。
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = start_time.into();
        self
    }

    /// 设置查询结束时间。
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = end_time.into();
        self
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListAuditInfoResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListAuditInfoResponse> {
        let mut url = format!(
            "/open-apis/admin/v1/audit_infos?start_time={}&end_time={}",
            self.start_time, self.end_time
        );

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

/// 获取审计日志列表响应。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListAuditInfoResponse {
    /// 审计日志条目列表。
    pub items: Vec<AuditInfoItem>,
    /// 分页标记。
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    pub has_more: bool,
}

/// 审计日志条目。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditInfoItem {
    /// 审计 ID。
    pub audit_id: String,
    /// 操作人 ID。
    pub operator_id: String,
    /// 操作类型。
    pub operation: String,
    /// 资源名称。
    pub resource: String,
    /// 操作时间戳。
    pub timestamp: String,
}

impl ApiResponseTrait for ListAuditInfoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to ListAuditInfoRequestBuilder, will be removed in v1.0 (#271)")]
pub type ListAuditInfoBuilder = ListAuditInfoRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = ListAuditInfoRequestBuilder::new(config.clone())
            .start_time("test".to_string())
            .end_time("test".to_string());
        let _ = request;
    }
}
