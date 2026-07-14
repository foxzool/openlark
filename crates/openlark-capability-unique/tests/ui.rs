//! compile-fail：generation-time catalog uniqueness（#423 / #455）
//!
//! 此测试通过 openlark-capability-unique 导出的宏来验证错误格式。
//! 实现体与 openlark-client 生产路径中的保持一致（手动同步）。
//! 生产路径在每次编译 catalog 时执行断言。

#[test]
fn capability_catalog_generation_time_uniqueness() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/duplicate_capability_field.rs");
    t.compile_fail("tests/ui/mismatch_capability_name.rs");
}
