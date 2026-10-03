//! OAuth相关数据模型
//!
//! 本模块包含OAuth授权相关的数据结构定义。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 授权码请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationCodeRequest {
    /// 应用ID
    pub app_id: String,
    /// 重定向URI
    pub redirect_uri: String,
    /// 授权范围
    pub scope: Option<String>,
    /// 状态参数
    pub state: Option<String>,
    /// 响应类型
    pub response_type: Option<String>,
}

/// 授权码响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationCodeResponse {
    /// 授权码
    pub code: String,
    /// 状态参数
    pub state: Option<String>,
}

/// 授权链接构建器
#[derive(Debug, Clone)]
pub struct AuthorizationUrlBuilder {
    app_id: String,
    redirect_uri: String,
    scope: Option<String>,
    state: Option<String>,
    response_type: String,
    base_url: String,
}

impl AuthorizationUrlBuilder {
    /// 创建新的授权链接构建器
    ///
    /// # 参数
    ///
    /// * `app_id` - 应用ID
    /// * `redirect_uri` - 重定向URI
    pub fn new(app_id: String, redirect_uri: String) -> Self {
        Self {
            app_id,
            redirect_uri,
            scope: None,
            state: None,
            response_type: "code".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
        }
    }

    /// 设置授权范围
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// 设置状态参数
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// 设置响应类型
    pub fn response_type(mut self, response_type: impl Into<String>) -> Self {
        self.response_type = response_type.into();
        self
    }

    /// 设置基础URL
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// 构建授权链接
    pub fn build_url(self) -> String {
        let mut url = format!(
            "{}/open-apis/authen/v1/index?app_id={}&redirect_uri={}",
            self.base_url,
            urlencoding::encode(&self.app_id),
            urlencoding::encode(&self.redirect_uri)
        );

        if let Some(scope) = &self.scope {
            url.push_str(&format!("&scope={}", urlencoding::encode(scope)));
        }

        if let Some(state) = &self.state {
            url.push_str(&format!("&state={}", urlencoding::encode(state)));
        }

        url.push_str(&format!("&response_type={}", self.response_type));

        url
    }
}

/// OAuth配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 重定向URI
    pub redirect_uri: String,
    /// 授权范围
    pub scope: Option<String>,
    /// 基础URL
    pub base_url: String,
}

impl Default for OAuthConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            redirect_uri: String::new(),
            scope: None,
            base_url: "https://open.feishu.cn".to_string(),
        }
    }
}

impl OAuthConfig {
    /// 创建新的OAuth配置
    pub fn new(app_id: String, app_secret: String, redirect_uri: String) -> Self {
        Self {
            app_id,
            app_secret,
            redirect_uri,
            scope: None,
            base_url: "https://open.feishu.cn".to_string(),
        }
    }

    /// 设置授权范围
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// 设置基础URL
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// 构建授权链接
    pub fn build_authorization_url(&self, state: Option<String>) -> String {
        let mut builder =
            AuthorizationUrlBuilder::new(self.app_id.clone(), self.redirect_uri.clone());

        if let Some(scope) = &self.scope {
            builder = builder.scope(scope);
        }

        if let Some(state) = state {
            builder = builder.state(state);
        }

        builder.base_url(&self.base_url).build_url()
    }
}

/// 授权状态信息
#[derive(Debug, Clone)]
pub struct AuthorizationState {
    /// 状态值
    pub state: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 过期时间（通常30分钟）
    pub expires_at: DateTime<Utc>,
    /// 关联的应用ID
    pub app_id: String,
    /// 重定向URI
    pub redirect_uri: String,
}

impl AuthorizationState {
    /// 创建新的授权状态
    pub fn new(app_id: String, redirect_uri: String) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::minutes(30);
        let state = uuid::Uuid::new_v4().to_string();

        Self {
            state,
            created_at: now,
            expires_at,
            app_id,
            redirect_uri,
        }
    }

    /// 检查状态是否已过期
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    /// 检查状态是否有效
    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.state.is_empty()
    }

    /// 获取剩余有效时间（秒）
    pub fn remaining_seconds(&self) -> i64 {
        (self.expires_at - Utc::now()).num_seconds().max(0)
    }
}

/// OAuth错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OAuthError {
    /// 无效的客户端
    InvalidClient,
    /// 无效的授权范围
    InvalidScope,
    /// 无效的授权码
    InvalidGrant,
    /// 无效的重定向URI
    InvalidRedirectUri,
    /// 授权码已过期
    ExpiredGrant,
    /// 授权码已被使用
    UsedGrant,
    /// 访问被拒绝
    AccessDenied,
    /// 服务器错误
    ServerError,
    /// 临时不可用
    TemporarilyUnavailable,
    /// 其他错误
    Other(String),
}

impl OAuthError {
    /// 从字符串解析OAuth错误
    pub fn from_string(error: &str) -> Self {
        match error {
            "invalid_client" => OAuthError::InvalidClient,
            "invalid_scope" => OAuthError::InvalidScope,
            "invalid_grant" => OAuthError::InvalidGrant,
            "invalid_redirect_uri" => OAuthError::InvalidRedirectUri,
            "expired_grant" => OAuthError::ExpiredGrant,
            "used_grant" => OAuthError::UsedGrant,
            "access_denied" => OAuthError::AccessDenied,
            "server_error" => OAuthError::ServerError,
            "temporarily_unavailable" => OAuthError::TemporarilyUnavailable,
            other => OAuthError::Other(other.to_string()),
        }
    }

    /// 转换为错误字符串
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            OAuthError::InvalidClient => "invalid_client".to_string(),
            OAuthError::InvalidScope => "invalid_scope".to_string(),
            OAuthError::InvalidGrant => "invalid_grant".to_string(),
            OAuthError::InvalidRedirectUri => "invalid_redirect_uri".to_string(),
            OAuthError::ExpiredGrant => "expired_grant".to_string(),
            OAuthError::UsedGrant => "used_grant".to_string(),
            OAuthError::AccessDenied => "access_denied".to_string(),
            OAuthError::ServerError => "server_error".to_string(),
            OAuthError::TemporarilyUnavailable => "temporarily_unavailable".to_string(),
            OAuthError::Other(msg) => msg.clone(),
        }
    }

    /// 获取用户友好的错误描述
    pub fn user_friendly_message(&self) -> String {
        match self {
            OAuthError::InvalidClient => "无效的应用客户端信息".to_string(),
            OAuthError::InvalidScope => "无效的授权范围".to_string(),
            OAuthError::InvalidGrant => "无效的授权码".to_string(),
            OAuthError::InvalidRedirectUri => "无效的重定向地址".to_string(),
            OAuthError::ExpiredGrant => "授权码已过期，请重新授权".to_string(),
            OAuthError::UsedGrant => "授权码已被使用".to_string(),
            OAuthError::AccessDenied => "用户拒绝了授权请求".to_string(),
            OAuthError::ServerError => "服务器内部错误，请稍后重试".to_string(),
            OAuthError::TemporarilyUnavailable => "服务暂时不可用，请稍后重试".to_string(),
            OAuthError::Other(msg) => format!("授权失败: {msg}"),
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization_url_builder() {
        let url = AuthorizationUrlBuilder::new(
            "test_app_id".to_string(),
            "https://example.com/callback".to_string(),
        )
        .scope("user_info")
        .state("test_state")
        .build_url();

        assert!(url.contains("app_id=test_app_id"));
        assert!(url.contains("redirect_uri=https%3A%2F%2Fexample.com%2Fcallback"));
        assert!(url.contains("scope=user_info"));
        assert!(url.contains("state=test_state"));
    }

    #[test]
    fn test_oauth_config() {
        let config = OAuthConfig::new(
            "test_app_id".to_string(),
            "test_app_secret".to_string(),
            "https://example.com/callback".to_string(),
        )
        .scope("user_info");

        let auth_url = config.build_authorization_url(Some("test_state".to_string()));

        assert!(auth_url.contains("app_id=test_app_id"));
        assert!(auth_url.contains("scope=user_info"));
        assert!(auth_url.contains("state=test_state"));
    }

    #[test]
    fn test_authorization_state() {
        let state = AuthorizationState::new(
            "test_app_id".to_string(),
            "https://example.com/callback".to_string(),
        );

        assert_eq!(state.app_id, "test_app_id");
        assert!(!state.state.is_empty());
        assert!(state.is_valid());
        assert!(!state.is_expired());
        assert!(state.remaining_seconds() > 0);
    }

    #[test]
    fn test_oauth_error() {
        let error = OAuthError::InvalidGrant;
        assert_eq!(error.to_string(), "invalid_grant");
        assert!(!error.user_friendly_message().is_empty());

        let error2 = OAuthError::from_string("invalid_grant");
        assert_eq!(error.to_string(), error2.to_string());
    }
}
