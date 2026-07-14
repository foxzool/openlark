//! compile-fail：generation-time catalog uniqueness（#423 / #455）
//!
//! UI 用例直接引入生产 `capability/unique.rs`，确保 compile-fail
//! 覆盖真实宏实现，不经过第二份 harness 实现。

#[test]
fn capability_catalog_generation_time_uniqueness() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/duplicate_capability_field.rs");
    t.compile_fail("tests/ui/mismatch_capability_name.rs");
}
