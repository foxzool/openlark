//! OpenLark Auth API 端点定义
//!
//! 提供统一的 API 端点管理和 URL 生成功能

use openlark_core::constants::{
    APP_ACCESS_TOKEN_INTERNAL_URL_PATH, APP_ACCESS_TOKEN_URL_PATH, APPLY_APP_TICKET_PATH,
    TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH, TENANT_ACCESS_TOKEN_URL_PATH,
};

/// Auth V3 API 端点枚举
#[derive(Debug, Clone)]
pub enum AuthApiV3 {
    /// 自建应用获取 app_access_token
    AppAccessTokenInternal,
    /// 自建应用获取 tenant_access_token
    TenantAccessTokenInternal,
    /// 重新获取 app_ticket
    AppTicketResend,
    /// 商店应用获取 app_access_token
    AppAccessToken,
    /// 商店应用获取 tenant_access_token
    TenantAccessToken,
}

impl AuthApiV3 {
    /// 获取对应的 API 路径
    pub fn path(&self) -> &'static str {
        match self {
            AuthApiV3::AppAccessTokenInternal => APP_ACCESS_TOKEN_INTERNAL_URL_PATH,
            AuthApiV3::TenantAccessTokenInternal => TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH,
            AuthApiV3::AppTicketResend => APPLY_APP_TICKET_PATH,
            AuthApiV3::AppAccessToken => APP_ACCESS_TOKEN_URL_PATH,
            AuthApiV3::TenantAccessToken => TENANT_ACCESS_TOKEN_URL_PATH,
        }
    }
}

/// Authen V1 API 端点枚举
#[derive(Debug, Clone)]
pub enum AuthenApiV1 {
    /// 获取用户信息
    UserInfo,
    /// 获取 user_access_token（v1 版本）
    AccessToken,
    /// 刷新 user_access_token（v1 版本）
    RefreshAccessToken,
    /// 获取 user_access_token (OIDC)
    OidcAccessToken,
    /// 刷新 user_access_token (OIDC)
    OidcRefreshAccessToken,
}

impl AuthenApiV1 {
    /// 获取对应的 API 路径
    pub fn path(&self) -> &'static str {
        match self {
            AuthenApiV1::UserInfo => "/open-apis/authen/v1/user_info",
            AuthenApiV1::AccessToken => "/open-apis/authen/v1/access_token",
            AuthenApiV1::RefreshAccessToken => "/open-apis/authen/v1/refresh_access_token",
            AuthenApiV1::OidcAccessToken => "/open-apis/authen/v1/oidc/access_token",
            AuthenApiV1::OidcRefreshAccessToken => "/open-apis/authen/v1/oidc/refresh_access_token",
        }
    }
}

/// OAuth Old API 端点枚举
#[derive(Debug, Clone)]
pub enum OAuthApiOld {
    /// 获取登录预授权码
    Index,
}

impl OAuthApiOld {
    /// 获取对应的 API 路径
    pub fn path(&self) -> &'static str {
        match self {
            OAuthApiOld::Index => "/open-apis/authen/v1/index",
        }
    }
}

/// Human Authentication V1 API 端点枚举
#[derive(Debug, Clone)]
pub enum HumanAuthenticationApiV1 {
    /// 录入身份信息
    IdentityCreate,
}

impl HumanAuthenticationApiV1 {
    /// 获取对应的 API 路径
    pub fn path(&self) -> &'static str {
        match self {
            HumanAuthenticationApiV1::IdentityCreate => {
                "/open-apis/human_authentication/v1/identities"
            }
        }
    }
}

/// 模块导出
pub mod prelude {
    pub use super::{AuthApiV3, AuthenApiV1, HumanAuthenticationApiV1, OAuthApiOld, PassportApiV1};
}

/// Passport V1 API 端点枚举
#[derive(Debug, Clone)]
pub enum PassportApiV1 {
    /// 重置登录密码
    PasswordUpdate,
    /// 批量查询用户登录会话
    SessionQuery,
    /// 登出用户会话
    SessionLogout,
}

impl PassportApiV1 {
    /// 获取对应的 API 路径
    pub fn path(&self) -> &'static str {
        match self {
            PassportApiV1::PasswordUpdate => "/open-apis/passport/v1/password",
            PassportApiV1::SessionQuery => "/open-apis/passport/v1/sessions/query",
            PassportApiV1::SessionLogout => "/open-apis/passport/v1/sessions/logout",
        }
    }
}
