//! 重复 field 必须在生成期失败

fn main() {
    openlark_client::assert_capability_catalog_unique! {
        { field: auth, name: "auth" },
        { field: auth, name: "auth" },
    }
}
