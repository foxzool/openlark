//! 用户身份认证模块
//!
//! 提供用户身份认证相关 API 的版本化入口，聚合用户信息、访问令牌与 OIDC 认证能力。
//!
//! ## 主要功能
//! - `v1`: 用户认证 v1 版本接口入口
//! - 用户访问令牌申请与刷新
//! - OIDC 访问令牌相关接口

// v1 模块显式导出（新名）
pub use v1::{
    AuthenServiceV1, OidcAccessTokenRequestBuilder, OidcRefreshAccessTokenRequestBuilder,
    OidcService, RefreshUserAccessTokenV1RequestBuilder, UserAccessTokenV1RequestBuilder,
    UserInfoRequestBuilder, UserInfoService,
};
// 旧名兼容别名（deprecated alias，v1.0 移除）
#[allow(deprecated)]
pub use v1::{
    OidcAccessTokenBuilder, OidcRefreshAccessTokenBuilder, RefreshUserAccessTokenV1Builder,
    UserAccessTokenV1Builder, UserInfoBuilder,
};

pub mod v1;
