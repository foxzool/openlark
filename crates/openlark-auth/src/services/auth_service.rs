//! 企业认证服务
//!
//! 提供企业应用的认证功能，包括自建应用和应用商店应用的访问令牌管理。

use crate::auth::auth::v3::AuthServiceV3;
use openlark_core::config::Config;

/// 企业认证服务
///
/// 提供飞书开放平台的企业级认证功能，支持：
/// - 企业自建应用认证
/// - 应用商店应用认证
/// - 访问令牌获取和管理
/// - 应用票据处理
///
/// # 示例
///
/// ```rust
/// use openlark_auth::AuthService;
/// use openlark_core::config::Config;
///
/// let config = Config::builder()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret")
///     .base_url("<https://open.feishu.cn")>
///     .build();
/// let auth_service = AuthService::new(config);
///
/// // 构建「自建应用获取 app_access_token」请求（这里只演示构建，不发送网络请求）
/// let _builder = auth_service
///     .v3()
///     .app_access_token_internal()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret");
///
/// // 真正发送请求需要配合 `openlark_core::http::Transport` 进行请求执行。
/// ```
#[derive(Debug, Clone)]
pub struct AuthService {
    config: Config,
}

impl AuthService {
    /// 创建新的企业认证服务实例
    ///
    /// # 参数
    ///
    /// * `config` - 飞书开放平台配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use openlark_auth::AuthService;
    /// use openlark_core::config::Config;
    ///
    /// let config = Config::default();
    /// let auth_service = AuthService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置的引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取v3版本的认证服务
    ///
    /// v3版本包含以下API：
    /// - `app_access_token`: 商店应用获取app_access_token
    /// - `app_access_token_internal`: 自建应用获取app_access_token
    /// - `tenant_access_token`: 商店应用获取tenant_access_token
    /// - `tenant_access_token_internal`: 自建应用获取tenant_access_token
    /// - `app_ticket_resend`: 重新获取app_ticket
    ///
    /// # 示例
    ///
    /// ```rust
    /// use openlark_auth::AuthService;
    /// use openlark_core::config::Config;
    ///
    /// let config = Config::default();
    /// let auth_service = AuthService::new(config);
    /// let v3_service = auth_service.v3();
    /// ```
    pub fn v3(&self) -> AuthServiceV3 {
        AuthServiceV3::new(self.config.clone())
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_service_creation() {
        let config = Config::default();
        let service = AuthService::new(config.clone());

        assert_eq!(service.config().app_id(), config.app_id());
    }

    #[test]
    fn test_v3_service() {
        let config = Config::default();
        let service = AuthService::new(config);
        let v3_service = service.v3();

        // 验证返回的是有效的服务实例
        let _ = format!("{:?}", v3_service);
    }

    #[test]
    fn test_default() {
        let service = AuthService::default();
        let _ = format!("{:?}", service);
    }
}
