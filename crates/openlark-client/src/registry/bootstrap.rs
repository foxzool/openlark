//! 内置服务注册（meta 单入口）
//!
//! openlark-client 不在本 crate 内重复实现业务 API 包装层，但仍需要一份轻量的“服务元信息”
//! 以支持 registry 的可观测、依赖关系展示等用途。
//!
//! 这里集中管理 **按 feature 编译进来的服务**，避免在多个位置重复注册（DRY）。

#[cfg(test)]
use super::ServiceStatus;

macro_rules! compiled_services {
    ($(
        {
            feature: $feature:literal,
            name: $name:literal,
            description: $description:literal,
            dependencies: [$($dependency:literal),* $(,)?],
            provides: [$($capability:literal),* $(,)?],
            priority: $priority:literal $(,)?
        }
    ),+ $(,)?) => {
        pub(crate) fn register_compiled_services(
            registry: &mut DefaultServiceRegistry,
        ) -> Result<()> {
            let _ = &registry;

            $(
                #[cfg(feature = $feature)]
                super::register(
                    registry,
                    super::service_metadata(
                        $name,
                        $description,
                        &[$($dependency),*],
                        &[$($capability),*],
                        $priority,
                    ),
                )?;
            )*

            Ok(())
        }

        #[cfg(test)]
        pub(super) fn compiled_service_names() -> Vec<&'static str> {
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

#[path = "catalog.rs"]
mod catalog;
pub(crate) use catalog::register_compiled_services;

#[cfg(test)]
use catalog::compiled_service_names;

#[cfg(any(
    feature = "auth",
    feature = "communication",
    feature = "docs",
    feature = "cardkit",
    feature = "meeting",
    feature = "security",
    feature = "hr",
    feature = "ai",
    feature = "workflow",
    feature = "platform",
    feature = "application",
    feature = "helpdesk",
    feature = "mail",
    feature = "analytics",
    feature = "user"
))]
fn register(
    registry: &mut super::DefaultServiceRegistry,
    metadata: super::ServiceMetadata,
) -> crate::Result<()> {
    super::ServiceRegistry::register_service(registry, metadata)
        .map_err(crate::error::registry_error)
}

#[cfg(any(
    test,
    feature = "auth",
    feature = "communication",
    feature = "docs",
    feature = "cardkit",
    feature = "meeting",
    feature = "security",
    feature = "hr",
    feature = "ai",
    feature = "workflow",
    feature = "platform",
    feature = "application",
    feature = "helpdesk",
    feature = "mail",
    feature = "analytics",
    feature = "user"
))]
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

#[cfg(any(
    test,
    feature = "auth",
    feature = "communication",
    feature = "docs",
    feature = "cardkit",
    feature = "meeting",
    feature = "security",
    feature = "hr",
    feature = "ai",
    feature = "workflow",
    feature = "platform",
    feature = "application",
    feature = "helpdesk",
    feature = "mail",
    feature = "analytics",
    feature = "user"
))]
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
    fn test_compiled_service_names_reflect_enabled_features() {
        let service_names = compiled_service_names();

        macro_rules! assert_feature_service {
            ($feature:literal, $service:literal) => {
                #[cfg(feature = $feature)]
                assert!(service_names.contains(&$service));

                #[cfg(not(feature = $feature))]
                assert!(!service_names.contains(&$service));
            };
        }

        assert_feature_service!("auth", "auth");
        assert_feature_service!("communication", "communication");
        assert_feature_service!("docs", "docs");
        assert_feature_service!("cardkit", "cardkit");
        assert_feature_service!("meeting", "meeting");
        assert_feature_service!("security", "security");
        assert_feature_service!("hr", "hr");
        assert_feature_service!("ai", "ai");
        assert_feature_service!("workflow", "workflow");
        assert_feature_service!("platform", "platform");
        assert_feature_service!("application", "application");
        assert_feature_service!("helpdesk", "helpdesk");
        assert_feature_service!("mail", "mail");
        assert_feature_service!("analytics", "analytics");
        assert_feature_service!("user", "user");
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
