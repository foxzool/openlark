//! `openlark-client/hr` feature 的最小对外契约检查。
//!
//! 该 feature 至少需要保证 `client.hr.config()` 可用。

#![cfg(feature = "hr")]

use openlark_client::Client;

#[test]
fn hr_feature_exposes_config_entry() {
    let client = Client::builder()
        .app_id("test_app")
        .app_secret("test_secret")
        .build()
        .expect("client should build with hr feature");

    assert_eq!(client.hr.config().app_id(), "test_app");
}
