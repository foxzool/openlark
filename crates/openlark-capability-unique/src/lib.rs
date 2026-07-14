//! trybuild harness：capability catalog 生成期唯一性（#423 / #455）
//!
//! 本 crate **`publish = false`**，且 **不是** `openlark-client` 的依赖。
//! 生产路径使用 `openlark-client` 内的 crate 私有宏；此处仅供 compile-fail
//! 固定期望，避免在可发布 crate 上导出测试宏或内部 Cargo feature。
//!
//! 导出的宏供 trybuild 使用。
//! 实现体与生产 client 中的重复（设计上为隔离；保持同步）。
//! 测试覆盖生成期拒绝逻辑。

/// 断言 catalog 条目在生成期唯一（trybuild 调用面）。
///
/// - 重复 `field` 标识符 → 重复 `mod` 项 → **编译失败**
/// - `name` 必须与 `stringify!(field)` 逐字节相等 → **const assert 失败**
///
/// 在普通 `cargo build` 下即可失败（无需 `-D warnings`）。
#[macro_export]
macro_rules! assert_capability_catalog_unique {
    ($({
        field: $field:ident,
        name: $name:literal $(,)?
    }),* $(,)?) => {
        mod __capability_catalog_unique_fields {
            $(
                /// 生成期占位模块：同名 field 重复时在此模块触发 E0428。
                /// 使用 snake_case `mod` 而非 struct，lint-clean（遵循 AGENTS.md PascalCase 仅适用于结构体）。
                pub mod $field {}
            )*
        }

        const fn __catalog_str_eq(a: &str, b: &str) -> bool {
            if a.len() != b.len() {
                return false;
            }
            let ab = a.as_bytes();
            let bb = b.as_bytes();
            let mut i = 0;
            while i < ab.len() {
                if ab[i] != bb[i] {
                    return false;
                }
                i += 1;
            }
            true
        }

        const _: () = {
            $(
                assert!(
                    __catalog_str_eq(::core::stringify!($field), $name),
                    "capability catalog: name must equal field identifier text"
                );
            )*
        };
    };
}
