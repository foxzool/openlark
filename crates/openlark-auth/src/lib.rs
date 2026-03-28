//! # OpenLark 认证模块
//!
//! OpenLark SDK的认证和授权模块，提供飞书开放平台的完整认证解决方案。
//!
//! ## 功能特性
//!
//! - **令牌管理**: 自动处理访问令牌的获取、刷新和缓存
//! - **多种认证方式**: 支持企业自建应用、应用商店应用和用户认证
//! - **OAuth支持**: 完整的OAuth 2.0授权流程
//! - **类型安全**: 基于Rust类型系统的API设计
//! - **错误处理**: 统一的错误处理和用户友好的错误消息
//! - **异步支持**: 基于tokio的全异步API设计
//!
//! ## 模块组织
//!
//! - [`services`][]: 核心认证服务
//! - [`models`][]: 数据模型定义
//! - `auth`: 认证 API (包含 auth, authen, oauth 子模块)
//!
//! ## 快速开始
//!
//! ```rust,no_run
//! use openlark_auth::{AuthService, AuthenService, OAuthService};
//! use openlark_core::config::Config;
//!
//! let config = Config::builder()
//!     .app_id("your_app_id")
//!     .app_secret("your_app_secret")
//!     .base_url("<https://open.feishu.cn")>
//!     .build();
//!
//! // 企业自建应用认证（这里只演示构建请求，不发送网络请求）
//! let auth_service = AuthService::new(config.clone());
//! let _token_builder = auth_service
//!     .v3()
//!         .app_access_token_internal()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret");
//!
//! // 用户认证（这里只演示构建请求，不发送网络请求）
//! let authen_service = AuthenService::new(config.clone());
//! let _user_info_builder = authen_service
//!     .v1()
//!         .user_info()
//!         .get()
//!         .user_access_token("user_token");
//!
//! // OAuth（构建授权链接）
//! let oauth_service = OAuthService::new(config);
//! let _auth_url = oauth_service
//!     .old()
//!     .authorization()
//!     .app_id("your_app_id")
//!     .redirect_uri("<https://example.com/callback")>
//!     .build_url();
//! ```

pub mod auth;
pub mod common;
pub mod human_authentication;
pub mod models;
pub mod passport;
pub mod services;
pub mod token_provider;
pub mod verification;

// 重新导出核心类型，方便用户使用
pub use services::{AuthService, AuthenService, OAuthService};
pub use token_provider::AuthTokenProvider;

/// 认证模块的预导入，包含最常用的类型和特征
pub mod prelude {
    pub use crate::{AuthService, AuthTokenProvider, AuthenService, OAuthService};
    pub use openlark_core::{config::Config, error::SDKResult, http::Transport};
}

/// 认证模块版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::marker::PhantomData;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_prelude_imports() {
        // 确保prelude中的类型可以正常导入，避免unused import警告
        use crate::prelude::{AuthService, AuthenService, OAuthService, SDKResult};

        // 这里只是验证类型导入，不进行实际操作
        let _: String = VERSION.to_string();

        // 验证导入的类型存在
        let _ = PhantomData::<AuthService>;
        let _ = PhantomData::<AuthenService>;
        let _ = PhantomData::<OAuthService>;
        let _: PhantomData<SDKResult<()>>;
    }
}
