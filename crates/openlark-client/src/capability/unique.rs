//! generation-time catalog 唯一性检查（#423）
//!
//! 生产路径：crate 私有 `assert_capability_catalog_unique!`（不进入公开 API）。
//! trybuild：仅 feature `__compile_fail_harness` 导出隐藏宏（`[[test]] required-features`）。

/// 生产用：私有唯一性断言（catalog 注册宏调用）。
macro_rules! assert_capability_catalog_unique {
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
pub(crate) use assert_capability_catalog_unique;

/// trybuild 专用：完整展开在调用方（不依赖 pub(crate) 宏跨 crate 可见性）。
///
/// **不是**受支持的公开 API。
#[cfg(feature = "__compile_fail_harness")]
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
