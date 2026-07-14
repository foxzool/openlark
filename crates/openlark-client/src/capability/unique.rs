//! generation-time catalog 唯一性检查（#423 / #455）
//!
//! **生产路径**：crate 私有宏（不进入公开 API、无 Cargo feature、无 `#[macro_export]`）。
//! **trybuild**：见 workspace 成员 `openlark-capability-unique`（`publish = false`，
//! 且 **不是** 本 crate 的依赖，避免阻断 `cargo package` / crates.io 发布）。
//!
//! 注意：宏定义体与 helper crate 存在文本重复（隔离发布目的）；修改时须同步。

/// 生产用：私有唯一性断言（catalog 注册宏调用）。
///
/// - 重复 `field` → 同模块重复 `mod` 项 → E0428 编译失败（lint clean，无需 non_camel_case_types allow）
/// - `name` 必须与 `stringify!(field)` 逐字节相等 → **const assert 失败**
macro_rules! assert_capability_catalog_unique {
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
