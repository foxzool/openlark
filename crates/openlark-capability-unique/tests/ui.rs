//! compile-fail：generation-time catalog uniqueness（#423 / #455）
//!
//! 此测试通过 openlark-capability-unique 导出的宏（其实现体通过 include! 直接来自
//! openlark-client 的唯一源码 unique-macro.inc.rs）来验证错误格式与行为。
//! 因此 compile-fail 实际使用与生产路径完全相同的逻辑。
//!
//! 生产路径在 openlark-client 每次编译 catalog（generate_catalog_registry!）时
//! 都会执行相同的断言，确保重复 field / name mismatch 会在 generation time 失败。

#[test]
fn capability_catalog_generation_time_uniqueness() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/duplicate_capability_field.rs");
    t.compile_fail("tests/ui/mismatch_capability_name.rs");
}
