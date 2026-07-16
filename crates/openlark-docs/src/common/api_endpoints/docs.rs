//! Docs API 端点目录。

use super::CatalogEndpoint;
use openlark_core::api::{ApiRequest, HttpMethod};
use openlark_core::constants::AccessTokenType;

/// Docs API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum DocsApiV1 {
    /// 获取云文档内容
    ContentGet,
}

impl DocsApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            DocsApiV1::ContentGet => "/open-apis/docs/v1/content".to_string(),
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for DocsApiV1 {
    fn to_url(&self) -> String {
        DocsApiV1::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn supported_access_token_types(&self) -> Option<Vec<AccessTokenType>> {
        Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
    }
}
