//! Transport::request 契约基线（#430 / parent #422）
//!
//! 仅经 `Transport::request` + 本地 HTTP adapter（wiremock）断言：
//! 请求构造、认证、响应解码、大小限制与错误形态。
//! **不**调用 internal request builder / response handler。

use std::time::Duration;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::CoreError,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, body_string_contains, header, method, path, query_param},
};

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ContractData {
    id: i32,
    name: String,
}

impl ApiResponseTrait for ContractData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

fn contract_config(base_url: &str) -> Config {
    Config::builder()
        .app_id("contract_app_id")
        .app_secret("contract_app_secret")
        .base_url(base_url)
        .enable_token_cache(false)
        .build()
}

fn config_with_max_response(base_url: &str, max_response_size: u64) -> Config {
    Config::builder()
        .app_id("contract_app_id")
        .app_secret("contract_app_secret")
        .base_url(base_url)
        .enable_token_cache(false)
        .max_response_size(max_response_size)
        .build()
}

fn no_auth_option() -> RequestOption {
    RequestOption::default()
}

fn user_token_option(token: &str) -> RequestOption {
    RequestOption::builder().user_access_token(token).build()
}

fn get_req(path: &str) -> ApiRequest<()> {
    ApiRequest::get(path).with_supported_access_token_types(vec![AccessTokenType::None])
}

fn post_req(path: &str) -> ApiRequest<()> {
    ApiRequest::post(path).with_supported_access_token_types(vec![AccessTokenType::None])
}

fn success_envelope(data: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "code": 0,
        "msg": "success",
        "data": data
    })
}

// ---------------------------------------------------------------------------
// Construction: GET / JSON body / multipart / query / custom headers
// ---------------------------------------------------------------------------

#[tokio::test]
async fn transport_get_success_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/get"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_envelope(
            serde_json::json!({"id": 7, "name": "alice"}),
        )))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/get");

    let resp = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("GET should succeed");

    assert!(resp.is_success());
    assert_eq!(resp.code(), 0);
    let data = resp.data.expect("data present");
    assert_eq!(data.id, 7);
    assert_eq!(data.name, "alice");
}

#[tokio::test]
async fn transport_json_body_is_sent() {
    let server = MockServer::start().await;
    let expected_body = serde_json::json!({"title": "hello", "count": 3});

    // body_json 匹配序列化后的 JSON（不依赖 Content-Type 大小写/charset）
    Mock::given(method("POST"))
        .and(path("/open-apis/contract/json"))
        .and(body_json(expected_body.clone()))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_envelope(
            serde_json::json!({"id": 1, "name": "json"}),
        )))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = post_req("/open-apis/contract/json").body(RequestData::Json(expected_body));

    let resp = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("JSON POST should succeed");
    assert!(resp.is_success());
    assert_eq!(resp.data.as_ref().map(|d| d.name.as_str()), Some("json"));
}

#[tokio::test]
async fn transport_query_params_are_sent() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/query"))
        .and(query_param("page_size", "20"))
        .and(query_param("page_token", "tok-abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_envelope(
            serde_json::json!({"id": 2, "name": "queried"}),
        )))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/query")
        .query("page_size", "20")
        .query("page_token", "tok-abc");

    let resp = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("query GET should succeed");
    assert!(resp.is_success());
    assert_eq!(resp.data.as_ref().map(|d| d.id), Some(2));
}

#[tokio::test]
async fn transport_custom_headers_from_request_and_option() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/headers"))
        .and(header("x-from-request", "req-value"))
        .and(header("x-from-option", "opt-value"))
        .and(header("Open-Lark-Request-Id", "req-id-430"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_envelope(
            serde_json::json!({"id": 3, "name": "headers"}),
        )))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/headers").header("X-From-Request", "req-value");
    let option = RequestOption::builder()
        .request_id("req-id-430")
        .add_header("X-From-Option", "opt-value")
        .build();

    let resp = Transport::<ContractData>::request(req, &config, Some(option))
        .await
        .expect("custom headers request should succeed");
    assert!(resp.is_success());
}

#[tokio::test]
async fn transport_multipart_file_upload() {
    let server = MockServer::start().await;
    // multipart body is not pure JSON; assert file bytes and form field appear
    Mock::given(method("POST"))
        .and(path("/open-apis/contract/multipart"))
        .and(body_string_contains("file-bytes-430"))
        .and(body_string_contains("field_a"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_envelope(
            serde_json::json!({"id": 4, "name": "multipart"}),
        )))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let form = serde_json::json!({
        "file_name": "contract.bin",
        "field_a": "value-a"
    });
    let req = post_req("/open-apis/contract/multipart")
        .body(RequestData::Json(form))
        .file_content(b"file-bytes-430".to_vec());

    let resp = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("multipart should succeed");
    assert!(resp.is_success());
    assert_eq!(
        resp.data.as_ref().map(|d| d.name.as_str()),
        Some("multipart")
    );
}

// ---------------------------------------------------------------------------
// Auth / timeout / request option
// ---------------------------------------------------------------------------

#[tokio::test]
async fn transport_user_token_authorization_header() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/auth"))
        .and(header("Authorization", "Bearer user-token-430"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_envelope(
            serde_json::json!({"id": 5, "name": "authed"}),
        )))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req: ApiRequest<()> = ApiRequest::get("/open-apis/contract/auth")
        .with_supported_access_token_types(vec![AccessTokenType::User]);

    let resp =
        Transport::<ContractData>::request(req, &config, Some(user_token_option("user-token-430")))
            .await
            .expect("user token request should succeed");
    assert!(resp.is_success());
}

#[tokio::test]
async fn transport_tenant_token_authorization_header() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/tenant-auth"))
        .and(header("Authorization", "Bearer tenant-token-430"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_envelope(
            serde_json::json!({"id": 6, "name": "tenant"}),
        )))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req: ApiRequest<()> = ApiRequest::get("/open-apis/contract/tenant-auth")
        .with_supported_access_token_types(vec![AccessTokenType::Tenant]);
    let option = RequestOption::builder()
        .tenant_access_token("tenant-token-430")
        .build();

    let resp = Transport::<ContractData>::request(req, &config, Some(option))
        .await
        .expect("tenant token request should succeed");
    assert!(resp.is_success());
}

#[tokio::test]
async fn transport_request_timeout_returns_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/slow"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(Duration::from_secs(3))
                .set_body_json(success_envelope(
                    serde_json::json!({"id": 0, "name": "slow"}),
                )),
        )
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/slow").timeout(Duration::from_millis(80));

    let err = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect_err("timeout should surface as error");

    // reqwest timeout maps into CoreError network/timeout family
    match err {
        CoreError::Timeout { .. } | CoreError::Network(_) => {}
        other => panic!("expected timeout/network error, got: {other:?}"),
    }
}

#[tokio::test]
async fn transport_missing_user_token_is_validation_error() {
    // enable_token_cache=true 时 determine 保留声明的 User 类型，validate 要求 user_access_token
    let config = Config::builder()
        .app_id("contract_app_id")
        .app_secret("contract_app_secret")
        .base_url("http://127.0.0.1:9")
        .enable_token_cache(true)
        .build();
    let req: ApiRequest<()> = ApiRequest::get("/open-apis/contract/need-user")
        .with_supported_access_token_types(vec![AccessTokenType::User]);

    let err = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect_err("missing user token should fail before send");

    assert!(
        matches!(err, CoreError::Validation { .. }),
        "expected Validation, got: {err:?}"
    );
    let msg = err.to_string();
    assert!(
        msg.to_lowercase().contains("user") || msg.contains("access token"),
        "message should mention user token: {msg}"
    );
}

// ---------------------------------------------------------------------------
// Response decoding: success / API error / malformed / oversized
// ---------------------------------------------------------------------------

#[tokio::test]
async fn transport_api_error_envelope_is_returned() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/api-error"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": 99991663,
            "msg": "app ticket invalid",
            "data": null
        })))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/api-error");

    let resp = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("API error envelope still returns Response");

    assert!(!resp.is_success());
    assert_eq!(resp.code(), 99991663);
    assert!(resp.message().contains("app ticket invalid") || resp.msg().contains("invalid"));
    assert!(resp.data.is_none());
}

#[tokio::test]
async fn transport_malformed_payload_returns_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/malformed"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json{{{"))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/malformed");

    let err = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect_err("malformed JSON should fail decode");

    // Contract: decode failure is Err (not Ok with empty data). Variant is handler-defined.
    let msg = err.to_string();
    assert!(
        !msg.contains("AppId is empty"),
        "should not be config validation: {err:?}"
    );
}

#[tokio::test]
async fn transport_oversized_known_body_returns_response_too_large() {
    let server = MockServer::start().await;
    let body = "x".repeat(2048);
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/oversized"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_string(body),
        )
        .expect(1)
        .mount(&server)
        .await;

    let config = config_with_max_response(&server.uri(), 512);
    let req = get_req("/open-apis/contract/oversized");

    let err = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect_err("oversized body must error");

    match err {
        CoreError::ResponseTooLarge { limit, actual, .. } => {
            assert_eq!(limit, 512);
            assert!(actual > 512, "actual={actual}");
        }
        other => panic!("expected ResponseTooLarge, got: {other:?}"),
    }
}

#[tokio::test]
async fn transport_success_without_matching_data_still_returns_ok() {
    // Baseline of *current* Data-format behavior: success code with missing/unparseable
    // data field yields Ok(Response) with data=None (documented for future deepen work).
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/empty-data"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": 0,
            "msg": "success"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/empty-data");

    let resp = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("current baseline returns Ok");
    assert!(resp.is_success());
    assert!(resp.data.is_none());
}
