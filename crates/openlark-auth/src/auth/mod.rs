//! 认证主模块
//!
//! 提供飞书开放平台认证能力的统一入口，聚合应用认证、用户认证与 OAuth 授权相关子模块。
//!
//! ## 主要功能
//! - `auth`: 企业应用令牌与票据相关接口
//! - `authen`: 用户身份认证与 OIDC 相关接口
//! - `oauth`: OAuth 授权流程相关接口

#![allow(clippy::module_inception)]

// v3 auth 模块显式导出（新名）
pub use self::auth::v3::auth::{
    AppAccessTokenRequestBuilder, AppAccessTokenInternalRequestBuilder, AppTicketResendRequestBuilder,
    AuthServiceV3, TenantAccessTokenRequestBuilder, TenantAccessTokenInternalRequestBuilder,
};
// 旧名兼容别名（deprecated alias，v1.0 移除）
#[allow(deprecated)]
pub use self::auth::v3::auth::{
    AppAccessTokenBuilder, AppAccessTokenInternalBuilder, AppTicketResendBuilder,
    TenantAccessTokenBuilder,
};

pub mod auth;
pub mod authen;
pub mod oauth;
