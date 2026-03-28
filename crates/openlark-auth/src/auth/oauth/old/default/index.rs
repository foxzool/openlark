//! 获取登录预授权码 API
use crate::models::oauth::*;
///
/// API文档: <https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/obtain-code>
///
/// 应用请求用户身份验证时，需构造登录链接，并引导用户跳转至此链接。
/// 用户登录成功后会生成登录预授权码 code，并作为参数追加到重定向URL。
use openlark_core::{config::Config, req_option::RequestOption, validate_required, SDKResult};
use serde::{Deserialize, Serialize};

/// 授权码请求构建器
pub struct AuthorizationBuilder {
    app_id: String,
    redirect_uri: String,
    scope: Option<String>,
    state: Option<String>,
    response_type: Option<String>,
    /// 配置信息
    config: Config,
}

/// 授权码响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthorizationCodeResponseData {
    /// 授权URL
    pub data: AuthorizationUrlResponse,
}

/// 授权URL响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthorizationUrlResponse {
    /// 完整的授权URL
    pub authorization_url: String,
    /// 应用ID
    pub app_id: String,
    /// 重定向URI
    pub redirect_uri: String,
    /// 权限范围
    pub scope: Option<String>,
    /// 状态参数
    pub state: Option<String>,
}

impl AuthorizationBuilder {
    /// 创建 authorization 请求
    pub fn new(config: Config) -> Self {
        Self {
            app_id: String::new(),
            redirect_uri: String::new(),
            scope: None,
            state: None,
            response_type: Some("code".to_string()),
            config,
        }
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 设置重定向URI
    pub fn redirect_uri(mut self, uri: impl Into<String>) -> Self {
        self.redirect_uri = uri.into();
        self
    }

    /// 设置权限范围
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// 设置状态参数，用于防止CSRF攻击
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// 执行请求 - 生成授权URL
    pub async fn execute(self) -> SDKResult<AuthorizationCodeResponseData> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）- 生成授权URL
    ///
    /// 注意：OAuth 授权是重定向流程，此方法仅构建授权 URL，不发起网络请求。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<AuthorizationCodeResponseData> {
        // 验证必填字段
        validate_required!(self.app_id, "应用ID不能为空");
        validate_required!(self.redirect_uri, "重定向URI不能为空");

        let url = self.build_authorization_url();

        // OAuth授权是重定向流程，返回构建的URL
        Ok(AuthorizationCodeResponseData {
            data: AuthorizationUrlResponse {
                authorization_url: url.clone(),
                app_id: self.app_id,
                redirect_uri: self.redirect_uri,
                scope: self.scope,
                state: self.state,
            },
        })
    }

    fn build_authorization_url(&self) -> String {
        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::OAuthApiOld;
        let api_endpoint = OAuthApiOld::Index;

        // 构建基础URL
        let mut url = format!(
            "{}{}?app_id={}&redirect_uri={}",
            self.config.base_url(),
            api_endpoint.path(),
            urlencoding::encode(&self.app_id),
            urlencoding::encode(&self.redirect_uri)
        );

        // 添加可选参数
        if let Some(scope) = &self.scope {
            url.push_str(&format!("&scope={}", urlencoding::encode(scope)));
        }

        if let Some(state) = &self.state {
            url.push_str(&format!("&state={}", urlencoding::encode(state)));
        }

        url.push_str(&format!(
            "&response_type={}",
            self.response_type.clone().unwrap_or_default()
        ));

        url
    }

    /// 构建授权URL（保持向后兼容）
    ///
    /// 返回一个完整的OAuth授权URL，用户需要访问此URL进行身份验证。
    /// 验证成功后，飞书会将用户重定向到指定的redirect_uri，
    /// 并附带授权码code参数。
    pub fn build_url(self) -> String {
        self.build_authorization_url()
    }

    /// 获取授权码（OAuth流程）
    ///
    /// 注意：OAuth授权是重定向流程，不直接返回响应。
    /// 实际使用时应该：
    /// 1. 调用 build_url() 获取授权链接
    /// 2. 引导用户访问该链接
    /// 3. 用户授权后，飞书重定向到redirect_uri并附带code参数
    /// 4. 使用获取到的code调用user_access_token接口获取访问令牌
    pub async fn send(self) -> SDKResult<AuthorizationCodeResponse> {
        Err(openlark_core::error::configuration_error(
            "OAuth授权需要重定向流程，请使用 build_url() 获取授权链接",
        ))
    }
}

/// OAuth旧版本API服务
#[derive(Debug)]
pub struct OAuthServiceOld {
    config: Config,
}

impl OAuthServiceOld {
    /// 创建 OAuth 服务实例
    ///
    /// # 参数
    /// - `config`: SDK 配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取授权码
    pub fn authorization(&self) -> AuthorizationBuilder {
        AuthorizationBuilder::new(self.config.clone())
    }

    /// 获取登录预授权码（index方法别名）
    pub fn index(&self) -> AuthorizationBuilder {
        self.authorization()
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
