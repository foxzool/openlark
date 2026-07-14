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
        // generation-time 唯一性（#423 / #455）：crate 私有宏（见 unique.rs；非公开 API）
        // 实现体共享自 unique-macro.inc.rs，compile-fail 测试通过 harness reexport 覆盖生产版本。
        // 使用 snake_case mod 占位实现重复检测，lint-clean（无 non_camel_case_types allow）。
        assert_capability_catalog_unique! {
            $(
                { field: $field, name: $name },
            )*
        }

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
