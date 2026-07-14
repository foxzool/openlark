//! compile-fail：generation-time catalog uniqueness（#423 / #455）
//!
//! 注意：此测试使用 helper 导出的宏外壳验证错误格式。
//! 生产路径（openlark-client 内的私有宏定义 + catalog 生成）在每次
//! `cargo check -p openlark-client` / 测试时均被实际编译使用并验证无重复。

#[test]
fn capability_catalog_generation_time_uniqueness() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/duplicate_capability_field.rs");
    t.compile_fail("tests/ui/mismatch_capability_name.rs");
}
