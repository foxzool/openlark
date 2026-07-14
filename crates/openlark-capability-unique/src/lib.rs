//! trybuild harness：capability catalog 生成期唯一性（#423 / #455）
//!
//! 本 crate **`publish = false`**，且 **不是** `openlark-client` 的依赖。
//! 生产路径使用 `openlark-client` 内的 crate 私有宏；此处仅供 compile-fail
//! 固定期望，避免在可发布 crate 上导出测试宏或内部 Cargo feature。
//!
//! 宏核心实现通过 `include!` 直接共享自 openlark-client 的 `unique-macro.inc.rs`。
//! 因此 compile-fail 测试使用与生产完全相同的逻辑文本（无手工同步风险）。

include!("../../openlark-client/src/capability/unique-macro.inc.rs");

/// 断言 catalog 条目在生成期唯一（trybuild 调用面）。
///
/// - 重复 `field` 标识符 → 重复 `mod` 项 → **编译失败**
/// - `name` 必须与 `stringify!(field)` 逐字节相等 → **const assert 失败**
///
/// 在普通 `cargo build` 下即可失败（无需 `-D warnings`）。
#[macro_export]
macro_rules! assert_capability_catalog_unique {
    ($($args:tt)*) => { $crate::__capability_catalog_unique_impl! { $($args)* } };
}
