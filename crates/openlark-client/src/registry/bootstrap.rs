//! 内置服务注册（meta 单入口）
//!
//! openlark-client 不在本 crate 内重复实现业务 API 包装层，但仍需要一份轻量的“服务元信息”
//! 以支持 registry 的可观测、依赖关系展示等用途。
//!
//! #434–#436 后，业务域元数据由 `capability` catalog 统一生成；legacy
//! `registry/catalog` 为空 no-op（#437 可进一步删除重复 bootstrap / FeatureLoader）。

#[cfg(test)]
use super::ServiceStatus;

#[path = "catalog.rs"]
mod catalog;

/// 注册所有已编译服务元数据（capability catalog；legacy catalog 为空）。
pub(crate) fn register_compiled_services(
    registry: &mut super::DefaultServiceRegistry,
) -> crate::Result<()> {
    catalog::register_compiled_services(registry)?;
    crate::capability::register_catalog_capabilities(registry)?;
    Ok(())
}

#[cfg(test)]
use catalog::compiled_service_names;

#[cfg(test)]
fn service_metadata(
    name: &'static str,
    description: &'static str,
    dependencies: &[&'static str],
    provides: &[&'static str],
    priority: u32,
) -> super::ServiceMetadata {
    super::ServiceMetadata {
        name: name.to_string(),
        version: "1.0.0".to_string(),
        description: Some(description.to_string()),
        dependencies: owned_strings(dependencies),
        provides: owned_strings(provides),
        status: super::ServiceStatus::Uninitialized,
        priority,
    }
}

#[cfg(test)]
fn owned_strings(values: &[&'static str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::super::DefaultServiceRegistry;
    use super::*;

    #[test]
    fn test_service_metadata_creation() {
        let metadata = service_metadata(
            "test_service",
            "Test service description",
            &["auth"],
            &["test"],
            1,
        );

        assert_eq!(metadata.name, "test_service");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(
            metadata.description.as_deref(),
            Some("Test service description")
        );
        assert_eq!(metadata.dependencies, vec!["auth"]);
        assert_eq!(metadata.provides, vec!["test"]);
        assert_eq!(metadata.priority, 1);
    }

    #[test]
    fn test_service_status_debug() {
        let status = ServiceStatus::Uninitialized;
        let debug_str = format!("{status:?}");
        assert!(debug_str.contains("Uninitialized"));
    }

    #[test]
    fn test_service_status_clone() {
        let status = ServiceStatus::Uninitialized;
        let cloned = status.clone();
        assert!(matches!(cloned, ServiceStatus::Uninitialized));
    }

    #[test]
    fn test_register_compiled_services() {
        let mut registry = DefaultServiceRegistry::new();
        let result = register_compiled_services(&mut registry);
        assert!(result.is_ok());
    }

    #[test]
    fn test_legacy_compiled_service_names_empty_after_catalog_migration() {
        let service_names = compiled_service_names();
        assert!(
            service_names.is_empty(),
            "legacy compiled_service_names 在 #436 后应为空，实际: {service_names:?}"
        );
    }

    #[test]
    fn test_register_compiled_services_includes_all_catalog_domains() {
        use super::super::ServiceRegistry;

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

    #[test]
    fn test_service_metadata_with_empty_dependencies() {
        let metadata = service_metadata("standalone_service", "Standalone", &[], &[], 0);

        assert!(metadata.dependencies.is_empty());
        assert!(metadata.provides.is_empty());
    }

    #[test]
    fn test_service_metadata_priority_ordering() {
        let low_priority = service_metadata("low", "Low priority", &[], &[], 1);
        let high_priority = service_metadata("high", "High priority", &[], &[], 10);

        assert!(high_priority.priority > low_priority.priority);
    }
}
