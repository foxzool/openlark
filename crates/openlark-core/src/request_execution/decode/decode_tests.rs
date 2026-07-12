//! ResponseDecoder 单元测试（精简：重量级路径由 transport_request_contract 覆盖）。

use super::*;
use crate::api::ResponseFormat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
struct SampleData {
    id: i32,
    name: String,
}

impl ApiResponseTrait for SampleData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
struct EmptyOk;

impl ApiResponseTrait for EmptyOk {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    fn empty_success() -> Option<Self> {
        Some(Self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BinFile {
    name: String,
    body: Vec<u8>,
}

impl ApiResponseTrait for BinFile {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Binary
    }

    fn from_binary(file_name: String, body: Vec<u8>) -> Option<Self> {
        Some(Self {
            name: file_name,
            body,
        })
    }
}

impl<'de> Deserialize<'de> for BinFile {
    fn deserialize<D: serde::Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom("binary only"))
    }
}

#[test]
fn resolve_missing_data_prefers_empty_success() {
    let got = resolve_missing_data_field::<EmptyOk>().expect("ok");
    assert_eq!(got, Some(EmptyOk));
}

#[test]
fn resolve_missing_data_errors_when_required() {
    let err = resolve_missing_data_field::<SampleData>().expect_err("err");
    assert!(err.contains("data"));
}

#[test]
fn resolve_missing_data_allows_unit_without_payload() {
    let got = resolve_missing_data_field::<()>().expect("ok");
    assert!(got.is_none());
}

#[tokio::test]
async fn data_format_parses_envelope() {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": 0,
            "msg": "ok",
            "data": {"id": 1, "name": "n"}
        })))
        .mount(&server)
        .await;

    let resp = reqwest::Client::new()
        .get(server.uri())
        .send()
        .await
        .expect("send");
    let decoded = ResponseDecoder::handle_response::<SampleData>(resp, 1024 * 1024)
        .await
        .expect("decode");
    assert!(decoded.is_success());
    assert_eq!(
        decoded.data,
        Some(SampleData {
            id: 1,
            name: "n".into()
        })
    );
}

#[tokio::test]
async fn data_format_missing_required_is_err() {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": 0,
            "msg": "ok"
        })))
        .mount(&server)
        .await;

    let resp = reqwest::Client::new()
        .get(server.uri())
        .send()
        .await
        .expect("send");
    let err = ResponseDecoder::handle_response::<SampleData>(resp, 1024 * 1024)
        .await
        .expect_err("missing data");
    assert!(err.to_string().contains("data") || format!("{err:?}").contains("data"));
}

#[tokio::test]
async fn data_format_empty_success_without_data_field() {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": 0,
            "msg": "ok"
        })))
        .mount(&server)
        .await;

    let resp = reqwest::Client::new()
        .get(server.uri())
        .send()
        .await
        .expect("send");
    let decoded = ResponseDecoder::handle_response::<EmptyOk>(resp, 1024 * 1024)
        .await
        .expect("empty_success");
    assert_eq!(decoded.data, Some(EmptyOk));
}

#[tokio::test]
async fn binary_uses_from_binary_with_filename() {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-disposition", "attachment; filename=\"a.bin\"")
                .set_body_bytes(b"xyz".as_slice()),
        )
        .mount(&server)
        .await;

    let resp = reqwest::Client::new()
        .get(server.uri())
        .send()
        .await
        .expect("send");
    let decoded = ResponseDecoder::handle_response::<BinFile>(resp, 1024 * 1024)
        .await
        .expect("binary");
    let data = decoded.data.expect("data");
    assert_eq!(data.name, "a.bin");
    assert_eq!(data.body, b"xyz");
}
