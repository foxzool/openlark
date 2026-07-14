//! compile-fail：generation-time catalog uniqueness（#423 / #455）
//!
//! 此测试直接使用 openlark_client::assert_capability_catalog_unique! (via the _test-catalog-unique feature on the dep).
//! This ensures the compile-fail test exercises the actual production macro implementation.

#[test]
fn capability_catalog_generation_time_uniqueness() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/duplicate_capability_field.rs");
    t.compile_fail("tests/ui/mismatch_capability_name.rs");
}
