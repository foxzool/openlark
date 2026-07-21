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

/// Transfer-Encoding: chunked（无 Content-Length）且 body 超限 → 流式累计限流（#422 streamed oversized）。
#[tokio::test]
async fn transport_oversized_streamed_body_returns_response_too_large() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind chunked server");
    let addr = listener.local_addr().expect("local_addr");
    let body = "y".repeat(2048);
    let body_len_hex = format!("{:x}", body.len());

    tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.expect("accept");
        let mut buf = vec![0u8; 4096];
        // 读完请求头
        loop {
            let n = stream.read(&mut buf).await.expect("read req");
            if n == 0 {
                return;
            }
            if buf[..n].windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
        }
        // 无 Content-Length 的 chunked 响应 → content_length() 为 None，走流式累计
        let resp = format!(
            "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nContent-Type: application/json\r\n\r\n{body_len_hex}\r\n{body}\r\n0\r\n\r\n"
        );
        let _ = stream.write_all(resp.as_bytes()).await;
    });

    let config = config_with_max_response(&format!("http://{addr}"), 512);
    let req = get_req("/open-apis/contract/oversized-stream");

    let err = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect_err("streamed oversized body must error");

    match err {
        CoreError::ResponseTooLarge { limit, actual, .. } => {
            assert_eq!(limit, 512);
            assert!(actual > 512, "actual={actual}");
        }
        other => panic!("expected ResponseTooLarge from stream path, got: {other:?}"),
    }
}

/// 业务 code=10012 时触发 app_ticket resend 副作用（#422 app-ticket recovery）。
#[tokio::test]
async fn transport_app_ticket_invalid_triggers_resend() {
    use openlark_core::constants::{APPLY_APP_TICKET_PATH, ERR_CODE_APP_TICKET_INVALID};

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/open-apis/contract/app-ticket"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": ERR_CODE_APP_TICKET_INVALID,
            "msg": "app ticket invalid",
            "data": null
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path(APPLY_APP_TICKET_PATH))
        .and(body_json(serde_json::json!({
            "app_id": "contract_app_id",
            "app_secret": "contract_app_secret"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": 0,
            "msg": "success"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/app-ticket");

    let resp = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("API error envelope still returns Ok Response");
    assert!(!resp.is_success());
    assert_eq!(resp.code(), ERR_CODE_APP_TICKET_INVALID);
    // wiremock expect(1) on resend 在 drop server 时校验
}

/// 现状契约：Config.retry_count 不在 Transport 层触发自动重试（配置存在、链路未接线）。
#[tokio::test]
async fn transport_does_not_auto_retry_on_network_error() {
    let server = MockServer::start().await;
    // 故意不 mount 任何 handler → 404/连接成功但无 mock 匹配时可能空响应或 404
    // 使用已关闭的端口制造网络错误更稳：先启 mock 再 drop，改用无效路径 + 只 expect 0
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/no-retry"))
        .respond_with(ResponseTemplate::new(500).set_body_string("fail"))
        .expect(1) // 若自动重试 3 次会 fail expect
        .mount(&server)
        .await;

    let config = Config::builder()
        .app_id("contract_app_id")
        .app_secret("contract_app_secret")
        .base_url(server.uri())
        .enable_token_cache(false)
        .retry_count(3)
        .build();

    let req = get_req("/open-apis/contract/no-retry");
    // 500 + 非 JSON 可能 decode 失败，或成功解析失败 — 关键是只打 1 次
    let _ = Transport::<ContractData>::request(req, &config, Some(no_auth_option())).await;
    // Mock expect(1) 在 MockServer drop 时断言调用次数
}

#[tokio::test]
async fn transport_success_missing_required_data_returns_error() {
    // #431：成功 code=0 但缺少必需 data → 明确错误，不再 Ok(data=None)
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

    let err = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect_err("missing required data must error");
    assert!(
        matches!(err, CoreError::Validation { .. }),
        "expected Validation, got: {err:?}"
    );
    assert!(
        err.to_string().contains("data") || err.to_string().contains("必需"),
        "message should mention data: {err}"
    );
}

#[tokio::test]
async fn transport_unit_success_without_data_is_ok() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/unit-empty"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": 0,
            "msg": "success"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/unit-empty");
    let resp = Transport::<()>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("() does not require payload");
    assert!(resp.is_success());
    assert!(resp.data.is_none());
}

// ---------------------------------------------------------------------------
// Typed formats: Flatten / Text / Binary / Custom (#431)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct FlattenToken {
    code: i32,
    msg: String,
    app_access_token: String,
    #[serde(alias = "expire")]
    expires_in: i64,
}

impl ApiResponseTrait for FlattenToken {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Flatten
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextBody(String);

impl ApiResponseTrait for TextBody {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Text
    }

    fn from_text(text: String) -> Option<Self> {
        Some(Self(text))
    }
}

// Transport 要求 Deserialize 约束；Text 路径实际经 from_text，此 impl 满足编译边界。
impl<'de> Deserialize<'de> for TextBody {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Self(s))
    }
}

impl Serialize for TextBody {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BinaryFile {
    file_name: String,
    content: Vec<u8>,
}

impl ApiResponseTrait for BinaryFile {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Binary
    }

    fn from_binary(file_name: String, body: Vec<u8>) -> Option<Self> {
        Some(Self {
            file_name,
            content: body,
        })
    }
}

impl<'de> Deserialize<'de> for BinaryFile {
    fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom(
            "BinaryFile is only constructed via from_binary",
        ))
    }
}

impl Serialize for BinaryFile {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = serializer.serialize_struct("BinaryFile", 2)?;
        st.serialize_field("file_name", &self.file_name)?;
        st.serialize_field("content", &self.content)?;
        st.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CustomRaw(Vec<u8>);

impl ApiResponseTrait for CustomRaw {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Custom
    }

    fn from_custom(body: Vec<u8>, _content_type: Option<&str>) -> Option<Self> {
        Some(Self(body))
    }
}

impl<'de> Deserialize<'de> for CustomRaw {
    fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom("CustomRaw via from_custom only"))
    }
}

impl Serialize for CustomRaw {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UnimplementedCustom;

impl ApiResponseTrait for UnimplementedCustom {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Custom
    }
}

#[tokio::test]
async fn transport_flatten_success() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/flatten"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": 0,
            "msg": "ok",
            "app_access_token": "tok-flat",
            "expire": 3600
        })))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/flatten");
    let resp = Transport::<FlattenToken>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("flatten should decode");
    let data = resp.data.expect("flatten payload");
    assert_eq!(data.app_access_token, "tok-flat");
    assert_eq!(data.expires_in, 3600);
}

#[tokio::test]
async fn transport_text_format_does_not_use_data_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/text"))
        .respond_with(ResponseTemplate::new(200).set_body_string("plain-text-body"))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/text");
    let resp = Transport::<TextBody>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("text path should decode raw body");
    assert_eq!(
        resp.data.as_ref().map(|t| t.0.as_str()),
        Some("plain-text-body")
    );
}

#[tokio::test]
async fn transport_binary_preserves_filename_metadata() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/binary"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-disposition", "attachment; filename=\"report.bin\"")
                .set_body_bytes(b"BIN-CONTENT-430".as_slice()),
        )
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/binary");
    let resp = Transport::<BinaryFile>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("binary should decode via from_binary");
    let data = resp.data.expect("binary data");
    assert_eq!(data.file_name, "report.bin");
    assert_eq!(data.content, b"BIN-CONTENT-430");
}

#[tokio::test]
async fn transport_custom_format_uses_from_custom() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/custom"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"custom-bytes".as_slice()))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/custom");
    let resp = Transport::<CustomRaw>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("custom should decode");
    assert_eq!(
        resp.data.as_ref().map(|c| c.0.as_slice()),
        Some(b"custom-bytes".as_slice())
    );
}

#[tokio::test]
async fn transport_custom_unimplemented_returns_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/custom-unimplemented"))
        .respond_with(ResponseTemplate::new(200).set_body_string("whatever"))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/custom-unimplemented");
    let err = Transport::<UnimplementedCustom>::request(req, &config, Some(no_auth_option()))
        .await
        .expect_err("unimplemented custom must not fall through to Data");
    assert!(
        err.to_string().to_lowercase().contains("custom")
            || err.to_string().contains("from_custom"),
        "got: {err}"
    );
}

/// raw 路径：HTTP 非 2xx 不得伪装为 code=0 成功（Codex #451 Spec High）。
#[tokio::test]
async fn transport_binary_http_error_is_not_success_payload() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/binary-http-err"))
        .respond_with(
            ResponseTemplate::new(500)
                .insert_header("X-Request-Id", "rid-http-500")
                .set_body_string("internal server error"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/binary-http-err");
    let resp = Transport::<BinaryFile>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("HTTP error should still return Response, not panic");
    assert!(!resp.is_success());
    assert_eq!(resp.code(), 500);
    assert!(resp.data.is_none());
    assert_eq!(
        resp.raw().request_id.as_deref(),
        Some("rid-http-500"),
        "request_id from header must be preserved"
    );
}

/// raw 路径：HTTP 200 但 body 为业务错误 envelope，不得走 from_binary。
#[tokio::test]
async fn transport_binary_api_error_envelope_is_not_success_payload() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/binary-api-err"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": 99991672,
            "msg": "download denied",
            "request_id": "rid-bin-api"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/binary-api-err");
    let resp = Transport::<BinaryFile>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("api error envelope");
    assert!(!resp.is_success());
    assert_eq!(resp.code(), 99991672);
    assert!(resp.data.is_none());
    assert_eq!(resp.raw().request_id.as_deref(), Some("rid-bin-api"));
}

/// Data 路径保留 header / body 中的 request_id。
#[tokio::test]
async fn transport_data_preserves_request_id_from_header() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/rid"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("X-Request-Id", "rid-header-data")
                .set_body_json(success_envelope(
                    serde_json::json!({"id": 9, "name": "rid"}),
                )),
        )
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/rid");
    let resp = Transport::<ContractData>::request(req, &config, Some(no_auth_option()))
        .await
        .expect("success");
    assert!(resp.is_success());
    assert_eq!(resp.raw().request_id.as_deref(), Some("rid-header-data"));
}

/// Text 严格 UTF-8：非法字节返回解码错误（非 lossy 成功）。
#[tokio::test]
async fn transport_text_invalid_utf8_returns_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/text-bad-utf8"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes([0xff, 0xfe, 0xfd].as_slice()))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/text-bad-utf8");
    let err = Transport::<TextBody>::request(req, &config, Some(no_auth_option()))
        .await
        .expect_err("invalid utf-8 must fail");
    assert!(
        err.to_string().contains("UTF-8") || err.to_string().to_lowercase().contains("utf"),
        "got: {err}"
    );
}

// ---------------------------------------------------------------------------
// Typed-request entry: Transport::request_typed (#479 / parent #470)
// ---------------------------------------------------------------------------
//
// Canonical 入口焊合 `Transport::request` + `extract_response_data`：成功直接返回
// typed `T`，失败经 `map_context` 附着 operation / resource / request_id。
// 下列测试复用上方 5 种 ResponseFormat fixture，断言每种格式下入口均返回 typed 结果，
// 外加一条业务错误失败路径断言 context 附着。

#[tokio::test]
async fn request_typed_data_success_returns_typed() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/typed/data"))
        .respond_with(ResponseTemplate::new(200).set_body_json(success_envelope(
            serde_json::json!({"id": 11, "name": "typed-data"}),
        )))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/typed/data");
    let data =
        Transport::<ContractData>::request_typed(req, &config, Some(no_auth_option()), "测试-Data")
            .await
            .expect("Data format should decode to typed T");
    assert_eq!(data.id, 11);
    assert_eq!(data.name, "typed-data");
}

#[tokio::test]
async fn request_typed_flatten_success_returns_typed() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/typed/flatten"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": 0,
            "msg": "ok",
            "app_access_token": "tok-typed",
            "expire": 7200
        })))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/typed/flatten");
    let data = Transport::<FlattenToken>::request_typed(
        req,
        &config,
        Some(no_auth_option()),
        "测试-Flatten",
    )
    .await
    .expect("Flatten format should decode to typed T");
    assert_eq!(data.app_access_token, "tok-typed");
    assert_eq!(data.expires_in, 7200);
}

#[tokio::test]
async fn request_typed_text_success_returns_typed() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/typed/text"))
        .respond_with(ResponseTemplate::new(200).set_body_string("typed-text-body"))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/typed/text");
    let data =
        Transport::<TextBody>::request_typed(req, &config, Some(no_auth_option()), "测试-Text")
            .await
            .expect("Text format should decode to typed T");
    assert_eq!(data.0, "typed-text-body");
}

#[tokio::test]
async fn request_typed_binary_success_returns_typed() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/typed/binary"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-disposition", "attachment; filename=\"typed.bin\"")
                .set_body_bytes(b"TYPED-BIN".as_slice()),
        )
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/typed/binary");
    let data =
        Transport::<BinaryFile>::request_typed(req, &config, Some(no_auth_option()), "测试-Binary")
            .await
            .expect("Binary format should decode to typed T");
    assert_eq!(data.file_name, "typed.bin");
    assert_eq!(data.content, b"TYPED-BIN");
}

#[tokio::test]
async fn request_typed_custom_success_returns_typed() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/typed/custom"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"typed-custom-bytes".as_slice()))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/typed/custom");
    let data =
        Transport::<CustomRaw>::request_typed(req, &config, Some(no_auth_option()), "测试-Custom")
            .await
            .expect("Custom format should decode to typed T");
    assert_eq!(data.0, b"typed-custom-bytes");
}

/// 失败路径：业务错误 envelope（code≠0，data=null）→ `Transport::request` 仍返回
/// `Ok(Response{data:None})`，`request_typed` 经 `extract_response_data` 抽取失败，
/// 错误须附着 `operation` + `resource`(=context) + `request_id`（来自 envelope）。
#[tokio::test]
async fn request_typed_failure_attaches_operation_resource_request_id() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contract/typed/fail"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": 99991663,
            "msg": "permission denied",
            "request_id": "rid-typed-fail",
            "data": null
        })))
        .expect(1)
        .mount(&server)
        .await;

    let config = contract_config(&server.uri());
    let req = get_req("/open-apis/contract/typed/fail");

    let err =
        Transport::<ContractData>::request_typed(req, &config, Some(no_auth_option()), "文本翻译")
            .await
            .expect_err("business error envelope must surface as extraction failure");

    // 经 canonical helper 的 map_context 附着的三件诊断上下文
    let ctx = err.ctx();
    assert_eq!(
        ctx.operation(),
        Some("extract_response_data"),
        "operation should identify the canonical extraction step"
    );
    assert_eq!(
        ctx.get_context("resource"),
        Some("文本翻译"),
        "resource should carry caller-provided context"
    );
    assert_eq!(
        ctx.request_id(),
        Some("rid-typed-fail"),
        "request_id from envelope must be attached for server-side reconciliation"
    );

    // Option C (#485)：业务错误（code≠0）保留飞书 msg，返回 Api 错误（非 Validation）；
    // 此前 canonical 路径把业务错误当"data 缺失"丢 msg。
    match &err {
        CoreError::Api(api) => {
            assert!(
                api.message.contains("permission denied"),
                "business envelope msg must be preserved: {}",
                api.message
            );
        }
        other => panic!("expected CoreError::Api for business error, got: {other:?}"),
    }
}
