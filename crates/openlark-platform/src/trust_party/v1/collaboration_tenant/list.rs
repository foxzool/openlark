//! 获取关联组织列表
//!
//! 文档: <https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取关联组织列表 Builder
#[derive(Debug, Clone)]
pub struct CollaborationTenantListBuilder {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl CollaborationTenantListBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationTenantListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationTenantListResponse> {
        let mut url = "/open-apis/trust_party/v1/collaboration_tenants".to_string();

        // 添加查询参数
        let mut params = Vec::new();
        if let Some(page_size) = self.page_size {
            params.push(format!("page_size={}", page_size));
        }
        if let Some(ref page_token) = self.page_token {
            params.push(format!("page_token={}", page_token));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let req: ApiRequest<CollaborationTenantListResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 关联组织列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationTenantListResponse {
    /// 关联组织列表
    pub items: Vec<CollaborationTenant>,
    /// 分页标记
    #[serde(rename = "page_token")]
    pub page_token: Option<String>,
    /// 是否还有更多
    #[serde(rename = "has_more")]
    pub has_more: Option<bool>,
}

/// 关联组织
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationTenant {
    /// 租户 key
    #[serde(rename = "tenant_key")]
    pub tenant_key: String,
    /// 租户名称
    #[serde(rename = "tenant_name")]
    pub tenant_name: String,
    /// 关联状态
    pub status: String,
}

impl ApiResponseTrait for CollaborationTenantListResponse {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CollaborationTenantListBuilder::new(config.clone())
            .page_size(1)
            .page_token("test".to_string());
        let _ = request;
    }
}
