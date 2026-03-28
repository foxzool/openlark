//! 用户认证服务
//!
//! 提供用户身份认证功能，包括用户信息获取、访问令牌管理等。

use crate::auth::authen::v1::AuthenServiceV1;
use openlark_core::config::Config;

/// 用户认证服务
///
/// 提供飞书开放平台的用户级认证功能，支持：
/// - 用户信息获取
/// - 用户访问令牌获取和刷新
/// - OIDC认证流程
///
/// # 示例
///
/// ```rust
/// use openlark_auth::AuthenService;
/// use openlark_core::config::Config;
///
/// let config = Config::builder()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret")
///     .base_url("<https://open.feishu.cn")>
///     .build();
/// let authen_service = AuthenService::new(config);
///
/// // 构建「获取用户信息」请求（这里只演示构建，不发送网络请求）
/// let _builder = authen_service
///     .v1()
///     .user_info()
///     .get()
///     .user_access_token("user_access_token");
///
/// // 真正发送请求需要配合 `openlark_core::http::Transport` 进行请求执行。
/// ```
#[derive(Debug, Clone)]
pub struct AuthenService {
    config: Config,
}

impl AuthenService {
    /// 创建新的用户认证服务实例
    ///
    /// # 参数
    ///
    /// * `config` - 飞书开放平台配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use openlark_auth::AuthenService;
    /// use openlark_core::config::Config;
    ///
    /// let config = Config::default();
    /// let authen_service = AuthenService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置的引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取v1版本的用户认证服务
    ///
    /// v1版本包含以下API：
    /// - `user_info.get`: 获取用户信息
    /// - `access_token.create`: 获取user_access_token (v1版本)
    /// - `refresh_access_token.create`: 刷新user_access_token (v1版本)
    /// - `oidc.access_token.create`: 获取user_access_token (OIDC)
    /// - `oidc.refresh_access_token.create`: 刷新user_access_token (OIDC)
    ///
    /// # 示例
    ///
    /// ```rust
    /// use openlark_auth::AuthenService;
    /// use openlark_core::config::Config;
    ///
    /// let config = Config::default();
    /// let authen_service = AuthenService::new(config);
    /// let v1_service = authen_service.v1();
    /// ```
    pub fn v1(&self) -> AuthenServiceV1 {
        AuthenServiceV1::new(self.config.clone())
    }
}

impl Default for AuthenService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_authen_service_creation() {
        let config = Config::default();
        let service = AuthenService::new(config.clone());

        assert_eq!(service.config().app_id(), config.app_id());
    }

    #[test]
    fn test_v1_service() {
        let config = Config::default();
        let service = AuthenService::new(config);
        let v1_service = service.v1();

        // 验证返回的是有效的服务实例
        let _ = format!("{:?}", v1_service);
    }

    #[test]
    fn test_default() {
        let service = AuthenService::default();
        let _ = format!("{:?}", service);
    }
}
