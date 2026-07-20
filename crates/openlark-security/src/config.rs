//! 安全服务配置（legacy，已在 v0.18 收缩）
//!
//! 仅用于向后兼容说明，请改用 core Config。
#![allow(deprecated)]

/// 安全服务配置（已废弃）
///
/// v0.18 起请直接使用 `openlark_core::config::Config` + `SecurityClient::from_config`。
#[deprecated(
    since = "0.18.0",
    note = "使用 openlark_core::config::Config 代替，并通过 SecurityClient::from_config 构造"
)]
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 基础URL
    pub base_url: String,
}

impl SecurityConfig {
    /// 创建新的安全配置实例
    pub fn new(app_id: impl Into<String>, app_secret: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            app_secret: app_secret.into(),
            base_url: "https://open.feishu.cn".to_string(),
        }
    }

    /// 设置基础URL
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.to_string();
        self
    }

    /// 获取应用访问令牌
    ///
    /// 使用 openlark-auth 的 AuthTokenProvider 获取真实的 app_access_token
    pub async fn get_app_access_token(&self) -> crate::SecurityResult<String> {
        use openlark_auth::AuthTokenProvider;
        use openlark_core::{
            auth::{TokenProvider, TokenRequest},
            config::Config,
        };

        // 从 SecurityConfig 创建 Config
        let config = Config::builder()
            .app_id(&self.app_id)
            .app_secret(&self.app_secret)
            .base_url(&self.base_url)
            .build();

        // 使用 AuthTokenProvider 获取 token
        let token_provider = AuthTokenProvider::new(config);
        let token: String = token_provider
            .get_token(TokenRequest::app())
            .await
            .map_err(|e: openlark_core::CoreError| {
                openlark_core::error::authentication_error(e.to_string())
            })?;

        Ok(token)
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            app_id: "".to_string(),
            app_secret: "".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
        }
    }
}
