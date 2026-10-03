use openlark_core::{config::Config, req_option::RequestOption};
use wiremock::MockServer;

pub(crate) const TEST_TENANT_TOKEN: &str = "test-tenant-token";
pub(crate) const TEST_USER_TOKEN: &str = "test-user-token";

/// 创建使用 tenant token 的 wiremock 测试传输环境。
pub(crate) async fn tenant_test_transport() -> (MockServer, Config, RequestOption) {
    let server = MockServer::start().await;
    let config = Config::builder()
        .app_id("ci_app_id")
        .app_secret("ci_app_secret")
        .base_url(server.uri())
        .enable_token_cache(false)
        .build();
    let option = RequestOption::builder()
        .tenant_access_token(TEST_TENANT_TOKEN)
        .build();

    (server, config, option)
}

/// 创建使用 user token 的 wiremock 测试传输环境。
pub(crate) async fn user_test_transport() -> (MockServer, Config, RequestOption) {
    let server = MockServer::start().await;
    let config = Config::builder()
        .app_id("ci_app_id")
        .app_secret("ci_app_secret")
        .base_url(server.uri())
        .enable_token_cache(false)
        .build();
    let option = RequestOption::builder()
        .user_access_token(TEST_USER_TOKEN)
        .build();

    (server, config, option)
}
