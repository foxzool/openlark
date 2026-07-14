//! name 与 field 不一致必须在生成期失败

#[macro_use]
#[path = "../../../openlark-client/src/capability/unique.rs"]
mod production_unique;

fn main() {
    assert_capability_catalog_unique! {
        { field: auth, name: "bot" },
    }
}
