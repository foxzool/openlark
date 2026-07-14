// Shared implementation body for generation-time catalog uniqueness assert (#423 / #455).
//
// This file is `include!`d by both:
// - Production: crates/openlark-client/src/capability/unique.rs (private macro)
// - Test harness: crates/openlark-capability-unique/src/lib.rs (#[macro_export] for trybuild)
//
// Single source of truth for the duplicate detection logic. Prevents drift between
// the version exercised by normal client builds and the compile-fail tests.
//
// The wrappers in the including sites provide the appropriate visibility
// (private vs exported) without duplicating the mod-generation + const-eval logic.

#[macro_export]
#[doc(hidden)]
macro_rules! __capability_catalog_unique_impl {
    ($({
        field: $field:ident,
        name: $name:literal $(,)?
    }),* $(,)?) => {
        mod __capability_catalog_unique_fields {
            $(
                /// 生成期占位模块：重复 field 标识符时在此触发 E0428。
                /// 使用 `mod`（snake_case 合法）而非 unit struct，避免 #[allow(non_camel_case_types)]。
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
