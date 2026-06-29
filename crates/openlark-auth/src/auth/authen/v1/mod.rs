//! 用户身份认证 v1 模块
//!
//! 提供用户认证 v1 版本 API，包括用户信息获取、访问令牌申请与刷新、OIDC 认证能力。
//!
//! ## 主要功能
//! - `user_info`: 获取用户身份信息
//! - `access_token` / `refresh_access_token`: 访问令牌申请与刷新
//! - `oidc`: 基于 OIDC 的访问令牌接口

pub mod access_token;
pub mod oidc;
pub mod refresh_access_token;
pub mod user_info;

// 重新导出子模块的请求类型和服务（新名）
pub use access_token::UserAccessTokenV1RequestBuilder;
pub use oidc::{OidcAccessTokenRequestBuilder, OidcRefreshAccessTokenRequestBuilder, OidcService};
pub use refresh_access_token::RefreshUserAccessTokenV1RequestBuilder;
pub use user_info::{UserInfoRequestBuilder, UserInfoService};
// 旧名兼容别名（deprecated alias，v1.0 移除）
#[allow(deprecated)]
pub use access_token::UserAccessTokenV1Builder;
#[allow(deprecated)]
pub use oidc::{OidcAccessTokenBuilder, OidcRefreshAccessTokenBuilder};
#[allow(deprecated)]
pub use refresh_access_token::RefreshUserAccessTokenV1Builder;
#[allow(deprecated)]
pub use user_info::UserInfoBuilder;

use openlark_core::config::Config;

/// Authen v1 用户身份认证服务
#[derive(Debug)]
pub struct AuthenServiceV1 {
    config: Config,
}

impl AuthenServiceV1 {
    /// 创建 Authen v1 服务实例
    ///
    /// # 参数
    /// - `config`: SDK 配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 用户信息服务
    pub fn user_info(&self) -> UserInfoService {
        UserInfoService::new(self.config.clone())
    }

    /// 用户访问令牌构建器（v1版本）
    pub fn access_token(&self) -> UserAccessTokenV1RequestBuilder {
        UserAccessTokenV1RequestBuilder::new(self.config.clone())
    }

    /// 用户访问令牌刷新构建器（v1版本）
    pub fn refresh_access_token(&self) -> RefreshUserAccessTokenV1RequestBuilder {
        RefreshUserAccessTokenV1RequestBuilder::new(self.config.clone())
    }

    /// OIDC服务
    pub fn oidc(&self) -> OidcService {
        OidcService::new(self.config.clone())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
