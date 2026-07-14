//! compile-fail：generation-time catalog uniqueness（#423 / #455）
//!
//! 此测试通过 openlark-capability-unique 导出的宏验证错误格式。
//! 宏实现臂直接 include 自 openlark-client 的生产源码（unique-macro.inc.rs）。
//! 因此测试覆盖的是与生产相同的逻辑实现。
//!
//! 生产路径在 client 每次生成 catalog 时也会执行该断言。

#[test]
fn capability_catalog_generation_time_uniqueness() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/duplicate_capability_field.rs");
    t.compile_fail("tests/ui/mismatch_capability_name.rs");
}
