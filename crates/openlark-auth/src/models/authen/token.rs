//! 用户访问令牌相关模型

use serde::{Deserialize, Serialize};

/// 用户访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAccessTokenResponse {
    /// 用户访问令牌
    #[serde(alias = "access_token")]
    pub user_access_token: String,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 令牌有效期（秒）
    pub expires_in: u64,
    /// 令牌类型
    pub token_type: Option<String>,
    /// 刷新令牌有效期（秒）
    pub refresh_expires_in: Option<u64>,
    /// 授权范围
    pub scope: Option<String>,
}

/// 用户访问令牌请求（v1版本）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccessTokenV1Request {
    /// 授权类型
    pub grant_type: String,
    /// 登录预授权码
    #[serde(rename = "code")]
    pub grant_code: String,
    /// 应用ID（历史兼容字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 应用密钥（历史兼容字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
}

/// 用户访问令牌刷新请求（v1版本）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshUserAccessTokenV1Request {
    /// 授权类型
    pub grant_type: String,
    /// 刷新令牌
    pub refresh_token: String,
    /// 应用ID（历史兼容字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 应用密钥（历史兼容字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
}

/// OIDC用户访问令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcUserAccessTokenRequest {
    /// 授权码
    pub code: String,
    /// 授权验证码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,
    /// 授权流程
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    /// 客户端ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// 客户端密钥
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// 授权类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
}

/// OIDC用户访问令牌刷新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcRefreshUserAccessTokenRequest {
    /// 刷新令牌
    pub refresh_token: String,
    /// 客户端ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// 客户端密钥
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// 授权类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
}
