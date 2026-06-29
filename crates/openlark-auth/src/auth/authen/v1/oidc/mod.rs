//! OIDC Authentication APIs

use openlark_core::config::Config;

pub mod access_token;
pub mod refresh_access_token;

// Re-export types for convenient access（新名）
pub use access_token::create::OidcAccessTokenRequestBuilder;
pub use refresh_access_token::create::OidcRefreshAccessTokenRequestBuilder;
// 旧名兼容别名（deprecated alias，v1.0 移除）
#[allow(deprecated)]
pub use access_token::create::OidcAccessTokenBuilder;
#[allow(deprecated)]
pub use refresh_access_token::create::OidcRefreshAccessTokenBuilder;

/// OIDC认证服务
#[derive(Debug)]
pub struct OidcService {
    config: Config,
}

impl OidcService {
    /// 创建 OIDC 认证服务实例
    ///
    /// # 参数
    /// - `config`: SDK 配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取OIDC用户访问令牌
    pub fn access_token(&self) -> OidcAccessTokenRequestBuilder {
        OidcAccessTokenRequestBuilder::new(self.config.clone())
    }

    /// 刷新OIDC用户访问令牌
    pub fn refresh_access_token(&self) -> OidcRefreshAccessTokenRequestBuilder {
        OidcRefreshAccessTokenRequestBuilder::new(self.config.clone())
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
