//! Auth相关数据模型
//!
//! 本模块包含企业认证相关的数据结构定义。

use chrono::{DateTime, Utc};
use openlark_core::api::responses::ApiResponseTrait;
use serde::{Deserialize, Serialize};

fn default_app_ticket_resend_success() -> bool {
    true
}

/// 访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccessTokenResponse {
    /// 应用访问令牌
    pub app_access_token: String,
    /// 令牌有效期（秒）
    #[serde(alias = "expire")]
    pub expires_in: u64,
    /// 租户标识（租户在飞书上的唯一标识）
    #[serde(default)]
    pub tenant_key: String,
    /// 令牌类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

/// 租户访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TenantAccessTokenResponse {
    /// 租户访问令牌
    pub tenant_access_token: String,
    /// 令牌有效期（秒）
    #[serde(alias = "expire")]
    pub expires_in: u64,
    /// 令牌类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

/// 应用票据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppTicketResponse {
    /// 应用票据
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub app_ticket: String,
    /// 操作结果
    #[serde(default = "default_app_ticket_resend_success")]
    pub success: bool,
    /// 错误信息
    #[serde(default, alias = "msg", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

// 为所有响应类型实现ApiResponseTrait
impl ApiResponseTrait for AccessTokenResponse {}
impl ApiResponseTrait for TenantAccessTokenResponse {}
impl ApiResponseTrait for AppTicketResponse {}

/// 应用访问令牌请求（自建应用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAccessTokenInternalRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
}

/// 应用访问令牌请求（商店应用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAccessTokenRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
}

/// 租户访问令牌请求（自建应用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantAccessTokenInternalRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
}

/// 租户访问令牌请求（商店应用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantAccessTokenRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 应用票据（通过事件推送获得）
    pub app_ticket: String,
}

/// 应用票据重发请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTicketResendRequest {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
}

/// 令牌信息
#[derive(Debug, Clone)]
pub struct TokenInfo {
    /// 访问令牌
    pub access_token: String,
    /// 过期时间
    pub expires_at: DateTime<Utc>,
    /// 令牌类型
    pub token_type: String,
    /// 获取时间
    pub created_at: DateTime<Utc>,
}

impl TokenInfo {
    /// 创建新的令牌信息
    pub fn new(access_token: String, expires_in: u64, token_type: String) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::seconds(expires_in as i64);

        Self {
            access_token,
            expires_at,
            token_type,
            created_at: now,
        }
    }

    /// 检查令牌是否已过期
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    /// 检查令牌是否即将过期（提前5分钟）
    pub fn is_expiring_soon(&self) -> bool {
        let soon = Utc::now() + chrono::Duration::minutes(5);
        soon >= self.expires_at
    }

    /// 获取剩余有效时间（秒）
    pub fn remaining_seconds(&self) -> i64 {
        (self.expires_at - Utc::now()).num_seconds().max(0)
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_access_token_response_serialization() {
        let response = AccessTokenResponse {
            app_access_token: "test_token".to_string(),
            expires_in: 3600,
            tenant_key: "test_tenant".to_string(),
            token_type: Some("Bearer".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("test_token"));
        assert!(json.contains("3600"));
        assert!(json.contains("test_tenant"));
    }

    #[test]
    fn test_token_info() {
        let token = TokenInfo::new("test_token".to_string(), 3600, "Bearer".to_string());

        assert_eq!(token.access_token, "test_token");
        assert_eq!(token.token_type, "Bearer");
        assert!(!token.is_expired());
        assert!(token.remaining_seconds() > 0);
    }

    #[test]
    fn test_expiring_token() {
        // 创建一个已过期的令牌
        let mut token = TokenInfo::new("test_token".to_string(), 1, "Bearer".to_string());
        token.expires_at = Utc::now() - chrono::Duration::seconds(1);

        assert!(token.is_expired());
        assert!(token.is_expiring_soon());
        assert_eq!(token.remaining_seconds(), 0);
    }
}
