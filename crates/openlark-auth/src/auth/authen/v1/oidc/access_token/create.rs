//! OIDC 用户访问令牌获取API
//! docPath: https://open.feishu.cn/document/historic-version/authen/create-3
use crate::models::authen::UserAccessTokenResponse;
///
/// API文档: https://open.feishu.cn/document/server-docs/user-authentication/access-token/oidc_access_token
///
/// 通过 OIDC 授权码获取用户访问令牌
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// OIDC 用户访问令牌请求
pub struct OidcAccessTokenBuilder {
    code: String,
    code_verifier: Option<String>,
    redirect_uri: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    grant_type: Option<String>,
    /// 配置信息
    config: Config,
}

/// OIDC 用户访问令牌响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OidcAccessTokenResponseData {
    /// 用户访问令牌响应
    pub data: UserAccessTokenResponse,
}

impl ApiResponseTrait for OidcAccessTokenResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl OidcAccessTokenBuilder {
    /// 创建 oidc_access_token 请求
    pub fn new(config: Config) -> Self {
        Self {
            code: String::new(),
            code_verifier: None,
            redirect_uri: None,
            client_id: None,
            client_secret: None,
            grant_type: Some("authorization_code".to_string()),
            config,
        }
    }

    /// 设置授权码
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = code.into();
        self
    }

    /// 设置授权验证码 (PKCE)
    pub fn code_verifier(mut self, code_verifier: impl Into<String>) -> Self {
        self.code_verifier = Some(code_verifier.into());
        self
    }

    /// 设置重定向URI
    pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.redirect_uri = Some(redirect_uri.into());
        self
    }

    /// 设置客户端ID
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// 设置客户端密钥
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }

    /// 设置授权类型
    pub fn grant_type(mut self, grant_type: impl Into<String>) -> Self {
        self.grant_type = Some(grant_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OidcAccessTokenResponseData> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<OidcAccessTokenResponseData> {
        // 验证必填字段
        validate_required!(self.code, "授权码不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::AuthenApiV1;
        let api_endpoint = AuthenApiV1::OidcAccessToken;

        // 构建表单数据
        let mut form_data = std::collections::HashMap::new();
        form_data.insert("code".to_string(), self.code.clone());
        if let Some(ref code_verifier) = self.code_verifier {
            form_data.insert("code_verifier".to_string(), code_verifier.clone());
        }
        if let Some(ref redirect_uri) = self.redirect_uri {
            form_data.insert("redirect_uri".to_string(), redirect_uri.clone());
        }
        if let Some(ref client_id) = self.client_id {
            form_data.insert("client_id".to_string(), client_id.clone());
        }
        if let Some(ref client_secret) = self.client_secret {
            form_data.insert("client_secret".to_string(), client_secret.clone());
        }
        if let Some(ref grant_type) = self.grant_type {
            form_data.insert("grant_type".to_string(), grant_type.clone());
        }

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<OidcAccessTokenResponseData> =
            ApiRequest::post(api_endpoint.path())
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(openlark_core::api::RequestData::Form(form_data));

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取 OIDC user_access_token", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_oidc_access_token_builder_new() {
        let config = create_test_config();
        let builder = OidcAccessTokenBuilder::new(config);
        assert!(builder.code.is_empty());
        assert!(builder.code_verifier.is_none());
        assert!(builder.redirect_uri.is_none());
        assert!(builder.client_id.is_none());
        assert!(builder.client_secret.is_none());
        assert_eq!(builder.grant_type, Some("authorization_code".to_string()));
    }

    #[test]
    fn test_oidc_access_token_builder_chain() {
        let config = create_test_config();
        let builder = OidcAccessTokenBuilder::new(config)
            .code("my_code")
            .code_verifier("my_verifier")
            .redirect_uri("https://example.com/callback")
            .client_id("my_client_id")
            .client_secret("my_client_secret")
            .grant_type("authorization_code");
        assert_eq!(builder.code, "my_code");
        assert_eq!(builder.code_verifier, Some("my_verifier".to_string()));
        assert_eq!(
            builder.redirect_uri,
            Some("https://example.com/callback".to_string())
        );
        assert_eq!(builder.client_id, Some("my_client_id".to_string()));
        assert_eq!(builder.client_secret, Some("my_client_secret".to_string()));
        assert_eq!(builder.grant_type, Some("authorization_code".to_string()));
    }

    #[test]
    fn test_oidc_access_token_builder_code_chained() {
        let config = create_test_config();
        let builder = OidcAccessTokenBuilder::new(config).code("chained_code");
        assert_eq!(builder.code, "chained_code");
    }

    #[test]
    fn test_oidc_access_token_builder_code_verifier_chained() {
        let config = create_test_config();
        let builder = OidcAccessTokenBuilder::new(config).code_verifier("chained_verifier");
        assert_eq!(builder.code_verifier, Some("chained_verifier".to_string()));
    }

    #[test]
    fn test_oidc_access_token_builder_redirect_uri_chained() {
        let config = create_test_config();
        let builder = OidcAccessTokenBuilder::new(config).redirect_uri("https://redirect.com");
        assert_eq!(
            builder.redirect_uri,
            Some("https://redirect.com".to_string())
        );
    }

    #[test]
    fn test_oidc_access_token_builder_client_id_chained() {
        let config = create_test_config();
        let builder = OidcAccessTokenBuilder::new(config).client_id("chained_client_id");
        assert_eq!(builder.client_id, Some("chained_client_id".to_string()));
    }

    #[test]
    fn test_oidc_access_token_builder_client_secret_chained() {
        let config = create_test_config();
        let builder = OidcAccessTokenBuilder::new(config).client_secret("chained_client_secret");
        assert_eq!(
            builder.client_secret,
            Some("chained_client_secret".to_string())
        );
    }

    #[test]
    fn test_oidc_access_token_response_data_deserialization() {
        let json = r#"{"data":{"user_access_token":"token123","expires_in":7200,"refresh_token":"refresh456"}}"#;
        let response: OidcAccessTokenResponseData =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(response.data.user_access_token, "token123");
        assert_eq!(response.data.expires_in, 7200);
        assert_eq!(response.data.refresh_token, Some("refresh456".to_string()));
    }

    #[test]
    fn test_oidc_access_token_response_data_format() {
        assert_eq!(
            OidcAccessTokenResponseData::data_format(),
            ResponseFormat::Data
        );
    }
}
