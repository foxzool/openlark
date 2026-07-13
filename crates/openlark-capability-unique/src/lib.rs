//! Internal generation-time uniqueness for the openlark-client capability catalog.
//!
//! This crate is **`publish = false`** and is not part of the public SDK surface.
//! openlark-client uses it at catalog generation; trybuild tests live here so
//! openlark-client need not export test harness macros or Cargo features.

#![allow(missing_docs)]

/// Assert catalog entries are unique at generation time (#423).
///
/// - Duplicate `field` identifiers → duplicate unit struct → **compile failure**
/// - `name` must equal `stringify!(field)` byte-for-byte → **const assert failure**
///
/// Fails under a normal `cargo build` (no `-D warnings` required).
#[macro_export]
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
