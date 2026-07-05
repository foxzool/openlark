//! Representative contract tests for openlark-user request/response models.
//!
//! Tests cover:
//! - `common::UserSetting` / `UserPreference` — core data models
//! - UserService / PersonalSettingsResource / SystemStatusResource — service access
//! - Version contract

use openlark_user::common::{UserPreference, UserSetting};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, from_value, json, to_value};

fn assert_json_contract<T>(value: &T, expected: Value)
where
    T: Serialize,
{
    assert_eq!(to_value(value).unwrap(), expected);
}

fn parse_contract<T>(payload: Value) -> T
where
    T: DeserializeOwned,
{
    from_value(payload).unwrap()
}

// ---------------------------------------------------------------------------
// common models — UserSetting
// ---------------------------------------------------------------------------

#[test]
fn user_setting_roundtrip() {
    let setting = UserSetting {
        key: "language".to_string(),
        value: "zh-CN".to_string(),
        setting_type: "ui".to_string(),
    };
    let expected = json!({
        "key": "language",
        "value": "zh-CN",
        "setting_type": "ui"
    });
    assert_json_contract(&setting, expected.clone());

    let back: UserSetting = parse_contract(expected);
    assert_eq!(back.key, "language");
    assert_eq!(back.value, "zh-CN");
    assert_eq!(back.setting_type, "ui");
}

#[test]
fn user_setting_notification_type() {
    let setting = UserSetting {
        key: "email_notify".to_string(),
        value: "true".to_string(),
        setting_type: "notification".to_string(),
    };
    assert_json_contract(
        &setting,
        json!({
            "key": "email_notify",
            "value": "true",
            "setting_type": "notification"
        }),
    );

    let back: UserSetting = parse_contract(json!({
        "key": "push_enabled",
        "value": "false",
        "setting_type": "notification"
    }));
    assert_eq!(back.setting_type, "notification");
    assert_eq!(back.value, "false");
}

#[test]
fn user_setting_privacy_type() {
    let setting = UserSetting {
        key: "profile_visible".to_string(),
        value: "true".to_string(),
        setting_type: "privacy".to_string(),
    };
    assert_json_contract(
        &setting,
        json!({
            "key": "profile_visible",
            "value": "true",
            "setting_type": "privacy"
        }),
    );
}

// ---------------------------------------------------------------------------
// common models — UserPreference
// ---------------------------------------------------------------------------

#[test]
fn user_preference_with_category_roundtrip() {
    let pref = UserPreference {
        key: "default_calendar_view".to_string(),
        value: "week".to_string(),
        category: Some("calendar".to_string()),
    };
    assert_json_contract(
        &pref,
        json!({
            "key": "default_calendar_view",
            "value": "week",
            "category": "calendar"
        }),
    );
}

#[test]
fn user_preference_without_category_roundtrip() {
    let parsed: UserPreference = parse_contract(json!({
        "key": "time_format",
        "value": "24h",
        "category": null
    }));
    assert_eq!(parsed.key, "time_format");
    assert!(parsed.category.is_none());
    assert_json_contract(
        &parsed,
        json!({
            "key": "time_format",
            "value": "24h",
            "category": null
        }),
    );
}

#[test]
fn user_preference_multiple_categories() {
    let shortcuts: UserPreference = parse_contract(json!({
        "key": "shortcut_paste",
        "value": "Ctrl+V",
        "category": "shortcuts"
    }));
    assert_eq!(shortcuts.category.as_deref(), Some("shortcuts"));

    let display: UserPreference = parse_contract(json!({
        "key": "font_size",
        "value": "14",
        "category": "display"
    }));
    assert_eq!(display.category.as_deref(), Some("display"));
    assert_json_contract(
        &display,
        json!({
            "key": "font_size",
            "value": "14",
            "category": "display"
        }),
    );
}

// ---------------------------------------------------------------------------
// UserService creation contract
// ---------------------------------------------------------------------------

#[test]
fn user_service_creation_contract() {
    use openlark_core::config::Config;
    use openlark_user::UserService;

    let config = Config::builder()
        .app_id("cli_test_app")
        .app_secret("test_secret")
        .build();

    let service = UserService::new(config);
    assert!(service.is_ok());
}

#[test]
fn user_service_config_roundtrip() {
    use openlark_core::config::Config;
    use openlark_user::UserService;

    let config = Config::builder()
        .app_id("cli_config_test")
        .app_secret("secret_value")
        .build();

    let service = UserService::new(config).unwrap();
    let config_arc = service.config();
    assert_eq!(config_arc.app_id(), "cli_config_test");
}

// ---------------------------------------------------------------------------
// Personal settings service access (system_status facade accessor)
// ---------------------------------------------------------------------------

#[test]
fn personal_settings_service_access_contract() {
    use openlark_core::config::Config;
    use openlark_user::UserService;

    let config = Config::builder()
        .app_id("cli_ps_test")
        .app_secret("secret")
        .build();
    let service = UserService::new(config).unwrap();

    // 门面 accessor 缩短 system_status 路径
    let ps = service.personal_settings();
    assert_eq!(ps.config().app_id(), "cli_ps_test");

    // system_status 资源 7 个真实构建器可达
    let system_status = ps.system_status();
    let _list = system_status.list();
    let _get = system_status.get();
    let _create = system_status.create();
    let _patch = system_status.patch();
    let _delete = system_status.delete();
    let _batch_open = system_status.batch_open("status_x");
    let _batch_close = system_status.batch_close("status_y");
}

// ---------------------------------------------------------------------------
// Version contract
// ---------------------------------------------------------------------------

#[test]
fn version_is_not_empty() {
    let version = openlark_user::VERSION;
    assert!(!version.is_empty());
    assert_eq!(version, env!("CARGO_PKG_VERSION"));
}
