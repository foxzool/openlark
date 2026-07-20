# Code Review: GitHub #445 "refactor(security): migrate ACS to retained canonical Config"

**Date**: 2026-07-20  
**Reviewer**: Grok (focused review)  
**Scope**: Changed files + key ACS entry points (AcsProject, sample leaf) + top SecurityClient construction + from_config paths + Config propagation

## Summary

The implementation successfully migrates ACS (and security_and_compliance) to use the retained canonical `openlark_core::config::Config` without rebuilding in the primary (`from_config`) paths.

- **Canonical construction path is clean**: `SecurityClient::from_config(config)` / `SecurityServices::from_config` simply clone the Config into `AcsProject`, `AcsV1Service`, leaf Services, and Request builders. No `Config::builder()` is called.
- **Full Config flows through**: `token_provider`, custom `header`s, `req_timeout`, `max_response_size` (and other fields) are preserved because `Config` is `Arc<ConfigInner>` backed, and `with_token_provider` does a full inner clone before swapping only the provider.
- **Transport layer uses the fields**: `UnifiedRequestBuilder` applies `config.header()`, `config.req_timeout`, `config.token_provider().get_token(...)`, and `config.max_response_size()`.
- **Representative wiremock tests exist**:
  - Direct `SecurityClient::from_config` test (`openlark-security/src/lib.rs`) hits `acs.v1().users().list()`, asserts base_url/header via wiremock matchers + explicit `acs.config()` assertions for timeout/response-size, and exercises token_provider.
  - Root `Client` -> `security.acs` path test (`openlark-client/src/client/tests.rs`) exercises the meta chain with custom header propagation.
- **Public leaf builder interfaces are unchanged**: `XXXRequest::new(config: Config)`, `.page_size()`, `.execute()`, `.execute_with_options()`, `.body(...)` etc. retain their prior shapes across users/devices/access_records/etc.
- **Legacy paths are isolated and documented**: `SecurityClient::new(legacy: SecurityConfig)` and `SecurityServices::new` still rebuild a minimal Config (only app_id/secret/base_url). Clearly marked as "兼容旧路径" with recommendation to use `from_config`.

**No evidence of stripping in the from_config → AcsProject → leaf Request path.**

The changes (test enhancements + doc comment updates) correctly cover the AC items for the canonical path.

## Issues

### Bug
(none found)

### Suggestion

1. **root->security propagation test could strengthen coverage for timeout/response-size**  
   File: `crates/openlark-client/src/client/tests.rs:687` (function `root_client_security_leaf_receives_base_url_headers_via_canonical_path`)  
   The test sets a custom header and proves it reaches the ACS leaf via wiremock, but does not set or assert `req_timeout` / `max_response_size` on `client.security.acs.config()` (unlike the direct `SecurityClient` test in security crate).  
   Since the root ClientBuilder applies a default 30s timeout and the wrapped config is passed verbatim to `SecurityClient::from_config`, this is not a correctness bug. Adding explicit assertions (or setting custom values + asserting) would make the root path test a fuller peer of the direct one for #445 AC.

2. **Consider whether legacy `SecurityClient::new(SecurityConfig)` should be hidden or deprecated in public API**  
   File: `crates/openlark-security/src/lib.rs:185` (`SecurityClient::new`) and `212` (Default impl)  
   The legacy constructor is still `pub` and re-exports `SecurityConfig` in prelude. While documented, users coming from older examples may silently lose advanced Config capabilities (headers, custom token provider, custom timeout/size).  
   Options: keep as-is (current expand-contract approach), mark `#[deprecated]`, or move to a `legacy` sub-module. Not required for this PR, but worth a follow-up note if full deprecation of the old shape is planned.

### Nit

1. **Legacy rebuild in `SecurityConfig::get_app_access_token`**  
   File: `crates/openlark-security/src/config.rs:41`  
   This method does `Config::builder()...` internally. It is only reachable via the legacy `SecurityConfig` type (not used by canonical ACS leaves or `SecurityClient::from_config`). The comment in the method explains the old path. Harmless for #445 scope, but a small source of "still rebuilding" code in the crate.

2. **Test-only `Config::builder` calls are numerous but correctly gated**  
   Many leaf files contain `fn test_config() -> Config { Config::builder()... }` inside `#[cfg(test)]`. This is expected and does not violate the "no Config::builder in ACS prod paths" rule. No action needed.

3. **Comment in `AcsProject` still references old migration Task language**  
   File: `crates/openlark-security/src/acs/acs/mod.rs:5-9`  
   The module doc mentions "Task 1 / Task 2-5" from the original migration spec. Now that canonical Config retention is done for ACS, a light cleanup of this comment could reduce future confusion (non-blocking nit).

4. **Minor asymmetry in root client test doc comment**  
   File: `crates/openlark-client/src/client/tests.rs:684-685`  
   The updated comment says "覆盖 base_url、headers" and explains option token priority. It is accurate for what the test does, but could optionally mention that `client.security` is itself a `SecurityClient` constructed via the catalog `from_config` path (already implied by the call to `client.security.acs`).

## Positive Observations (for completeness)

- `AcsProject::new`, `AcsV1Service::new`, `UsersService::new`, `ListUsersRequest::new` (and peers) are all pure "store the Config" — excellent minimal surface.
- `with_token_provider` implementation in core correctly clones the full inner before swapping provider (preserves headers/timeout/size).
- `apply_headers` in `UnifiedRequestBuilder` iterates `config.header()` after option headers (correct precedence for custom headers from root builder).
- Timeout application prefers `req.timeout` > `config.req_timeout`.
- The representative leaf (`users.list`) + wiremock header matchers directly prove end-to-end propagation for the security client canonical path.
- No `Config::builder` appears in any non-test code under `src/acs/` or inside the service leaf modules.

## Files Inspected

- Changed (per diff):
  - `crates/openlark-security/src/lib.rs`
  - `crates/openlark-client/src/client/tests.rs`
- Key ACS / construction:
  - `crates/openlark-security/src/acs/acs/mod.rs` (AcsProject, AcsV1Service, FaceAccessors)
  - `crates/openlark-security/src/acs/acs/v1/users/mod.rs`
  - `crates/openlark-security/src/acs/acs/v1/user/list.rs` (representative leaf)
  - `crates/openlark-security/src/acs/acs/v1/user/get.rs`, `create.rs`, `device/list.rs`, `access_record/list.rs` (sampled)
  - `crates/openlark-security/src/lib.rs` (SecurityClient/SecurityServices + tests)
  - `crates/openlark-security/src/config.rs` (legacy)
  - `crates/openlark-security/src/security/security_and_compliance/mod.rs`
- Client wiring:
  - `crates/openlark-client/src/capability/catalog.rs`
  - `crates/openlark-client/src/client.rs`
  - `crates/openlark-client/src/client/builder.rs`
- Core (for propagation verification):
  - `crates/openlark-core/src/config/mod.rs` (`with_token_provider`, fields)
  - `crates/openlark-core/src/request_execution/mod.rs` (`apply_headers`, timeout)
  - `crates/openlark-core/src/request_execution/auth_handler.rs` (token_provider usage)
  - `crates/openlark-core/src/http.rs`

## Conclusion

The implementation meets the stated acceptance criteria for the canonical retained Config path. The test updates are appropriate and targeted. Only minor suggestions/nits; no blocking bugs.

Recommended actions: consider the two suggestions for future polish; merge is safe for the scope of #445.
