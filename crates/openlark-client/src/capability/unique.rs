//! generation-time catalog 唯一性检查（#423 / #455）
//!
//! **生产路径**：crate 私有宏（不进入公开 API、无 Cargo feature、无 `#[macro_export]`）。
//! **trybuild**：见 workspace 成员 `openlark-capability-unique`（`publish = false`，
//! 且 **不是** 本 crate 的依赖，避免阻断 `cargo package` / crates.io 发布）。
//!
//! 宏实现臂通过 include! 共享自 unique-macro.inc.rs，实现体单一来源。
//! 生产 client 私有使用，harness 仅用于导出测试宏。

/// 生产用：私有唯一性断言（catalog 注册宏调用）。
///
/// - 重复 `field` → 同模块重复 `mod` 项 → E0428 编译失败（lint clean，无需 non_camel_case_types allow）
/// - `name` 必须与 `stringify!(field)` 逐字节相等 → **const assert 失败**
include!("unique-macro.inc.rs");

#[cfg(not(feature = "_test-catalog-unique"))]
macro_rules! assert_capability_catalog_unique {
    ($($t:tt)*) => { __capability_catalog_unique_body! { $($t)* } };
}

#[cfg(feature = "_test-catalog-unique")]
#[macro_export]
#[doc(hidden)]
macro_rules! assert_capability_catalog_unique {
    ($($t:tt)*) => { __capability_catalog_unique_body! { $($t)* } };
}
