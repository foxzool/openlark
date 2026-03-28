//! OAuth授权服务
//!
//! 提供OAuth 2.0授权流程支持，包括获取登录预授权码等功能。

use crate::auth::oauth::old::OAuthServiceOld;
use openlark_core::config::Config;

/// OAuth授权服务
///
/// 提供飞书开放平台的OAuth 2.0授权功能，支持：
/// - 获取登录预授权码
/// - 构建授权链接
/// - 处理授权回调
///
/// # 示例
///
/// ```rust
/// use openlark_auth::OAuthService;
/// use openlark_core::config::Config;
///
/// let config = Config::builder()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret")
///     .base_url("<https://open.feishu.cn")>
///     .build();
/// let oauth_service = OAuthService::new(config);
///
/// // 构建授权链接
/// let auth_url = oauth_service
///     .old()
///     .authorization()
///     .app_id("your_app_id")
///     .redirect_uri("<https://your-app.com/callback")>
///     .build_url();
///
/// println!("授权链接: {}", auth_url);
/// ```
#[derive(Debug, Clone)]
pub struct OAuthService {
    config: Config,
}

impl OAuthService {
    /// 创建新的OAuth授权服务实例
    ///
    /// # 参数
    ///
    /// * `config` - 飞书开放平台配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use openlark_auth::OAuthService;
    /// use openlark_core::config::Config;
    ///
    /// let config = Config::default();
    /// let oauth_service = OAuthService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置的引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取旧版本的OAuth授权服务
    ///
    /// 旧版本包含以下API：
    /// - `authorization.v1/index`: 获取登录预授权码
    ///
    /// # 示例
    ///
    /// ```rust
    /// use openlark_auth::OAuthService;
    /// use openlark_core::config::Config;
    ///
    /// let config = Config::default();
    /// let oauth_service = OAuthService::new(config);
    /// let old_service = oauth_service.old();
    /// ```
    pub fn old(&self) -> OAuthServiceOld {
        OAuthServiceOld::new(self.config.clone())
    }
}

impl Default for OAuthService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_service_creation() {
        let config = Config::default();
        let service = OAuthService::new(config.clone());

        assert_eq!(service.config().app_id(), config.app_id());
    }

    #[test]
    fn test_old_service() {
        let config = Config::default();
        let service = OAuthService::new(config);
        let old_service = service.old();

        // 验证返回的是有效的服务实例
        let _ = format!("{:?}", old_service);
    }

    #[test]
    fn test_default() {
        let service = OAuthService::default();
        let _ = format!("{:?}", service);
    }
}
