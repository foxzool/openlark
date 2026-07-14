//! generation-time catalog 唯一性检查（#423 / #455）
//!
//! **生产路径**：crate 私有宏（不进入公开 API、无 Cargo feature、无 `#[macro_export]`）。
//! **trybuild**：见 workspace 成员 `openlark-capability-unique`（`publish = false`，
//! 且 **不是** 本 crate 的依赖，避免阻断 `cargo package` / crates.io 发布）。
//!
//! 宏核心实现（__capability_catalog_unique_impl）通过 include! 共享自
//! `unique-macro.inc.rs`，确保生产路径与 compile-fail 测试使用完全一致的逻辑，
//! 无需手工同步两份实现。

include!("unique-macro.inc.rs");

/// 生产用：私有唯一性断言（catalog 注册宏调用）。
///
/// - 重复 `field` → 同模块重复 `mod` 项 → E0428 编译失败（lint clean，无需 non_camel_case_types allow）
/// - `name` 必须与 `stringify!(field)` 逐字节相等 → **const assert 失败**
macro_rules! assert_capability_catalog_unique {
    ($($args:tt)*) => { __capability_catalog_unique_impl! { $($args)* } };
}
