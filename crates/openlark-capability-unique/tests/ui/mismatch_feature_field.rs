//! feature 与 field 不一致必须在生成期失败（#471 review P1）。
//!
//! `feature: "auth", field: bot` 会把 `bot` 字段静默挂到 `auth` Cargo feature 下
//!（runtime 后果：字段被错误门控）。宏在生成期用 const 断言拒绝。

#[macro_use]
#[path = "../../../openlark-client/src/capability/unique.rs"]
mod production_unique;

fn main() {
    assert_capability_catalog_unique! {
        { feature: "auth", field: bot },
    }
}
