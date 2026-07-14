//! compile-fail：generation-time catalog uniqueness（#423 / #455）
//!
//! 此测试通过 openlark_capability_unique 导出的宏验证错误格式。
//! 实现体与生产 client 中的保持一致（设计隔离）。
//! 生产路径在 client 每次构建 catalog 时执行断言。

#[test]
fn capability_catalog_generation_time_uniqueness() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/duplicate_capability_field.rs");
    t.compile_fail("tests/ui/mismatch_capability_name.rs");
}
