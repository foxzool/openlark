//! openlark-webhook 集成测试。
#![cfg(feature = "robot")]

use openlark_webhook::common::error::WebhookError;
use openlark_webhook::robot::v1::client::WebhookClient;
use openlark_webhook::robot::v1::send::SendWebhookMessageRequest;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

fn webhook_url(server: &MockServer) -> String {
    format!("{}/webhook", server.uri())
}

async fn mock_success(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code": 0, "msg": "ok"})))
        .mount(server)
        .await;
}

async fn mock_error(server: &MockServer, status: u16, code: i32, msg: &str) {
    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(
            ResponseTemplate::new(status).set_body_json(json!({"code": code, "msg": msg})),
        )
        .mount(server)
        .await;
}

#[tokio::test]
async fn test_send_text_message_success() {
    let server = MockServer::start().await;
    mock_success(&server).await;

    let resp = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("hello webhook".to_string())
        .execute()
        .await
        .expect("text message should be sent successfully");

    assert_eq!(resp.code, 0);
    assert_eq!(resp.msg, "ok");
}

#[tokio::test]
async fn test_send_all_message_types() {
    let server = MockServer::start().await;
    mock_success(&server).await;
    let url = webhook_url(&server);

    SendWebhookMessageRequest::new(url.clone())
        .text("type-text".to_string())
        .execute()
        .await
        .expect("text should succeed");

    SendWebhookMessageRequest::new(url.clone())
        .post(
            "{\"zh_cn\":{\"title\":\"t\",\"content\":[[{\"tag\":\"text\",\"text\":\"body\"}]]}}"
                .to_string(),
        )
        .execute()
        .await
        .expect("post should succeed");

    SendWebhookMessageRequest::new(url.clone())
        .image("img_v2_test_key".to_string())
        .execute()
        .await
        .expect("image should succeed");

    SendWebhookMessageRequest::new(url)
        .file("file_v2_test_key".to_string())
        .execute()
        .await
        .expect("file should succeed");
}

#[cfg(feature = "card")]
#[tokio::test]
async fn test_send_interactive_card() {
    let server = MockServer::start().await;
    mock_success(&server).await;

    let _ = SendWebhookMessageRequest::new(webhook_url(&server))
        .card(json!({
            "config": { "wide_screen_mode": true },
            "header": { "title": { "tag": "plain_text", "content": "Integration" } },
            "elements": [{ "tag": "div", "text": { "tag": "lark_md", "content": "card body" } }]
        }))
        .execute()
        .await
        .expect("card should succeed");
}

#[tokio::test]
async fn test_error_400() {
    let server = MockServer::start().await;
    mock_error(&server, 400, 19001, "invalid request").await;

    let err = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("test".to_string())
        .execute()
        .await
        .expect_err("should fail");

    match err {
        WebhookError::Http(message) => assert!(message.contains("400")),
        _ => panic!("expected Http error"),
    }
}

#[tokio::test]
async fn test_error_429() {
    let server = MockServer::start().await;
    mock_error(&server, 429, 19020, "rate limit").await;

    let err = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("test".to_string())
        .execute()
        .await
        .expect_err("should fail");

    match err {
        WebhookError::Http(message) => assert!(message.contains("429")),
        _ => panic!("expected Http error"),
    }
}

#[tokio::test]
async fn test_error_500() {
    let server = MockServer::start().await;
    mock_error(&server, 500, 20000, "internal error").await;

    let err = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("test".to_string())
        .execute()
        .await
        .expect_err("should fail");

    match err {
        WebhookError::Http(message) => assert!(message.contains("500")),
        _ => panic!("expected Http error"),
    }
}

#[tokio::test]
async fn test_response_parsing_success() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"code": 12345, "msg": "accepted"})),
        )
        .mount(&server)
        .await;

    let resp = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("test".to_string())
        .execute()
        .await
        .expect("should succeed");

    assert_eq!(resp.code, 12345);
    assert_eq!(resp.msg, "accepted");
}

#[tokio::test]
async fn test_response_parsing_failure() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .mount(&server)
        .await;

    let err = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("test".to_string())
        .execute()
        .await
        .expect_err("should fail");

    match err {
        WebhookError::Serialization(_) => {}
        _ => panic!("expected Serialization error"),
    }
}

#[cfg(feature = "signature")]
#[tokio::test]
async fn test_send_with_signature_headers() {
    let server = MockServer::start().await;

    fn has_signature_headers(req: &Request) -> bool {
        let has_signature = req
            .headers
            .get("X-Lark-Signature")
            .map(|v| !v.as_bytes().is_empty())
            .unwrap_or(false);
        let has_timestamp = req
            .headers
            .get("X-Lark-Timestamp")
            .map(|v| !v.as_bytes().is_empty())
            .unwrap_or(false);
        has_signature && has_timestamp
    }

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .and(has_signature_headers)
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code": 0, "msg": "ok"})))
        .mount(&server)
        .await;

    let resp = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("signed message".to_string())
        .with_secret("test-secret".to_string())
        .execute()
        .await
        .expect("signed message should be sent successfully");

    assert_eq!(resp.code, 0);
    assert_eq!(resp.msg, "ok");
}

#[cfg(feature = "signature")]
#[tokio::test]
async fn test_client_send_with_signature_headers() {
    let server = MockServer::start().await;

    fn has_signature_headers(req: &Request) -> bool {
        let has_signature = req
            .headers
            .get("X-Lark-Signature")
            .map(|v| !v.as_bytes().is_empty())
            .unwrap_or(false);
        let has_timestamp = req
            .headers
            .get("X-Lark-Timestamp")
            .map(|v| !v.as_bytes().is_empty())
            .unwrap_or(false);
        has_signature && has_timestamp
    }

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .and(has_signature_headers)
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code": 0, "msg": "ok"})))
        .mount(&server)
        .await;

    let client = WebhookClient::new().with_secret("test-secret".to_string());

    let resp = client
        .send_text(&webhook_url(&server), "client signed message".to_string())
        .await
        .expect("client signed message should be sent successfully");

    assert_eq!(resp.code, 0);
    assert_eq!(resp.msg, "ok");
}

#[tokio::test]
async fn test_send_without_signature_no_headers() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code": 0, "msg": "ok"})))
        .mount(&server)
        .await;

    let resp = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("unsigned message".to_string())
        .execute()
        .await
        .expect("unsigned message should be sent successfully");

    assert_eq!(resp.code, 0);
}
