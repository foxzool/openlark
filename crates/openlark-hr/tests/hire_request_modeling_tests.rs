//! openlark-hr 招聘请求建模测试。
#![cfg(feature = "hire")]

use openlark_core::{config::Config, req_option::RequestOption};
use openlark_hr::hire::hire::{
    v1::{evaluation, evaluation_task, location, referral, referral_account, talent_object, todo},
    v2::{interview_record, talent},
};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path, query_param},
};

fn test_config(base_url: &str) -> Config {
    Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url(base_url)
        .enable_token_cache(false)
        .build()
}

fn auth_option() -> RequestOption {
    RequestOption::builder()
        .tenant_access_token("test_token")
        .build()
}

#[tokio::test]
async fn evaluation_task_requires_user_id() {
    let result = evaluation_task::list::ListRequest::new(test_config("https://127.0.0.1:9"))
        .page_size(10)
        .execute_with_options(auth_option())
        .await;

    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("user_id"));
}

#[tokio::test]
async fn location_list_requires_usage() {
    let result = location::list::ListRequest::new(test_config("https://127.0.0.1:9"))
        .page_size(10)
        .execute_with_options(auth_option())
        .await;

    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("usage"));
}

#[tokio::test]
async fn todo_list_requires_type() {
    let result = todo::list::ListRequest::new(test_config("https://127.0.0.1:9"))
        .page_size(10)
        .execute_with_options(auth_option())
        .await;

    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("todo_type"));
}

#[tokio::test]
async fn referral_search_requires_talent_id() {
    let result = referral::search::SearchRequest::new(test_config("https://127.0.0.1:9"))
        .user_id_type("open_id")
        .execute_with_options(auth_option())
        .await;

    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("talent_id"));
}

#[tokio::test]
async fn referral_account_create_requires_contact() {
    let result = referral_account::create::CreateRequest::new(test_config("https://127.0.0.1:9"))
        .execute_with_options(auth_option())
        .await;

    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("contact"));
}

#[tokio::test]
async fn evaluation_list_serializes_query_parameters() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/hire/v1/evaluations"))
        .and(header("Authorization", "Bearer test_token"))
        .and(query_param("application_id", "app_001"))
        .and(query_param("page_token", "token_1"))
        .and(query_param("page_size", "20"))
        .and(query_param("update_start_time", "1700000000000"))
        .and(query_param("update_end_time", "1700000009999"))
        .and(query_param("user_id_type", "open_id"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"code":0,"msg":"success","data":{"data":{"items":[]}}})),
        )
        .mount(&mock_server)
        .await;

    let resp = evaluation::list::ListRequest::new(test_config(&mock_server.uri()))
        .application_id("app_001")
        .page_token("token_1")
        .page_size(20)
        .update_start_time("1700000000000")
        .update_end_time("1700000009999")
        .user_id_type("open_id")
        .execute_with_options(auth_option())
        .await
        .unwrap();

    assert!(resp.items.is_empty());
}

#[tokio::test]
async fn referral_search_serializes_body_and_query() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/open-apis/hire/v1/referrals/search"))
        .and(header("Authorization", "Bearer test_token"))
        .and(query_param("user_id_type", "open_id"))
        .and(body_json(json!({
            "talent_id":"talent_001",
            "start_time":"1701226882718",
            "end_time":"1701226882719"
        })))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"code":0,"msg":"success","data":{"data":{"items":[]}}})),
        )
        .mount(&mock_server)
        .await;

    let resp = referral::search::SearchRequest::new(test_config(&mock_server.uri()))
        .user_id_type("open_id")
        .talent_id("talent_001")
        .start_time("1701226882718")
        .end_time("1701226882719")
        .execute_with_options(auth_option())
        .await
        .unwrap();

    assert!(resp.items.is_empty());
}

#[tokio::test]
async fn referral_account_create_serializes_nested_mobile_body() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/open-apis/hire/v1/referral_account"))
        .and(header("Authorization", "Bearer test_token"))
        .and(body_json(json!({
            "mobile": {"code": "86", "number": "18900001111"},
            "email": "hire@example.com"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"code":0,"msg":"success","data":{"account":{"account_id":"acc_001"}}}),
        ))
        .mount(&mock_server)
        .await;

    let resp = referral_account::create::CreateRequest::new(test_config(&mock_server.uri()))
        .mobile("86", "18900001111")
        .email("hire@example.com")
        .execute_with_options(auth_option())
        .await
        .unwrap();

    assert_eq!(
        resp.account
            .as_ref()
            .and_then(|account| account.account_id.as_deref()),
        Some("acc_001")
    );
}

#[tokio::test]
async fn interview_record_v2_list_serializes_ids_as_json_array_query() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/hire/v2/interview_records"))
        .and(header("Authorization", "Bearer test_token"))
        .and(query_param("ids", r#"["ir_001","ir_002"]"#))
        .and(query_param("page_size", "10"))
        .and(query_param("page_token", "cursor_1"))
        .and(query_param("user_id_type", "open_id"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"code":0,"msg":"success","data":{"data":{"items":[]}}})),
        )
        .mount(&mock_server)
        .await;

    let resp = interview_record::list::ListRequest::new(test_config(&mock_server.uri()))
        .ids(vec!["ir_001".to_string(), "ir_002".to_string()])
        .page_size(10)
        .page_token("cursor_1")
        .user_id_type("open_id")
        .execute_with_options(auth_option())
        .await
        .unwrap();

    assert!(resp.items.is_empty());
}

#[tokio::test]
async fn talent_object_query_is_still_parameterless_and_executable() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/hire/v1/talent_objects/query"))
        .and(header("Authorization", "Bearer test_token"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"code":0,"msg":"success","data":{"data":{"items":[]}}})),
        )
        .mount(&mock_server)
        .await;

    let resp = talent_object::query::QueryRequest::new(test_config(&mock_server.uri()))
        .execute_with_options(auth_option())
        .await
        .unwrap();

    assert!(resp.items.is_empty());
}

#[tokio::test]
async fn referral_account_assets_serializes_query_parameters() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/open-apis/hire/v1/referral_account/get_account_assets",
        ))
        .and(header("Authorization", "Bearer test_token"))
        .and(query_param("referral_account_id", "acc_123"))
        .and(query_param("user_id_type", "open_id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code":0,
            "msg":"success",
            "data":{"account":{"account_id":"acc_123"}}
        })))
        .mount(&mock_server)
        .await;

    let resp = referral_account::get_account_assets::GetAccountAssetsRequest::new(test_config(
        &mock_server.uri(),
    ))
    .account_id("acc_123".to_string())
    .user_id_type("open_id")
    .execute_with_options(auth_option())
    .await
    .unwrap();

    assert_eq!(
        resp.account.as_ref().and_then(|v| v.account_id.as_deref()),
        Some("acc_123")
    );
}

#[tokio::test]
async fn talent_v2_get_serializes_path_and_query() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/hire/v2/talents/talent_123"))
        .and(header("Authorization", "Bearer test_token"))
        .and(query_param("user_id_type", "open_id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code":0,
            "msg":"success",
            "data":{"talent_id":"talent_123","basic_info":{"name":"张三"}}
        })))
        .mount(&mock_server)
        .await;

    let resp = talent::get::GetRequest::new(test_config(&mock_server.uri()))
        .talent_id("talent_123".to_string())
        .user_id_type("open_id")
        .execute_with_options(auth_option())
        .await
        .unwrap();

    assert_eq!(resp.talent_id.as_deref(), Some("talent_123"));
}
