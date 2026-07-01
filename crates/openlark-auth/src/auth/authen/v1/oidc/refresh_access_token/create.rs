//! OIDC 用户访问令牌刷新API
//! docPath: <https://open.feishu.cn/document/historic-version/authen/create-4>
use crate::models::authen::{OidcRefreshUserAccessTokenRequest, UserAccessTokenResponse};
///
/// API文档: <https://open.feishu.cn/document/server-docs/user-authentication/access-token/oidc_refresh_access_token>
///
/// 通过 OIDC 刷新令牌获取新的用户访问令牌
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// OIDC 用户访问令牌刷新请求
pub struct OidcRefreshAccessTokenRequestBuilder {
    refresh_token: String,
    client_id: Option<String>,
    client_secret: Option<String>,
    grant_type: Option<String>,
    /// 配置信息
    config: Config,
}

/// OIDC 用户访问令牌刷新响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct OidcRefreshAccessTokenResponseData {
    /// 用户访问令牌响应
    pub data: UserAccessTokenResponse,
}

impl ApiResponseTrait for OidcRefreshAccessTokenResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl OidcRefreshAccessTokenRequestBuilder {
    /// 创建 oidc_refresh_access_token 请求
    pub fn new(config: Config) -> Self {
        Self {
            refresh_token: String::new(),
            client_id: None,
            client_secret: None,
            grant_type: Some("refresh_token".to_string()),
            config,
        }
    }

    /// 设置刷新令牌
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.refresh_token = refresh_token.into();
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
    pub async fn execute(self) -> SDKResult<OidcRefreshAccessTokenResponseData> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<OidcRefreshAccessTokenResponseData> {
        // 验证必填字段
        validate_required!(self.refresh_token, "刷新令牌不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::AuthenApiV1;
        let api_endpoint = AuthenApiV1::OidcRefreshAccessToken;

        // 构建请求体
        let request_body = OidcRefreshUserAccessTokenRequest {
            refresh_token: self.refresh_token.clone(),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            grant_type: self.grant_type.clone(),
        };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<OidcRefreshAccessTokenResponseData> =
            ApiRequest::post(api_endpoint.path())
                .body(serde_json::to_value(&request_body)?)
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("刷新 OIDC user_access_token", "响应数据为空")
        })
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to OidcRefreshAccessTokenRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type OidcRefreshAccessTokenBuilder = OidcRefreshAccessTokenRequestBuilder;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use openlark_core::req_option::RequestOption;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, header, method, path},
    };

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_oidc_refresh_access_token_builder_new() {
        let config = create_test_config();
        let builder = OidcRefreshAccessTokenRequestBuilder::new(config);
        assert!(builder.refresh_token.is_empty());
        assert!(builder.client_id.is_none());
        assert!(builder.client_secret.is_none());
        assert_eq!(builder.grant_type, Some("refresh_token".to_string()));
    }

    #[test]
    fn test_oidc_refresh_access_token_builder_chain() {
        let config = create_test_config();
        let builder = OidcRefreshAccessTokenRequestBuilder::new(config)
            .refresh_token("my_refresh_token")
            .client_id("my_client_id")
            .client_secret("my_client_secret")
            .grant_type("refresh_token");
        assert_eq!(builder.refresh_token, "my_refresh_token");
        assert_eq!(builder.client_id, Some("my_client_id".to_string()));
        assert_eq!(builder.client_secret, Some("my_client_secret".to_string()));
        assert_eq!(builder.grant_type, Some("refresh_token".to_string()));
    }

    #[test]
    fn test_oidc_refresh_access_token_builder_refresh_token_chained() {
        let config = create_test_config();
        let builder = OidcRefreshAccessTokenRequestBuilder::new(config)
            .refresh_token("chained_refresh_token");
        assert_eq!(builder.refresh_token, "chained_refresh_token");
    }

    #[test]
    fn test_oidc_refresh_access_token_builder_client_id_chained() {
        let config = create_test_config();
        let builder =
            OidcRefreshAccessTokenRequestBuilder::new(config).client_id("chained_client_id");
        assert_eq!(builder.client_id, Some("chained_client_id".to_string()));
    }

    #[test]
    fn test_oidc_refresh_access_token_builder_client_secret_chained() {
        let config = create_test_config();
        let builder = OidcRefreshAccessTokenRequestBuilder::new(config)
            .client_secret("chained_client_secret");
        assert_eq!(
            builder.client_secret,
            Some("chained_client_secret".to_string())
        );
    }

    #[test]
    fn test_oidc_refresh_access_token_builder_grant_type_chained() {
        let config = create_test_config();
        let builder = OidcRefreshAccessTokenRequestBuilder::new(config).grant_type("refresh_token");
        assert_eq!(builder.grant_type, Some("refresh_token".to_string()));
    }

    #[test]
    fn test_oidc_refresh_access_token_response_data_deserialization() {
        let json =
            r#"{"user_access_token":"token123","expires_in":7200,"refresh_token":"refresh456"}"#;
        let response: OidcRefreshAccessTokenResponseData =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(response.data.user_access_token, "token123");
        assert_eq!(response.data.expires_in, 7200);
        assert_eq!(response.data.refresh_token, Some("refresh456".to_string()));
    }

    #[test]
    fn test_oidc_refresh_access_token_response_data_format() {
        assert_eq!(
            OidcRefreshAccessTokenResponseData::data_format(),
            ResponseFormat::Data
        );
    }

    #[tokio::test]
    async fn test_execute_uses_json_body_and_access_token_response() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/authen/v1/oidc/refresh_access_token"))
            .and(header("authorization", "Bearer app_token"))
            .and(header("content-type", "application/json; charset=utf-8"))
            .and(body_json(json!({
                "grant_type": "refresh_token",
                "refresh_token": "old_oidc_refresh_token"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "access_token": "new-oidc-token",
                    "refresh_token": "new-oidc-refresh-token",
                    "token_type": "Bearer",
                    "expires_in": 7140,
                    "refresh_expires_in": 2591999,
                    "scope": "auth:user.id:read"
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        let option = RequestOption::builder()
            .app_access_token("app_token")
            .build();

        let response = OidcRefreshAccessTokenRequestBuilder::new(config)
            .refresh_token("old_oidc_refresh_token")
            .execute_with_options(option)
            .await
            .expect("OIDC refresh_access_token 请求应成功");

        assert_eq!(response.data.user_access_token, "new-oidc-token");
        assert_eq!(
            response.data.refresh_token,
            Some("new-oidc-refresh-token".to_string())
        );
    }
}
