//! 重复 field 必须在生成期失败

#[macro_use]
#[path = "../../../openlark-client/src/capability/unique.rs"]
mod production_unique;

fn main() {
    assert_capability_catalog_unique! {
        { feature: "auth", field: auth },
        { feature: "auth", field: auth },
    }
}
