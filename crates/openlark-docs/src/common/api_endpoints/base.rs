//! Base API V2 端点枚举
//!
//! 提供 Base v2 的端点定义，支持 method、path 和认证要求的统一管理（#438 tracer）。

use super::CatalogEndpoint;
use openlark_core::api::HttpMethod;
use openlark_core::constants::AccessTokenType;

/// Base API V2 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum BaseApiV2 {
    /// 新增自定义角色
    RoleCreate(String),
    /// 更新自定义角色
    RoleUpdate(String, String),
    /// 列出自定义角色
    RoleList(String),
    /// 删除自定义角色
    RoleDelete(String, String),
}

impl BaseApiV2 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            BaseApiV2::RoleCreate(app_token) => {
                format!("/open-apis/base/v2/apps/{app_token}/roles")
            }
            BaseApiV2::RoleUpdate(app_token, role_id) => {
                format!("/open-apis/base/v2/apps/{app_token}/roles/{role_id}")
            }
            BaseApiV2::RoleList(app_token) => {
                format!("/open-apis/base/v2/apps/{app_token}/roles")
            }
            BaseApiV2::RoleDelete(app_token, role_id) => {
                format!("/open-apis/base/v2/apps/{app_token}/roles/{role_id}")
            }
        }
    }
}

impl CatalogEndpoint for BaseApiV2 {
    fn to_url(&self) -> String {
        // delegate to inherent for backward compat with direct .to_url() calls
        BaseApiV2::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            BaseApiV2::RoleCreate(_) => HttpMethod::Post,
            BaseApiV2::RoleUpdate(_, _) => HttpMethod::Put,
            BaseApiV2::RoleList(_) => HttpMethod::Get,
            BaseApiV2::RoleDelete(_, _) => HttpMethod::Delete,
        }
    }

    fn supported_access_token_types(&self) -> Option<Vec<AccessTokenType>> {
        // 证明 catalog 统一拥有认证要求（#438 tracer）；与 core 默认一致，但显式声明
        Some(vec![AccessTokenType::User, AccessTokenType::Tenant])
    }

    // to_request 使用 trait 默认实现，会应用 supported token types
}
