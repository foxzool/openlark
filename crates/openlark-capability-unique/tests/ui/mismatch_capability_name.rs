//! name 与 field 不一致必须在生成期失败

fn main() {
    openlark_client::assert_capability_catalog_unique! {
        { field: auth, name: "bot" },
    }
}
