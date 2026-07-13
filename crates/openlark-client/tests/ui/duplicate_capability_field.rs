//! 重复 field 必须在生成期失败（非 runtime register）

fn main() {
    openlark_client::__openlark_assert_capability_catalog_unique! {
        { field: auth, name: "auth" },
        { field: auth, name: "auth" },
    }
}
