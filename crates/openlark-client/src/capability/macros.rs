//! 编译期能力目录宏
//!
//! callback 模式：`for_each_compiled_capability!`（定义于 `catalog.rs`）持有统一声明，
//! 本模块的 `generate_catalog_registry!` 与 `client` 侧 `append_catalog_entries!`
//! 分别投影同一条目列表（Client 构造 vs registry 诊断）。
//!
//! 宏面保持最小（单列表 + 两投影）；扩容靠 catalog 条目，不新增宏层。

/// 由 `for_each_compiled_capability!` 展开：生成 registry 注册与测试辅助。
///
/// 死匹配（ty/doc/init）：统一条目同时含构造与诊断字段，本侧只消费诊断字段；
/// 与 `append_catalog_entries!` 对称，是双投影的固有成本（#434 review）。
macro_rules! generate_catalog_registry {
    ($({
        feature: $feature:literal,
        // `$field`：Client 字段名；generation-time 唯一性（同模块重复 struct → 编译失败）
        field: $field:ident,
        ty: $_ty:ty,
        doc: $_doc:literal,
        init: |$_core_config:ident, $_base_core_config:ident| $_init:block,
        name: $name:literal,
        description: $description:literal,
        dependencies: [$($dependency:literal),* $(,)?],
        provides: [$($capability:literal),* $(,)?],
        priority: $priority:literal $(,)?
    }),* $(,)?) => {
        // generation-time 唯一性（#423）：
        // 每个 field 生成 unit struct（重复标识符 → 编译失败）；const 块 size_of 引用类型，
        // 且 assert name == stringify!(field)。普通 cargo build 即拒绝，无需 -D warnings。
        #[allow(non_camel_case_types)]
        mod __capability_catalog_unique_fields {
            $(
                pub struct $field;
            )*
        }

        /// const 字符串相等（generation-time name == field）。
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
                let _ = core::mem::size_of::<__capability_catalog_unique_fields::$field>();
                assert!(
                    __catalog_str_eq(stringify!($field), $name),
                    "capability catalog: name must equal field identifier text"
                );
            )*
        };

        /// 将 catalog 中已编译的能力注册为 registry 元数据（metadata-only）。
        pub(crate) fn register_catalog_capabilities(
            registry: &mut crate::DefaultServiceRegistry,
        ) -> crate::Result<()> {
            // 保持 `register_service` 在无业务 feature 时仍被引用（避免 dead_code）
            let _: fn(
                &mut crate::DefaultServiceRegistry,
                crate::registry::ServiceMetadata,
            ) -> crate::registry::RegistryResult<()> =
                crate::DefaultServiceRegistry::register_service;

            $(
                #[cfg(feature = $feature)]
                {
                    use crate::registry::ServiceMetadata;

                    let metadata = ServiceMetadata {
                        name: $name.to_string(),
                        version: "1.0.0".to_string(),
                        description: Some($description.to_string()),
                        dependencies: vec![$($dependency.to_string()),*],
                        provides: vec![$($capability.to_string()),*],
                        priority: $priority,
                    };
                    registry
                        .register_service(metadata)
                        .map_err(crate::error::registry_error)?;
                }
            )*

            let _ = registry;
            Ok(())
        }

        /// 当前编译进包的 catalog 能力名（顺序与声明一致；仅测试对照生成结果）。
        #[cfg(test)]
        pub(crate) fn catalog_capability_names() -> Vec<&'static str> {
            [
                None::<&'static str>,
                $(
                    #[cfg(feature = $feature)]
                    Some($name),
                )*
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    };
}
