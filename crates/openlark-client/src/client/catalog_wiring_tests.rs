//! Catalog 接线测试（#471：替代 4 个 tautological contract 测试）。
//!
//! 真不变量仍由 trybuild（`openlark-capability-unique`）编译期保证：字段唯一、
//! 禁用 feature 不产 Client 字段。本模块只做 runtime 接线 smoke：
//! (1) `with_checked_core_config` 跑通 catalog 字段构造（bootstrap）；
//! (2) Client 业务域字段数 == 当前开启的 catalog feature 数；
//! (3) Client 公开字段顺序保持 0.17 兼容（#423 锁定，非 tautological）。

use super::Client;

fn test_client() -> Client {
    Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap()
}

/// Client Debug 为每个 catalog 条目输出 `"field": <bool>`（值为 `cfg!(feature)`）。
/// 计 `": true"` 个数 = 已编译进 Client 的业务域字段数。
fn count_enabled_client_fields(client: &Client) -> usize {
    format!("{client:?}").matches(": true").count()
}

/// 当前编译进 Client 的 catalog feature 数（test-local 计数，非 production oracle）。
/// 与 `capability/catalog.rs` 声明顺序无关；只数 feature 开关。
fn count_enabled_catalog_features() -> usize {
    [
        cfg!(feature = "cardkit"),
        cfg!(feature = "auth"),
        cfg!(feature = "docs"),
        cfg!(feature = "communication"),
        cfg!(feature = "hr"),
        cfg!(feature = "meeting"),
        cfg!(feature = "ai"),
        cfg!(feature = "workflow"),
        cfg!(feature = "platform"),
        cfg!(feature = "application"),
        cfg!(feature = "helpdesk"),
        cfg!(feature = "mail"),
        cfg!(feature = "analytics"),
        cfg!(feature = "user"),
        cfg!(feature = "security"),
        cfg!(feature = "bot"),
    ]
    .into_iter()
    .filter(|b| *b)
    .count()
}

#[test]
fn catalog_bootstraps_one_client_field_per_enabled_feature() {
    // bootstrap：with_checked_core_config 完整跑通 catalog 字段构造。
    let client = test_client();

    let field_count = count_enabled_client_fields(&client);
    let feature_count = count_enabled_catalog_features();

    assert_eq!(
        field_count, feature_count,
        "Client 业务域字段数必须等于开启的 catalog feature 数（catalog 接线一致）"
    );
}

/// #423 明确不允许在未单独决策时调整公开业务字段顺序。
///
/// `Client` 的 Debug 投影对每个 catalog 条目（含禁用的，输出 `false`）按声明顺序输出，
/// 因此用它锁定 0.17 已有字段顺序。这不是 tautological——它防止意外的字段重排。
#[test]
fn client_public_field_order_remains_compatible() {
    let debug = format!("{:?}", test_client());
    let expected = [
        "cardkit",
        "auth",
        "docs",
        "communication",
        "hr",
        "meeting",
        "ai",
        "workflow",
        "platform",
        "application",
        "helpdesk",
        "mail",
        "analytics",
        "user",
        "security",
        "bot",
    ];

    let mut previous = 0;
    for field in expected {
        let marker = format!("{field}: ");
        let position = debug
            .find(&marker)
            .unwrap_or_else(|| panic!("Client Debug 缺少字段 {field}: {debug}"));
        assert!(
            position >= previous,
            "Client 公开字段顺序改变：{field} 出现在前一字段之前；{debug}"
        );
        previous = position;
    }
}
