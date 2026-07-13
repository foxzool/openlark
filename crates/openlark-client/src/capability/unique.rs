//! generation-time catalog 唯一性检查（#423）
//!
//! 从 `generate_catalog_registry!` 抽出，便于 trybuild 回归注入重复条目。

/// 断言一组 catalog 条目的 `field` / `name` 在生成期唯一且一致。
///
/// - 重复 `field` 标识符 → 同模块重复 unit struct → **编译失败**
/// - `name` 与 `stringify!(field)` 不一致 → const assert 失败 → **编译失败**
///
/// 普通 `cargo build` 即拒绝，不依赖 `-D warnings`。
///
/// 供 capability catalog 使用；`#[doc(hidden)]` 导出仅用于 compile-fail 测试。
#[macro_export]
#[doc(hidden)]
macro_rules! __openlark_assert_capability_catalog_unique {
    ($({
        field: $field:ident,
        name: $name:literal $(,)?
    }),* $(,)?) => {
        #[allow(non_camel_case_types)]
        mod __capability_catalog_unique_fields {
            $(
                pub struct $field;
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
                let _ = ::core::mem::size_of::<__capability_catalog_unique_fields::$field>();
                assert!(
                    __catalog_str_eq(::core::stringify!($field), $name),
                    "capability catalog: name must equal field identifier text"
                );
            )*
        };
    };
}
