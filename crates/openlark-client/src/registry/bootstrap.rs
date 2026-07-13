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

    #[test]
    fn test_register_compiled_services() {
        let mut registry = DefaultServiceRegistry::new();
        assert!(register_compiled_services(&mut registry).is_ok());
    }

    #[test]
    fn test_register_compiled_services_includes_all_catalog_domains() {
        let mut registry = DefaultServiceRegistry::new();
        register_compiled_services(&mut registry).unwrap();

        macro_rules! assert_catalog_service {
            ($feature:literal, $name:literal) => {
                #[cfg(feature = $feature)]
                assert!(
                    registry.has_service($name),
                    "{} feature 启用时 register_compiled_services 必须注册 {}",
                    $name,
                    $name
                );

                #[cfg(not(feature = $feature))]
                assert!(
                    !registry.has_service($name),
                    "{} feature 禁用时 register_compiled_services 不得注册 {}",
                    $name,
                    $name
                );
            };
        }

        assert_catalog_service!("auth", "auth");
        assert_catalog_service!("communication", "communication");
        assert_catalog_service!("docs", "docs");
        assert_catalog_service!("cardkit", "cardkit");
        assert_catalog_service!("meeting", "meeting");
        assert_catalog_service!("security", "security");
        assert_catalog_service!("hr", "hr");
        assert_catalog_service!("ai", "ai");
        assert_catalog_service!("workflow", "workflow");
        assert_catalog_service!("platform", "platform");
        assert_catalog_service!("application", "application");
        assert_catalog_service!("helpdesk", "helpdesk");
        assert_catalog_service!("mail", "mail");
        assert_catalog_service!("analytics", "analytics");
        assert_catalog_service!("user", "user");
        assert_catalog_service!("bot", "bot");
    }

    /// registry 为 metadata-only：无 typed instance、无 lifecycle 突变 API。
    #[test]
    fn registry_exposes_immutable_metadata_only() {
        let mut registry = DefaultServiceRegistry::new();
        register_compiled_services(&mut registry).unwrap();

        #[cfg(feature = "auth")]
        {
            let entry = registry.get_service("auth").unwrap();
            assert_eq!(entry.metadata.name, "auth");
            assert!(entry.metadata.description.is_some());
            // 字段存在性由类型系统保证：无 instance / status / created_at
            let _deps = &entry.metadata.dependencies;
            let _provides = &entry.metadata.provides;
            let _priority = entry.metadata.priority;
        }
    }
}
