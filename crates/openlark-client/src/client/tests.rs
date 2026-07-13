use super::{Client, ClientBuilder, ClientErrorHandling};
use crate::Result;
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
    let config = openlark_core::config::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://open.feishu.cn")
        .build();

    let client = Client::with_core_config(config).unwrap();
    assert_eq!(client.config().app_id(), "test_app_id");
    assert_eq!(client.config().app_secret(), "test_app_secret");
    assert!(client.is_configured());
}

#[test]
fn test_client_not_configured() {
    let config = openlark_core::config::Config::builder().build();

    let client_result = Client::with_core_config(config);
    assert!(client_result.is_err());

    if let Err(error) = client_result {
        assert!(error.is_config_error() || error.is_validation_error());
        assert!(!error.user_message().unwrap_or("未知错误").is_empty());
    }
}

// ===== #416: unified with_checked_core_config seam =====

/// `Config::validate` 经 `validation_builder` 时 field 在 `CoreError::Validation` 变体上；
/// `validation_error()` 还会写入 context。两种路径都接受。
fn validation_field(err: &openlark_core::error::CoreError) -> Option<&str> {
    match err {
        openlark_core::error::CoreError::Validation { field, .. } => Some(field.as_ref()),
        _ => err.context().get_context("field"),
    }
}

#[test]
fn test_with_core_config_rejects_custom_host_without_flag() {
    let config = openlark_core::config::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://proxy.example.com")
        .build();
    let err = Client::with_core_config(config).unwrap_err();
    assert!(err.is_validation_error());
    assert_eq!(validation_field(&err), Some("base_url"));
    assert_eq!(
        err.context().get_context("operation"),
        Some("Client::with_core_config")
    );
    let msg = format!("{err}");
    assert!(
        msg.contains("allow_custom_base_url") || msg.contains("白名单"),
        "with_core_config 必须执行白名单: {msg}"
    );
}

#[test]
fn test_with_core_config_allows_custom_host_with_flag() {
    let config = openlark_core::config::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://proxy.example.com")
        .allow_custom_base_url(true)
        .build();
    let client = Client::with_core_config(config).unwrap();
    assert!(client.config().allow_custom_base_url());
    assert_eq!(client.config().base_url(), "https://proxy.example.com");
}

#[test]
fn test_with_core_config_accepts_unset_timeout() {
    let config = openlark_core::config::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://open.feishu.cn")
        .build();
    assert!(config.req_timeout().is_none());
    let client = Client::with_core_config(config).unwrap();
    assert!(client.config().req_timeout().is_none());
}

#[test]
fn test_with_core_config_rejects_zero_timeout() {
    let config = openlark_core::config::Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://open.feishu.cn")
        .req_timeout(Duration::ZERO)
        .build();
    let err = Client::with_core_config(config).unwrap_err();
    assert!(err.is_validation_error());
    assert_eq!(validation_field(&err), Some("timeout"));
    assert_eq!(
        err.context().get_context("operation"),
        Some("Client::with_core_config")
    );
}

#[test]
fn test_client_builder_rejects_zero_timeout() {
    let err = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .timeout(Duration::ZERO)
        .build()
        .unwrap_err();
    assert!(err.is_validation_error());
    assert_eq!(validation_field(&err), Some("timeout"));
    assert_eq!(
        err.context().get_context("operation"),
        Some("ClientBuilder::build")
    );
}

#[test]
fn test_client_builder_validation_error_has_operation_context() {
    let err = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://proxy.example.com")
        .build()
        .unwrap_err();
    assert!(err.is_validation_error());
    assert_eq!(validation_field(&err), Some("base_url"));
    assert_eq!(
        err.context().get_context("operation"),
        Some("ClientBuilder::build")
    );
}

#[test]
fn test_with_core_config_empty_app_id_field_and_operation() {
    let config = openlark_core::config::Config::builder()
        .app_secret("secret")
        .base_url("https://open.feishu.cn")
        .build();
    let err = Client::with_core_config(config).unwrap_err();
    assert!(err.is_validation_error());
    assert_eq!(validation_field(&err), Some("app_id"));
    assert_eq!(
        err.context().get_context("operation"),
        Some("Client::with_core_config")
    );
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
    let _ = aily.agent().agent_chat_session().create("agent_123");
    let _ = aily
        .agent()
        .agent_chat_session()
        .get("agent_123", "session_123");
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
    let _ = okr_v2
        .cycle()
        .list("user_123")
        .user_id_type("open_id")
        .page_size(20);
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

    let task = client.workflow.v2().task();

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
    let _ = task
        .search()
        .page_size(20)
        .page_token("token_abc")
        .user_id_type("open_id");

    let _ = client
        .workflow
        .v2()
        .tasklist()
        .search()
        .page_size(20)
        .page_token("token_abc")
        .user_id_type("open_id");
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
fn test_builder_default_requires_credentials() {
    // 空 builder 构建失败（经公开 interface 观察，不读私有字段）
    let result = ClientBuilder::default().build();
    assert!(result.is_err());
}

// ===== #415: ClientBuilder delegates to core ConfigBuilder =====

#[test]
fn test_client_builder_default_timeout_is_30s() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();
    assert_eq!(
        client.config().req_timeout(),
        Some(Duration::from_secs(30)),
        "ClientBuilder 默认超时应为 30 秒"
    );
}

#[test]
fn test_client_builder_allow_custom_base_url_propagates() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://proxy.example.com")
        .allow_custom_base_url(true)
        .build()
        .unwrap();
    assert!(client.config().allow_custom_base_url());
    assert_eq!(client.config().base_url(), "https://proxy.example.com");
}

#[test]
fn test_client_builder_custom_host_rejected_without_flag() {
    let err = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("https://proxy.example.com")
        .build()
        .unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("allow_custom_base_url") || msg.contains("白名单"),
        "未放行自定义域名应失败: {msg}"
    );
}

#[test]
fn test_client_builder_from_env_then_setter_overrides() {
    crate::test_utils::with_env_vars(
        &[
            ("OPENLARK_APP_ID", Some("env_app")),
            ("OPENLARK_APP_SECRET", Some("env_secret")),
            ("OPENLARK_TIMEOUT", Some("90")),
        ],
        || {
            let client = Client::builder()
                .from_env()
                .app_id("code_app")
                .timeout(Duration::from_secs(12))
                .build()
                .unwrap();
            assert_eq!(client.config().app_id(), "code_app");
            assert_eq!(client.config().app_secret(), "env_secret");
            assert_eq!(client.config().req_timeout(), Some(Duration::from_secs(12)));
        },
    );
}

#[test]
fn test_client_builder_setter_then_from_env_overrides() {
    crate::test_utils::with_env_vars(
        &[
            ("OPENLARK_APP_ID", Some("env_app")),
            ("OPENLARK_APP_SECRET", Some("env_secret")),
            ("OPENLARK_TIMEOUT", Some("90")),
        ],
        || {
            let client = Client::builder()
                .app_id("code_app")
                .app_secret("code_secret")
                .timeout(Duration::from_secs(12))
                .from_env()
                .build()
                .unwrap();
            assert_eq!(client.config().app_id(), "env_app");
            assert_eq!(client.config().app_secret(), "env_secret");
            assert_eq!(client.config().req_timeout(), Some(Duration::from_secs(90)));
        },
    );
}

#[test]
fn test_client_builder_header_incremental_and_last_write_wins() {
    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .add_header("X-A", "1")
        .add_header("X-B", "2")
        .add_header("X-A", "3")
        .build()
        .unwrap();
    assert_eq!(client.config().header().get("X-A"), Some(&"3".to_string()));
    assert_eq!(client.config().header().get("X-B"), Some(&"2".to_string()));
}

#[test]
fn test_client_builder_preserves_all_options_through_config() {
    use openlark_core::constants::AppType;

    let client = Client::builder()
        .app_id("opt_app")
        .app_secret("opt_secret")
        .app_type(AppType::Marketplace)
        .enable_token_cache(false)
        .base_url("https://open.larksuite.com")
        .timeout(Duration::from_secs(55))
        .retry_count(5)
        .enable_log(false)
        .max_response_size(12345)
        .add_header("X-Custom", "v")
        .allow_custom_base_url(false)
        .build()
        .unwrap();

    let c = client.config();
    assert_eq!(c.app_id(), "opt_app");
    assert_eq!(c.app_secret(), "opt_secret");
    assert_eq!(c.app_type(), AppType::Marketplace);
    assert!(!c.enable_token_cache());
    assert_eq!(c.base_url(), "https://open.larksuite.com");
    assert_eq!(c.req_timeout(), Some(Duration::from_secs(55)));
    assert_eq!(c.retry_count(), 5);
    assert!(!c.enable_log());
    assert_eq!(c.max_response_size(), 12345);
    assert_eq!(c.header().get("X-Custom"), Some(&"v".to_string()));
    assert!(!c.allow_custom_base_url());
}

#[test]
fn test_client_builder_clone_and_redacted_debug() {
    let builder = Client::builder()
        .app_id("debug_app")
        .app_secret("super-secret-value")
        .add_header("Authorization", "Bearer secret-token");
    let cloned = builder.clone();
    let debug = format!("{builder:?}");
    assert!(
        debug.starts_with("ClientBuilder"),
        "Debug 应以 ClientBuilder 开头: {debug}"
    );
    assert!(
        !debug.contains("super-secret-value"),
        "Debug 不得泄露 app_secret: {debug}"
    );
    assert!(
        !debug.contains("Bearer secret-token"),
        "Debug 不得泄露 header 值: {debug}"
    );

    let client = cloned.build().unwrap();
    assert_eq!(client.config().app_id(), "debug_app");
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

// ===== #434: compiled-capability catalog bot tracer =====
// 最高运行时 seam：`Client::registry()` 与 Client 字段对 bot feature 的一致性。

#[test]
fn bot_capability_client_and_registry_agree() {
    use crate::registry::ServiceRegistry;

    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    #[cfg(feature = "bot")]
    {
        // Client 字段可用
        let _bot = &client.bot;
        // registry 报告 bot，且元数据完整
        assert!(
            client.registry().has_service("bot"),
            "bot feature 启用时 registry 必须报告 bot"
        );
        let entry = client.registry().get_service("bot").unwrap();
        assert_eq!(entry.metadata.name, "bot");
        assert_eq!(
            entry.metadata.description.as_deref(),
            Some("飞书机器人服务，提供机器人搜索等功能")
        );
        assert_eq!(entry.metadata.dependencies, vec!["auth".to_string()]);
        assert_eq!(entry.metadata.provides, vec!["bot".to_string()]);
        assert_eq!(entry.metadata.priority, 4);
        // metadata-only：无 runtime instance
        assert!(entry.instance.is_none());
    }

    #[cfg(not(feature = "bot"))]
    {
        assert!(
            !client.registry().has_service("bot"),
            "bot feature 禁用时 registry 不得报告 bot"
        );
    }
}

#[test]
fn legacy_domains_still_register_via_old_path() {
    use crate::registry::ServiceRegistry;

    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    // 默认 feature 含 auth；确认旧路径未被 capability catalog 破坏
    #[cfg(feature = "auth")]
    assert!(client.registry().has_service("auth"));

    #[cfg(feature = "communication")]
    assert!(client.registry().has_service("communication"));

    #[cfg(not(feature = "hr"))]
    assert!(!client.registry().has_service("hr"));
}
