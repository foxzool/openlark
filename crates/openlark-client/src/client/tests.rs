#![allow(deprecated)]

use super::{Client, ClientBuilder, ClientErrorHandling};
use crate::{Config, Result};
use openlark_core::error::ErrorTrait;
use std::time::Duration;

#[test]
fn test_client_builder() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .timeout(Duration::from_secs(30))
        .build();

    assert!(client.is_ok());
}

#[test]
fn test_client_builder_populates_core_config_options() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .timeout(Duration::from_secs(45))
        .retry_count(4)
        .enable_log(false)
        .max_response_size(8 * 1024 * 1024)
        .add_header("X-Test", "true")
        .build()
        .unwrap();

    assert_eq!(client.config().req_timeout(), Some(Duration::from_secs(45)));
    assert_eq!(client.config().retry_count(), 4);
    assert!(!client.config().enable_log());
    assert_eq!(client.config().max_response_size(), 8 * 1024 * 1024);
    assert_eq!(
        client.config().header().get("X-Test"),
        Some(&"true".to_string())
    );
}

#[test]
fn test_client_with_core_config() {
    let core_config = openlark_core::config::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://open.feishu.cn")
        .retry_count(2)
        .build();

    let client = Client::with_core_config(core_config).unwrap();

    assert_eq!(client.config().app_id(), "test_app_id");
    assert_eq!(client.config().app_secret(), "test_app_secret");
    assert_eq!(client.config().retry_count(), 2);
}

#[test]
fn test_client_config() {
    let config = Config {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
        base_url: "https://open.feishu.cn".to_string(),
        ..Default::default()
    };

    let client = Client::with_config(config).unwrap();
    assert_eq!(client.config().app_id(), "test_app_id");
    assert_eq!(client.config().app_secret(), "test_app_secret");
    assert!(client.is_configured());
}

#[test]
fn test_client_not_configured() {
    let config = Config {
        app_id: String::new(),
        app_secret: String::new(),
        ..Default::default()
    };

    let client_result = Client::with_config(config);
    assert!(client_result.is_err());

    if let Err(error) = client_result {
        assert!(error.is_config_error() || error.is_validation_error());
        assert!(!error.user_message().unwrap_or("未知错误").is_empty());
    }
}

#[test]
fn test_client_clone() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    let cloned_client = client.clone();
    assert_eq!(client.config().app_id(), cloned_client.config().app_id());
}

#[cfg(feature = "cardkit")]
#[test]
fn test_cardkit_chain_exists() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    let _ = &client.cardkit.v1.card;
}

#[cfg(feature = "docs")]
#[test]
fn test_docs_chain_exists() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    let _ = client.docs.config();
}

#[cfg(feature = "communication")]
#[test]
fn test_communication_chain_exists() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    let _ = client.communication.config();
}

#[cfg(feature = "meeting")]
#[test]
fn test_meeting_chain_exists() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    let _ = client.meeting.config();
}

#[test]
fn test_client_error_handling() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    let error_result: Result<i32> =
        Err(crate::error::validation_error("field", "validation failed"));
    let result = client.handle_error(error_result, "test_operation");

    assert!(result.is_err());
    if let Err(error) = result {
        assert!(error.context().has_context("operation"));
        assert_eq!(
            error.context().get_context("operation"),
            Some("test_operation")
        );
        assert_eq!(error.context().get_context("component"), Some("Client"));
    }
}

#[tokio::test]
async fn test_async_error_handling() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    let result = client
        .handle_async_error(
            async { Err::<i32, _>(crate::error::network_error("async error")) },
            "async_test",
        )
        .await;

    assert!(result.is_err());
    if let Err(error) = result {
        assert!(error.context().has_context("operation"));
        assert_eq!(error.context().get_context("operation"), Some("async_test"));
        assert_eq!(error.context().get_context("component"), Some("Client"));
    }
}

#[test]
fn test_from_env_missing_vars() {
    let builder = ClientBuilder::default();
    let result = builder.build();
    assert!(result.is_err());
}

#[test]
fn test_from_app_id_string() {
    crate::test_utils::with_env_vars(
        &[
            ("OPENLARK_APP_ID", Some("test_app_id")),
            ("OPENLARK_APP_SECRET", Some("test_secret")),
        ],
        || {
            let result: Result<Client> = Client::from_env();
            assert!(result.is_ok());

            if let Ok(client) = result {
                assert_eq!(client.config().app_id(), "test_app_id");
                assert_eq!(client.config().app_secret(), "test_secret");
            }
        },
    );
}

#[test]
fn test_builder_default() {
    let builder = ClientBuilder::default();
    assert!(builder.config.app_id.is_empty());
    assert!(builder.config.app_secret.is_empty());
}

#[cfg(feature = "communication")]
#[test]
fn test_communication_service_access() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    let _comm = &client.communication;
}
