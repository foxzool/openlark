//! 编译期服务元信息注册
//!
//! 唯一入口：capability catalog（#434–#437）。不再有 legacy catalog 双路径
//! 或 FeatureLoader 旁路初始化。

/// 将当前 feature 组合下的 catalog 能力注册为 registry 诊断元数据。
pub(crate) fn register_compiled_services(
    registry: &mut super::DefaultServiceRegistry,
) -> crate::Result<()> {
    crate::capability::register_catalog_capabilities(registry)
}

#[cfg(test)]
mod tests {
    use super::super::{DefaultServiceRegistry, ServiceRegistry};
    use super::*;
    use crate::capability::catalog_capability_names;

    #[test]
    fn test_register_compiled_services() {
        let mut registry = DefaultServiceRegistry::new();
        assert!(register_compiled_services(&mut registry).is_ok());
    }

    /// bootstrap 只委托 catalog：注册集合 = catalog 能力名（无第二套域矩阵）。
    #[test]
    fn register_compiled_services_matches_catalog_names() {
        let mut registry = DefaultServiceRegistry::new();
        register_compiled_services(&mut registry).unwrap();

        let catalog = catalog_capability_names();
        assert_eq!(registry.list_services().len(), catalog.len());
        for name in &catalog {
            assert!(
                registry.has_service(name),
                "bootstrap 必须注册 catalog 能力 {name}"
            );
        }
        // 稳定顺序：list_services 按 priority 再 name；与 catalog 声明顺序在同 priority 下一致
        let listed: Vec<&str> = registry
            .list_services()
            .into_iter()
            .map(|e| e.metadata.name.as_str())
            .collect();
        let mut expected = catalog;
        expected.sort_by(|a, b| {
            let pa = registry.get_service(a).unwrap().metadata.priority;
            let pb = registry.get_service(b).unwrap().metadata.priority;
            pa.cmp(&pb).then_with(|| a.cmp(b))
        });
        assert_eq!(listed, expected, "list_services 须为稳定顺序（priority, name）");
    }

    #[test]
    fn registry_exposes_immutable_metadata_only() {
        let mut registry = DefaultServiceRegistry::new();
        register_compiled_services(&mut registry).unwrap();

        #[cfg(feature = "auth")]
        {
            let entry = registry.get_service("auth").unwrap();
            assert_eq!(entry.metadata.name, "auth");
            assert!(entry.metadata.description.is_some());
            let _deps = &entry.metadata.dependencies;
            let _provides = &entry.metadata.provides;
            let _priority = entry.metadata.priority;
        }
    }
}
