//! generation-time catalog 唯一性检查（#423 / #455 / #471）
//!
//! **生产路径**：crate 私有宏（不进入公开 API、无 Cargo feature、无 `#[macro_export]`）。
//! **trybuild**：见 workspace 成员 `openlark-capability-unique`（`publish = false`，
//! 且 **不是** 本 crate 的依赖，避免阻断 `cargo package` / crates.io 发布）。
//!
//! UI 用例通过 `#[path]` 直接引入本文件，因此既覆盖生产实现，
//! 又不需要为测试暴露 Cargo feature 或 `#[macro_export]`。

// catalog 字段标识符的生成期唯一性断言（见上方用法）。
//
// 本宏只接收 catalog 条目的 `feature` / `field`（`ty`/`doc`/`init` 由
// `assert_catalog_fields_unique!` 投影掉，catalog 条目本身仍是 5 字段），检查两条不变量：
// 1. 字段标识符唯一：重复 `$field` 触发 E0428。
// 2. feature↔field 不漂移：`$feature` 字面量必须等于 `stringify!($field)`，
//    防止 `feature: "auth", field: bot` 这类把字段静默挂到错误 Cargo feature 下的错误
//    （该漂移有 runtime 后果——字段被错误门控；#471 review P1）。
macro_rules! assert_capability_catalog_unique {
    ($({
        feature: $feature:literal,
        field: $field:ident $(,)?
    }),* $(,)?) => {
        // 1. 字段标识符唯一性：重复 $field 在此触发 E0428。
        //    使用 `mod`（snake_case 合法）而非 unit struct，避免 #[allow(non_camel_case_types)]。
        mod __capability_catalog_unique_fields {
            $(
                /// 生成期占位模块：重复 field 标识符时在此触发 E0428。
                pub mod $field {}
            )*
        }

        // 2. feature↔field 不漂移：feature 字面量必须等于 field 标识符文本。
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
                    __catalog_str_eq($feature, ::core::stringify!($field)),
                    "capability catalog: feature must equal field identifier text"
                );
            )*
        };
    };
}
