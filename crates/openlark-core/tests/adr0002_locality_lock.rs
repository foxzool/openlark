//! ADR-0002 locality regression locks (#584).
//!
//! These are **structural** contracts (source ownership), not behavioral call-graph
//! snapshots. They fail if:
//! - token-type decision helpers are re-homed onto Transport (`http.rs`)
//! - app_ticket resend reintroduces an ad-hoc HTTP client path
//! - AuthHandler is resurrected under `request_execution/`
//! - recovery / resend escape hatches become public
//! - ADR-0002 status regresses away from Accepted without an explicit residual ticket
//!
//! Behavioral coverage remains in:
//! - `auth/app_ticket.rs` unit tests (condition matrix)
//! - `transport_request_contract::transport_app_ticket_invalid_triggers_resend`

use std::fs;
use std::path::{Path, PathBuf};

fn core_src() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// Strip `//` line comments and `/* ... */` blocks so string literals / comments
/// cannot falsely satisfy or defeat ownership asserts.
fn strip_rust_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let bytes = src.as_bytes();
    let mut i = 0;
    let mut in_block = false;
    while i < bytes.len() {
        if in_block {
            if bytes[i] == b'*' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
                in_block = false;
                i += 2;
                continue;
            }
            i += 1;
            continue;
        }
        if bytes[i] == b'/' && i + 1 < bytes.len() {
            if bytes[i + 1] == b'/' {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            if bytes[i + 1] == b'*' {
                in_block = true;
                i += 2;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn has_fn_def(src: &str, name: &str) -> bool {
    // Match `fn name(` / `async fn name(` / `pub(crate) fn name(` etc.
    let needle = format!("fn {name}(");
    src.contains(&needle)
}

#[test]
fn policy_entry_points_live_under_auth_policy() {
    let policy = strip_rust_comments(&read(&core_src().join("auth/policy.rs")));
    assert!(
        has_fn_def(&policy, "determine_token_type"),
        "determine_token_type must be defined in auth/policy.rs (ADR-0002)"
    );
    assert!(
        has_fn_def(&policy, "validate_token_type"),
        "validate_token_type must be defined in auth/policy.rs (ADR-0002)"
    );
    assert!(
        has_fn_def(&policy, "validate_authorization"),
        "validate_authorization must be defined in auth/policy.rs (ADR-0002)"
    );
}

#[test]
fn transport_does_not_redefine_policy_helpers() {
    let http = strip_rust_comments(&read(&core_src().join("http.rs")));
    assert!(
        !has_fn_def(&http, "determine_token_type"),
        "http.rs must not redefine determine_token_type; call auth::policy (ADR-0002)"
    );
    assert!(
        !has_fn_def(&http, "validate_token_type"),
        "http.rs must not redefine validate_token_type; call auth::policy (ADR-0002)"
    );
    assert!(
        !has_fn_def(&http, "validate_authorization"),
        "http.rs must not redefine validate_authorization; call auth::policy (ADR-0002)"
    );
    // α-delegate: Transport names the policy module, does not re-encode rules.
    assert!(
        http.contains("auth::policy::determine_token_type")
            || http.contains("crate::auth::policy::determine_token_type"),
        "Transport must delegate determine_token_type by name to auth::policy"
    );
    assert!(
        http.contains("auth::policy::validate_token_type")
            || http.contains("crate::auth::policy::validate_token_type"),
        "Transport must delegate validate_token_type by name to auth::policy"
    );
}

#[test]
fn transport_delegates_app_ticket_recovery_by_name() {
    let http = strip_rust_comments(&read(&core_src().join("http.rs")));
    assert!(
        http.contains("auth::app_ticket::recover_app_ticket_if_needed")
            || http.contains("crate::auth::app_ticket::recover_app_ticket_if_needed"),
        "do_request must α-delegate recover_app_ticket_if_needed (ADR-0002)"
    );
    assert!(
        !has_fn_def(&http, "recover_app_ticket_if_needed"),
        "recovery condition+action must not be reimplemented in http.rs"
    );
    assert!(
        !has_fn_def(&http, "apply_app_ticket"),
        "deleted apply_app_ticket must not return to Transport (ADR-0002 breaking)"
    );
    assert!(
        !has_fn_def(&http, "resend_app_ticket"),
        "resend must stay in auth/app_ticket, not Transport"
    );
}

#[test]
fn resend_uses_unified_request_builder_not_ad_hoc_client() {
    let app_ticket = strip_rust_comments(&read(&core_src().join("auth/app_ticket.rs")));
    assert!(
        has_fn_def(&app_ticket, "resend_app_ticket"),
        "resend_app_ticket must live in auth/app_ticket.rs"
    );
    assert!(
        app_ticket.contains("UnifiedRequestBuilder::build"),
        "resend must go through UnifiedRequestBuilder bootstrap (ADR-0002 Q1)"
    );
    // Forbidden ad-hoc HTTP exits (the pre-ADR second reqwest path).
    assert!(
        !app_ticket.contains("reqwest::Client::new"),
        "resend must not construct a fresh reqwest::Client"
    );
    assert!(
        !app_ticket.contains("http_client().post(") && !app_ticket.contains("http_client.post("),
        "resend must not call http_client().post() ad-hoc; use UnifiedRequestBuilder"
    );
    // Visibility: recovery helpers stay crate-private (no public escape hatch).
    let raw = read(&core_src().join("auth/app_ticket.rs"));
    for name in ["recover_app_ticket_if_needed", "resend_app_ticket"] {
        let ok = raw.contains(&format!("pub(crate) async fn {name}"))
            || raw.contains(&format!("pub(crate) fn {name}"));
        assert!(
            ok,
            "{name} must remain pub(crate) — no public resend/recovery escape hatch"
        );
        // Reject bare `pub fn` / `pub async fn` (without `(crate)`).
        for line in raw.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("pub(crate)") {
                continue;
            }
            if trimmed.starts_with(&format!("pub async fn {name}"))
                || trimmed.starts_with(&format!("pub fn {name}"))
            {
                panic!("{name} must not be public API; found: {trimmed}");
            }
        }
    }
}

#[test]
fn auth_handler_lives_under_auth_acquisition_not_request_execution() {
    let acquisition = core_src().join("auth/acquisition.rs");
    assert!(
        acquisition.is_file(),
        "auth/acquisition.rs must exist (AuthHandler home after ADR-0002)"
    );
    let acq = strip_rust_comments(&read(&acquisition));
    assert!(
        acq.contains("struct AuthHandler"),
        "AuthHandler must be defined in auth/acquisition.rs"
    );

    let resurrected = core_src().join("request_execution/auth_handler.rs");
    assert!(
        !resurrected.exists(),
        "request_execution/auth_handler.rs must stay deleted; AuthHandler lives under auth/"
    );

    let re_mod = strip_rust_comments(&read(&core_src().join("request_execution/mod.rs")));
    assert!(
        !has_fn_def(&re_mod, "determine_token_type"),
        "request_execution must not own token-type decision"
    );
    assert!(
        re_mod.contains("auth::AuthHandler") || re_mod.contains("crate::auth::AuthHandler"),
        "UnifiedRequestBuilder must import AuthHandler from auth, not define it locally"
    );
}

#[test]
fn adr0002_status_is_accepted() {
    let adr = repo_root().join("docs/adr/0002-auth-concern-concentration.md");
    let text = read(&adr);
    // Header status line must say Accepted (not merely mention the word later).
    let status_line = text
        .lines()
        .find(|l| l.contains("**状态**") || l.to_ascii_lowercase().contains("**status**"))
        .expect("ADR-0002 must have a status line");
    assert!(
        status_line.contains("Accepted"),
        "ADR-0002 status must be Accepted after lock-in (#584); got: {status_line}"
    );
    assert!(
        !status_line.contains("Proposed"),
        "ADR-0002 must not remain Proposed after code migration + lock-in; got: {status_line}"
    );
}
