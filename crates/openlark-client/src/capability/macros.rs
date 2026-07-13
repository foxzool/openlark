//! 编译期能力目录宏
//!
//! callback 模式：`for_each_compiled_capability!`（定义于 `catalog.rs`）持有统一声明，
//! 本模块的 `generate_catalog_registry!` 与 `client` 侧 `append_catalog_entries!`
//! 分别投影同一条目列表（Client 构造 vs registry 诊断）。
//!
//! 宏面保持最小（单列表 + 两投影）；扩容靠 catalog 条目，不新增宏层。

/// 由 `for_each_compiled_capability!` 展开：生成 registry 注册与测试辅助。
///
/// 死匹配（field/ty/doc/init）：统一条目同时含构造与诊断字段，本侧只消费诊断字段；
/// 与 `append_catalog_entries!` 对称，是双投影的固有成本（#434 review）。
macro_rules! generate_catalog_registry {
    ($({
        feature: $feature:literal,
        // Client 构造字段：由 append_catalog_entries! 消费；此处仅匹配统一条目形状
        // `$field` 同时用于 generation-time 唯一性（同名字段 → 重复 struct 编译失败）
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
        /// generation-time：catalog 声明内 `field` 必须唯一（#423 duplicate-name rejection）。
        /// 不随 feature cfg 开关——声明表本身不可有重复条目。
        #[allow(non_camel_case_types, dead_code)]
        mod __capability_catalog_unique_fields {
            $(
                pub struct $field;
            )*
        }

        /// generation-time：诊断 `name` 字面量必须唯一（重复 match arm → unreachable pattern）。
        /// 在 `register_catalog_capabilities` 内调用，避免 unused 警告。
        fn __capability_catalog_unique_names() {
            // 重复 `$name` 会在第二次成为 unreachable pattern（-D warnings 失败）
            match "" {
                $(
                    $name => {}
                )*
                _ => {}
            }
        }

        /// 将 catalog 中已编译的能力注册为 registry 元数据（metadata-only）。
        pub(crate) fn register_catalog_capabilities(
            registry: &mut crate::DefaultServiceRegistry,
        ) -> crate::Result<()> {
            // generation-time 重名检查；并保持 `register_service` 在无 feature 时仍被引用
            __capability_catalog_unique_names();
            let _: fn(
                &mut crate::DefaultServiceRegistry,
                crate::registry::ServiceMetadata,
            ) -> crate::registry::RegistryResult<()> =
                crate::DefaultServiceRegistry::register_service;

            $(
                #[cfg(feature = $feature)]
                {
                    use crate::registry::ServiceMetadata;

                    // 只写诊断元数据；构造字段已在匹配中吸收。
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

        /// 当前编译进包的 catalog 能力名（仅测试/诊断使用；顺序与声明一致）。
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
