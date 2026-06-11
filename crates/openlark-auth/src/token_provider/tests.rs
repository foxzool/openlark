use super::AuthTokenProvider;
use openlark_core::{
    auth::{TokenProvider, TokenRequest},
    config::Config,
    constants::{AccessTokenType, AppType},
};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, method, path},
};

fn marketplace_config(base_url: impl Into<String>) -> Config {
    Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .app_type(AppType::Marketplace)
        .base_url(base_url)
        .build()
}

#[tokio::test]
async fn tenant_token_fetch_no_longer_uses_noop_provider() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("http://127.0.0.1:9")
        .build();

    let provider = AuthTokenProvider::new(config);
    let err = provider
        .get_token(TokenRequest::tenant())
        .await
        .expect_err("should fail on unreachable test endpoint");

    assert!(!err.to_string().contains("NoOpTokenProvider"));
}

#[tokio::test]
async fn tenant_cache_key_should_include_tenant_key() {
    let request = TokenRequest::tenant().tenant_key("tenant_key_001");

    let key = AuthTokenProvider::cache_key(&AccessTokenType::Tenant, &AppType::SelfBuild, &request);

    assert_eq!(key, "Tenant_SelfBuild_tenant_key_001");
}

#[tokio::test]
async fn app_cache_key_should_include_app_ticket_for_marketplace() {
    let request = TokenRequest::app().app_ticket("ticket_001");

    let key = AuthTokenProvider::cache_key(&AccessTokenType::App, &AppType::Marketplace, &request);

    assert!(key.starts_with("App_Marketplace_"));
    assert!(!key.contains("ticket_001"));
    assert_eq!(
        key,
        AuthTokenProvider::cache_key(&AccessTokenType::App, &AppType::Marketplace, &request)
    );
}

#[tokio::test]
async fn debug_output_should_not_include_marketplace_app_ticket() {
    let provider = AuthTokenProvider::new(marketplace_config("http://127.0.0.1:9"));
    let _ = provider
        .get_token(TokenRequest::app().app_ticket("ticket_001"))
        .await;

    let debug_output = format!("{provider:?}");

    assert!(!debug_output.contains("ticket_001"));
}

#[tokio::test]
async fn marketplace_app_token_requires_app_ticket() {
    let provider = AuthTokenProvider::new(marketplace_config("http://127.0.0.1:9"));

    let err = provider
        .get_token(TokenRequest::app())
        .await
        .expect_err("marketplace app token without app_ticket should fail before network");

    assert!(err.to_string().contains("app_ticket"));
}

#[tokio::test]
async fn marketplace_tenant_token_requires_tenant_key() {
    let provider = AuthTokenProvider::new(marketplace_config("http://127.0.0.1:9"));

    let err = provider
        .get_token(TokenRequest::tenant().app_ticket("ticket_001"))
        .await
        .expect_err("marketplace tenant token without tenant_key should fail before network");

    assert!(err.to_string().contains("tenant_key"));
}

#[tokio::test]
async fn marketplace_tenant_token_uses_app_token_and_tenant_key_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/open-apis/auth/v3/app_access_token"))
        .and(body_json(json!({
            "app_id": "test_app_id",
            "app_secret": "test_app_secret",
            "app_ticket": "ticket_001"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "app_access_token": "market-app-token",
            "expire": 7200
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/open-apis/auth/v3/tenant_access_token"))
        .and(body_json(json!({
            "app_access_token": "market-app-token",
            "tenant_key": "tenant_key_001"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "tenant_access_token": "tenant-token",
            "expire": 7200
        })))
        .mount(&server)
        .await;

    let provider = AuthTokenProvider::new(marketplace_config(server.uri()));

    let token = provider
        .get_token(
            TokenRequest::tenant()
                .tenant_key("tenant_key_001")
                .app_ticket("ticket_001"),
        )
        .await
        .expect("marketplace tenant token should be fetched through app token");

    assert_eq!(token, "tenant-token");

    let received_requests = server.received_requests().await.unwrap_or_default();
    assert_eq!(received_requests.len(), 2);
    assert!(
        received_requests
            .iter()
            .all(|request| !request.headers.contains_key("authorization"))
    );
}
