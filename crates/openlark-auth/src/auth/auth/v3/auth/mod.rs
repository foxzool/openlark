//! Auth v3 /auth 路径下的API实现
//!
//! 包含企业应用认证相关的具体API实现，按照CSV规范拆分为独立文件

use openlark_core::config::Config;

// 导入各个API实现
mod app_access_token;
mod app_access_token_internal;
mod app_ticket_resend;
mod tenant_access_token;
mod tenant_access_token_internal;

// 重新导出各个请求类型（新名）
pub use app_access_token::AppAccessTokenRequestBuilder;
pub use app_access_token_internal::AppAccessTokenInternalRequestBuilder;
pub use app_ticket_resend::AppTicketResendRequestBuilder;
pub use tenant_access_token::TenantAccessTokenRequestBuilder;
pub use tenant_access_token_internal::TenantAccessTokenInternalRequestBuilder;
// 旧名兼容别名（deprecated alias，v1.0 移除；TenantAccessTokenInternalRequestBuilder 已是目标名，无旧别名）
#[allow(deprecated)]
pub use app_access_token::AppAccessTokenBuilder;
#[allow(deprecated)]
pub use app_access_token_internal::AppAccessTokenInternalBuilder;
#[allow(deprecated)]
pub use app_ticket_resend::AppTicketResendBuilder;
#[allow(deprecated)]
pub use tenant_access_token::TenantAccessTokenBuilder;

/// Auth v3 API服务
#[derive(Debug)]
pub struct AuthServiceV3 {
    config: Config,
}

impl AuthServiceV3 {
    /// 创建 Auth v3 API 服务实例
    ///
    /// # 参数
    /// - `config`: SDK 配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 商店应用获取app_access_token
    pub fn app_access_token(&self) -> AppAccessTokenRequestBuilder {
        AppAccessTokenRequestBuilder::new(self.config.clone())
    }

    /// 自建应用获取app_access_token
    pub fn app_access_token_internal(&self) -> AppAccessTokenInternalRequestBuilder {
        AppAccessTokenInternalRequestBuilder::new(self.config.clone())
    }

    /// 商店应用获取tenant_access_token
    pub fn tenant_access_token(&self) -> TenantAccessTokenRequestBuilder {
        TenantAccessTokenRequestBuilder::new(self.config.clone())
    }

    /// 自建应用获取tenant_access_token
    pub fn tenant_access_token_internal(&self) -> TenantAccessTokenInternalRequestBuilder {
        TenantAccessTokenInternalRequestBuilder::new(self.config.clone())
    }

    /// 重新获取app_ticket
    pub fn app_ticket_resend(&self) -> AppTicketResendRequestBuilder {
        AppTicketResendRequestBuilder::new(self.config.clone())
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
