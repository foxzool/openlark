//! name 与 field 标识符不一致必须在生成期失败

fn main() {
    openlark_client::__openlark_assert_capability_catalog_unique! {
        { field: auth, name: "bot" },
    }
}
