//! Passport v1 重置登录密码 API 的公开契约测试。

use openlark_auth::passport::passport::v1::password::update::{
    PassportUserIdType, UpdatePasswordBody, UpdatePasswordRequest,
};
use openlark_core::{config::Config, req_option::RequestOption};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path, query_param},
};

fn test_config(base_url: impl Into<String>) -> Config {
    Config::builder()
        .app_id("ci_app_id")
        .app_secret("ci_app_secret")
        .base_url(base_url)
        .enable_token_cache(false)
        .build()
}

#[tokio::test]
async fn update_password_rejects_blank_user_id_before_sending_request() {
    let server = MockServer::start().await;

    let result = UpdatePasswordRequest::new(test_config(server.uri()))
        .execute(UpdatePasswordBody {
            user_id: "   ".to_string(),
            password: Some("1234abcd".to_string()),
            require_reset: Some(true),
        })
        .await;

    let error = result.expect_err("空白 user_id 应在发起网络请求前被拒绝");
    assert!(error.to_string().contains("user_id"));
    assert!(
        server
            .received_requests()
            .await
            .unwrap_or_default()
            .is_empty()
    );
}

#[tokio::test]
async fn update_password_sends_official_put_contract_with_tenant_token() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/open-apis/passport/v1/password"))
        .and(query_param("user_id_type", "user_id"))
        .and(header("authorization", "Bearer tenant-token"))
        .and(header("x-trace-id", "issue-596"))
        .and(body_json(json!({
            "user_id": "u_596",
            "password": "1234abcd",
            "require_reset": true
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {}
        })))
        .mount(&server)
        .await;

    let option = RequestOption::builder()
        .tenant_access_token("tenant-token")
        .add_header("x-trace-id", "issue-596")
        .build();

    UpdatePasswordRequest::new(test_config(server.uri()))
        .user_id_type(PassportUserIdType::UserId)
        .execute_with_options(
            UpdatePasswordBody {
                user_id: "u_596".to_string(),
                password: Some("1234abcd".to_string()),
                require_reset: Some(true),
            },
            option,
        )
        .await
        .expect("重置登录密码请求应按官方契约成功");

    let received = server.received_requests().await.unwrap_or_default();
    assert_eq!(received.len(), 1);
    assert_eq!(received[0].method, "PUT");
}
