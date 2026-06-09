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

#[cfg(feature = "communication")]
#[test]
fn test_communication_aily_chain_exists() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    let aily = &client.communication.aily;

    let _ = aily.aily_session().create();
    let _ = aily.aily_session().delete("session_123");
    let _ = aily.aily_session().get("session_123");
    let _ = aily.aily_session().update("session_123");

    let _ = aily.aily_session().aily_message().create("session_123");
    let _ = aily
        .aily_session()
        .aily_message()
        .get("session_123", "message_123");
    let _ = aily.aily_session().aily_message().list("session_123");

    let _ = aily.aily_session().run().cancel("session_123", "run_123");
    let _ = aily.aily_session().run().create("session_123");
    let _ = aily.aily_session().run().get("session_123", "run_123");
    let _ = aily.aily_session().run().list("session_123");

    let _ = aily.app().data_asset().create("app_123");
    let _ = aily.app().data_asset().delete("app_123", "asset_123");
    let _ = aily.app().data_asset().get("app_123", "asset_123");
    let _ = aily.app().data_asset().list("app_123");
    let _ = aily.app().data_asset().upload_file("app_123");
    let _ = aily.app().data_asset_tag().list("app_123");
    let _ = aily.app().knowledge().ask("app_123");
    let _ = aily.app().skill().get("app_123", "skill_123");
    let _ = aily.app().skill().list("app_123");
    let _ = aily.app().skill().start("app_123", "skill_123");

    let _ = aily
        .agent()
        .agent_artifact()
        .get("agent_123", "artifact_123");
    let _ = aily.agent().agent_attachment().create("agent_123");
    let _ = aily.agent().agent_chat().create("agent_123");
    let _ = aily.agent().agent_chat().get("agent_123", "chat_123");
    let _ = aily.agent().agent_visibility().check("agent_123");

    let _ = aily.tenant().app_stat().list();
    let _ = client.communication.aily.config();
}

#[cfg(feature = "hr")]
#[test]
fn test_hr_okr_v2_chain_exists() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    let okr_v2 = client.hr.okr.v2();

    let _ = okr_v2.alignment().get("alignment_123");
    let _ = okr_v2.alignment().delete("alignment_123");
    let _ = okr_v2.category().list();
    let _ = okr_v2.cycle().list();
    let _ = okr_v2.cycle().objectives_position("cycle_123");
    let _ = okr_v2.cycle().objectives_weight("cycle_123");
    let _ = okr_v2.cycle().create_objective("cycle_123");
    let _ = okr_v2.cycle().list_objectives("cycle_123");
    let _ = okr_v2.indicator().patch("indicator_123");
    let _ = okr_v2.key_result().get("kr_123");
    let _ = okr_v2.key_result().delete("kr_123");
    let _ = okr_v2.key_result().patch("kr_123");
    let _ = okr_v2.key_result().list_indicators("kr_123");
    let _ = okr_v2.key_result().list_progresses("kr_123");
    let _ = okr_v2.objective().get("obj_123");
    let _ = okr_v2.objective().delete("obj_123");
    let _ = okr_v2.objective().patch("obj_123");
    let _ = okr_v2.objective().key_results_position("obj_123");
    let _ = okr_v2.objective().key_results_weight("obj_123");
    let _ = okr_v2.objective().create_alignment("obj_123");
    let _ = okr_v2.objective().list_alignments("obj_123");
    let _ = okr_v2.objective().list_indicators("obj_123");
    let _ = okr_v2.objective().create_key_result("obj_123");
    let _ = okr_v2.objective().list_key_results("obj_123");
    let _ = okr_v2.objective().list_progresses("obj_123");
    let _ = okr_v2.config();
}

#[cfg(feature = "meeting")]
#[test]
fn test_meeting_vc_note_chain_exists() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    // Verify VC note chain access compiles
    let _ = client.meeting.vc.v1.note.get("note_123");
    let _ = client.meeting.vc.v1.note.subscribe();
    let _ = client.meeting.vc.v1.note.unsubscribe();
    let _ = client.meeting.vc.v1.note.config();
}

#[cfg(feature = "workflow")]
#[test]
fn test_workflow_task_v2_chain_exists() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    let task = client.workflow.task();

    let _ = task.create();
    let _ = task.update("task_123");
    let _ = task.delete("task_123");
    let _ = task.get("task_123");
    let _ = task.list();
    let _ = task.complete("task_123");
    let _ = task.uncomplete("task_123");
    let _ = task.subtask("task_123");
    let _ = task.add_tasklist("task_123");
    let _ = task.remove_tasklist("task_123");
    let _ = task.tasklists("task_123");
    let _ = task.add_members("task_123");
    let _ = task.remove_members("task_123");
    let _ = task.add_reminders("task_123");
    let _ = task.remove_reminders("task_123");
    let _ = task.add_dependencies("task_123");
    let _ = task.remove_dependencies("task_123");
    let _ = task.set_ancestor_task("task_123");
    let _ = task.search();

    let _ = client.workflow.tasklist().search();
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
