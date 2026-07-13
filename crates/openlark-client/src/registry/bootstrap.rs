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
    use crate::capability::expected_capability_names_from_features;

    #[test]
    fn test_register_compiled_services() {
        let mut registry = DefaultServiceRegistry::new();
        assert!(register_compiled_services(&mut registry).is_ok());
    }

    /// bootstrap 委托 catalog：集合与顺序相对独立 feature oracle（非宏内部列表）。
    #[test]
    fn register_compiled_services_matches_catalog_names() {
        let mut registry = DefaultServiceRegistry::new();
        register_compiled_services(&mut registry).unwrap();

        let mut expected = expected_capability_names_from_features();
        assert_eq!(registry.list_services().len(), expected.len());
        for name in &expected {
            assert!(registry.has_service(name), "bootstrap 必须注册能力 {name}");
        }
        let listed: Vec<&str> = registry
            .list_services()
            .into_iter()
            .map(|e| e.metadata.name.as_str())
            .collect();
        expected.sort_by(|a, b| {
            let pa = registry.get_service(a).unwrap().metadata.priority;
            let pb = registry.get_service(b).unwrap().metadata.priority;
            pa.cmp(&pb).then_with(|| a.cmp(b))
        });
        assert_eq!(
            listed, expected,
            "list_services 须为稳定顺序（priority, name）"
        );
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
