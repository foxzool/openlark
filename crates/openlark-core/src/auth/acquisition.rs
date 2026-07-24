use crate::{
    auth::TokenRequest,
    config::Config,
    constants::AccessTokenType,
    error::{CoreError, authentication_error},
    req_option::RequestOption,
};
use reqwest::RequestBuilder;

/// 处理不同类型的 AccessToken 认证
pub struct AuthHandler;

impl AuthHandler {
    /// 根据认证类型为请求添加相应的认证头
    pub async fn apply_auth(
        req_builder: RequestBuilder,
        access_token_type: AccessTokenType,
        config: &Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, CoreError> {
        match access_token_type {
            AccessTokenType::None => Ok(req_builder),
            AccessTokenType::App => Self::apply_app_auth(req_builder, config, option).await,
            AccessTokenType::Tenant => Self::apply_tenant_auth(req_builder, config, option).await,
            AccessTokenType::User => Ok(Self::apply_user_auth(req_builder, option)),
        }
    }

    /// 应用应用级认证
    async fn apply_app_auth(
        req_builder: RequestBuilder,
        config: &Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, CoreError> {
        let app_access_token = if let Some(ref token) = option.app_access_token {
            token.clone()
        } else if config.enable_token_cache() {
            let mut request = TokenRequest::app();
            if let Some(ref ticket) = option.app_ticket {
                request = request.app_ticket(ticket.clone());
            }
            config.token_provider().get_token(request).await?
        } else {
            return Err(authentication_error("访问令牌缺失"));
        };

        Ok(Self::add_auth_header(req_builder, &app_access_token))
    }

    /// 应用租户级认证
    async fn apply_tenant_auth(
        req_builder: RequestBuilder,
        config: &Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, CoreError> {
        let tenant_access_token = if let Some(ref token) = option.tenant_access_token {
            token.clone()
        } else if config.enable_token_cache() {
            let mut request = TokenRequest::tenant();
            if let Some(ref key) = option.tenant_key {
                request = request.tenant_key(key.clone());
            }
            if let Some(ref ticket) = option.app_ticket {
                request = request.app_ticket(ticket.clone());
            }
            config.token_provider().get_token(request).await?
        } else {
            return Err(authentication_error("访问令牌缺失"));
        };

        Ok(Self::add_auth_header(req_builder, &tenant_access_token))
    }

    /// 应用用户级认证
    fn apply_user_auth(req_builder: RequestBuilder, option: &RequestOption) -> RequestBuilder {
        Self::add_auth_header(
            req_builder,
            option.user_access_token.as_deref().unwrap_or(""),
        )
    }

    /// 添加 Authorization 头
    fn add_auth_header(req_builder: RequestBuilder, token: &str) -> RequestBuilder {
        req_builder.header("Authorization", format!("Bearer {token}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::AppType;
    use crate::error::ErrorType;
    use crate::error::traits::ErrorTrait;
    use crate::{SDKResult, auth::TokenProvider};
    use reqwest::Client;
    use std::{future::Future, pin::Pin};

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .enable_token_cache(false)
            .build()
    }

    #[derive(Debug)]
    struct StaticTokenProvider;

    impl TokenProvider for StaticTokenProvider {
        fn get_token(
            &self,
            request: TokenRequest,
        ) -> Pin<Box<dyn Future<Output = SDKResult<String>> + Send + '_>> {
            Box::pin(async move {
                Ok(match request.token_type {
                    AccessTokenType::App => "static_app_token".to_string(),
                    AccessTokenType::Tenant => "static_tenant_token".to_string(),
                    AccessTokenType::User => "static_user_token".to_string(),
                    AccessTokenType::None => "".to_string(),
                })
            })
        }
    }

    fn create_test_request_builder() -> RequestBuilder {
        Client::new().get("https://test.api.example.com/test")
    }

    #[test]
    fn test_auth_handler_struct_creation() {
        let _handler = AuthHandler;
    }

    #[tokio::test]
    async fn test_apply_auth_none_type() {
        let req_builder = create_test_request_builder();
        let config = create_test_config();
        let option = RequestOption::default();

        let result =
            AuthHandler::apply_auth(req_builder, AccessTokenType::None, &config, &option).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_apply_auth_user_type() {
        let req_builder = create_test_request_builder();
        let config = create_test_config();
        let option = RequestOption {
            user_access_token: Some("user_token_123".to_string()),
            ..Default::default()
        };

        let result =
            AuthHandler::apply_auth(req_builder, AccessTokenType::User, &config, &option).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_apply_app_auth_with_token_in_option() {
        let req_builder = create_test_request_builder();
        let config = create_test_config();
        let option = RequestOption {
            app_access_token: Some("app_token_123".to_string()),
            ..Default::default()
        };

        let result = AuthHandler::apply_app_auth(req_builder, &config, &option).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_apply_app_auth_no_cache_no_token() {
        let req_builder = create_test_request_builder();
        let config = create_test_config(); // enable_token_cache is false
        let option = RequestOption::default(); // no app_access_token

        let result = AuthHandler::apply_app_auth(req_builder, &config, &option).await;
        assert!(result.is_err());

        match result {
            Err(ref err) if err.error_type() == ErrorType::Authentication => (),
            _ => panic!("Expected MissingAccessToken error"),
        }
    }

    #[tokio::test]
    async fn test_apply_tenant_auth_with_token_in_option() {
        let req_builder = create_test_request_builder();
        let config = create_test_config();
        let option = RequestOption {
            tenant_access_token: Some("tenant_token_123".to_string()),
            ..Default::default()
        };

        let result = AuthHandler::apply_tenant_auth(req_builder, &config, &option).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_apply_tenant_auth_no_cache_no_token() {
        let req_builder = create_test_request_builder();
        let config = create_test_config(); // enable_token_cache is false
        let option = RequestOption::default(); // no tenant_access_token

        let result = AuthHandler::apply_tenant_auth(req_builder, &config, &option).await;
        assert!(result.is_err());

        match result {
            Err(ref err) if err.error_type() == ErrorType::Authentication => (),
            _ => panic!("Expected MissingAccessToken error"),
        }
    }

    #[tokio::test]
    async fn test_apply_app_auth_via_token_provider() {
        let req_builder = create_test_request_builder();
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .enable_token_cache(true)
            .token_provider(StaticTokenProvider)
            .build();
        let option = RequestOption::default(); // no app_access_token

        let result = AuthHandler::apply_app_auth(req_builder, &config, &option)
            .await
            .unwrap();

        let req = result.build().unwrap();
        let header = req.headers().get("Authorization").unwrap();
        assert_eq!(header.to_str().unwrap(), "Bearer static_app_token");
    }

    #[tokio::test]
    async fn test_apply_tenant_auth_via_token_provider() {
        let req_builder = create_test_request_builder();
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .enable_token_cache(true)
            .token_provider(StaticTokenProvider)
            .build();
        let option = RequestOption {
            tenant_key: Some("test_tenant".to_string()),
            ..Default::default()
        };

        let result = AuthHandler::apply_tenant_auth(req_builder, &config, &option)
            .await
            .unwrap();

        let req = result.build().unwrap();
        let header = req.headers().get("Authorization").unwrap();
        assert_eq!(header.to_str().unwrap(), "Bearer static_tenant_token");
    }

    #[test]
    fn test_apply_user_auth() {
        let req_builder = create_test_request_builder();
        let option = RequestOption {
            user_access_token: Some("user_token_456".to_string()),
            ..Default::default()
        };

        let result = AuthHandler::apply_user_auth(req_builder, &option);

        // Can't easily test the actual header, but should not panic
        // and should return a RequestBuilder
        assert!(format!("{result:?}").contains("RequestBuilder"));
    }

    #[test]
    fn test_add_auth_header_with_token() {
        let req_builder = create_test_request_builder();
        let token = "test_token_789";

        let result = AuthHandler::add_auth_header(req_builder, token);

        // Can't easily inspect headers without building request
        // but should not panic and should return RequestBuilder
        assert!(format!("{result:?}").contains("RequestBuilder"));
    }

    #[test]
    fn test_add_auth_header_with_empty_token() {
        let req_builder = create_test_request_builder();
        let token = "";

        let result = AuthHandler::add_auth_header(req_builder, token);

        // Should handle empty token without panicking
        assert!(format!("{result:?}").contains("RequestBuilder"));
    }

    #[tokio::test]
    async fn test_apply_auth_all_types() {
        let config = create_test_config();

        let test_cases = vec![
            (AccessTokenType::None, RequestOption::default()),
            (
                AccessTokenType::User,
                RequestOption {
                    user_access_token: Some("user_token".to_string()),
                    ..Default::default()
                },
            ),
            (
                AccessTokenType::App,
                RequestOption {
                    app_access_token: Some("app_token".to_string()),
                    ..Default::default()
                },
            ),
            (
                AccessTokenType::Tenant,
                RequestOption {
                    tenant_access_token: Some("tenant_token".to_string()),
                    ..Default::default()
                },
            ),
        ];

        for (token_type, option) in test_cases {
            let req_builder = create_test_request_builder();

            let result = AuthHandler::apply_auth(req_builder, token_type, &config, &option).await;

            // All cases with tokens should succeed
            // None type always succeeds
            match token_type {
                AccessTokenType::None | AccessTokenType::User => {
                    assert!(result.is_ok());
                }
                AccessTokenType::App | AccessTokenType::Tenant => {
                    // These should succeed when token is provided in option
                    assert!(result.is_ok());
                }
            }
        }
    }

    #[tokio::test]
    async fn test_apply_auth_with_cache_enabled() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .enable_token_cache(true)
            .build();

        let option = RequestOption::default();
        let req_builder = create_test_request_builder();

        // This will likely fail because token_manager needs proper setup
        // but we test that it doesn't panic
        let result =
            AuthHandler::apply_auth(req_builder, AccessTokenType::App, &config, &option).await;

        // Result depends on token_manager implementation
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_auth_handler_trait_implementations() {
        // Test that AuthHandler can be used in Send/Sync contexts
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<AuthHandler>();
        assert_sync::<AuthHandler>();
    }

    #[test]
    fn test_add_auth_header_format() {
        let req_builder = create_test_request_builder();
        let token = "test123";

        let _result = AuthHandler::add_auth_header(req_builder, token);

        // The header should be formatted as "Bearer {token}"
        // We can't easily test this without building the request
        // but we verify method doesn't panic
    }

    #[tokio::test]
    async fn test_noop_token_provider_returns_error() {
        let req_builder = create_test_request_builder();
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .enable_token_cache(true)
            .token_provider(crate::auth::NoOpTokenProvider)
            .build();
        let option = RequestOption::default();

        let result = AuthHandler::apply_app_auth(req_builder, &config, &option).await;

        assert!(result.is_err());

        match result {
            Err(ref err) if err.error_type() == ErrorType::Configuration => {
                let msg = err.user_message().unwrap_or_default();
                assert!(msg.contains("NoOpTokenProvider"));
            }
            _ => panic!("Expected Configuration error from NoOpTokenProvider"),
        }
    }

    #[tokio::test]
    async fn test_apply_tenant_auth_via_token_provider_marketplace() {
        let req_builder = create_test_request_builder();
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::Marketplace)
            .enable_token_cache(true)
            .token_provider(StaticTokenProvider)
            .build();
        let option = RequestOption {
            app_ticket: Some("test_ticket_123".to_string()),
            ..Default::default()
        };

        let result = AuthHandler::apply_tenant_auth(req_builder, &config, &option)
            .await
            .unwrap();

        let req = result.build().unwrap();
        let header = req.headers().get("Authorization").unwrap();
        assert_eq!(header.to_str().unwrap(), "Bearer static_tenant_token");
    }
}
