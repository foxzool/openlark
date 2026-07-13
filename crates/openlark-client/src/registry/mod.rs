//! 服务注册与诊断查询（metadata-only）
//!
//! #423 / #437：`Client::registry()` 是编译能力的诊断 seam，只暴露 listing /
//! lookup / presence 与不可变元数据。不提供 runtime 服务实例、生命周期状态机
//! 或 typed downcast。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

pub(crate) mod bootstrap;

/// 服务注册表错误
#[derive(Error, Debug, Clone)]
pub enum RegistryError {
    /// 服务名称已存在
    #[error("服务 '{name}' 已存在")]
    ServiceAlreadyExists {
        /// 已存在的服务名称
        name: String,
    },

    /// 服务不存在
    #[error("服务 '{name}' 不存在")]
    ServiceNotFound {
        /// 不存在的服务名称
        name: String,
    },
}

/// 服务注册表结果类型
pub type RegistryResult<T> = Result<T, RegistryError>;

/// 服务诊断元数据（不可变；与 capability catalog 对齐）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    /// 服务名称（与 Cargo feature / Client 字段对应）
    pub name: String,
    /// 服务版本（元信息）
    pub version: String,
    /// 服务描述
    pub description: Option<String>,
    /// 依赖的服务 / feature 名（须与 Cargo feature 关系一致）
    pub dependencies: Vec<String>,
    /// 提供的能力标签
    pub provides: Vec<String>,
    /// 优先级（数值越小优先级越高）
    pub priority: u32,
}

/// 服务条目：仅包装诊断元数据
#[derive(Debug, Clone)]
pub struct ServiceEntry {
    /// 服务元数据
    pub metadata: ServiceMetadata,
}

/// 服务注册表诊断接口（只读）
///
/// 注册发生在 Client 构造期（`pub(crate)`），不经本 trait 暴露给调用方。
pub trait ServiceRegistry: Send + Sync {
    /// 按名称查找服务条目
    fn get_service(&self, name: &str) -> RegistryResult<&ServiceEntry>;

    /// 列出所有已编译服务
    fn list_services(&self) -> Vec<&ServiceEntry>;

    /// 检查服务是否存在（是否在当前 feature 组合下编译）
    fn has_service(&self, name: &str) -> bool;

    /// 依赖关系图：`name -> dependencies`
    fn get_dependency_graph(&self) -> HashMap<String, Vec<String>>;
}

/// 默认服务注册表实现
#[derive(Debug)]
pub struct DefaultServiceRegistry {
    services: HashMap<String, ServiceEntry>,
}

impl Default for DefaultServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultServiceRegistry {
    /// 创建空注册表
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// 注册服务元数据（仅 Client / catalog bootstrap 使用）
    pub(crate) fn register_service(&mut self, metadata: ServiceMetadata) -> RegistryResult<()> {
        if self.services.contains_key(&metadata.name) {
            return Err(RegistryError::ServiceAlreadyExists {
                name: metadata.name,
            });
        }

        let name = metadata.name.clone();
        self.services.insert(name, ServiceEntry { metadata });
        Ok(())
    }
}

impl ServiceRegistry for DefaultServiceRegistry {
    fn get_service(&self, name: &str) -> RegistryResult<&ServiceEntry> {
        self.services
            .get(name)
            .ok_or_else(|| RegistryError::ServiceNotFound {
                name: name.to_string(),
            })
    }

    /// 稳定顺序：priority 升序，同 priority 按 name 字典序。
    fn list_services(&self) -> Vec<&ServiceEntry> {
        let mut entries: Vec<&ServiceEntry> = self.services.values().collect();
        entries.sort_by(|a, b| {
            a.metadata
                .priority
                .cmp(&b.metadata.priority)
                .then_with(|| a.metadata.name.cmp(&b.metadata.name))
        });
        entries
    }

    fn has_service(&self, name: &str) -> bool {
        self.services.contains_key(name)
    }

    fn get_dependency_graph(&self) -> HashMap<String, Vec<String>> {
        self.services
            .iter()
            .map(|(name, entry)| (name.clone(), entry.metadata.dependencies.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_metadata(name: &str) -> ServiceMetadata {
        ServiceMetadata {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            description: Some("测试服务".to_string()),
            dependencies: vec![],
            provides: vec!["test-feature".to_string()],
            priority: 1,
        }
    }

    #[test]
    fn test_service_registration_and_lookup() {
        let mut registry = DefaultServiceRegistry::new();
        registry.register_service(sample_metadata("test-service")).unwrap();

        assert!(registry.has_service("test-service"));
        let entry = registry.get_service("test-service").unwrap();
        assert_eq!(entry.metadata.name, "test-service");
        assert_eq!(entry.metadata.priority, 1);
        assert_eq!(registry.list_services().len(), 1);
    }

    #[test]
    fn test_duplicate_registration() {
        let mut registry = DefaultServiceRegistry::new();
        registry.register_service(sample_metadata("test-service")).unwrap();

        let result = registry.register_service(sample_metadata("test-service"));
        assert!(matches!(
            result,
            Err(RegistryError::ServiceAlreadyExists { .. })
        ));
    }

    #[test]
    fn test_missing_service() {
        let registry = DefaultServiceRegistry::new();
        assert!(!registry.has_service("missing"));
        assert!(matches!(
            registry.get_service("missing"),
            Err(RegistryError::ServiceNotFound { .. })
        ));
    }

    #[test]
    fn test_dependency_graph() {
        let mut registry = DefaultServiceRegistry::new();
        registry
            .register_service(ServiceMetadata {
                name: "comm".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                dependencies: vec!["auth".to_string()],
                provides: vec!["im".to_string()],
                priority: 2,
            })
            .unwrap();

        let graph = registry.get_dependency_graph();
        assert_eq!(graph.get("comm").unwrap(), &vec!["auth".to_string()]);
    }

    #[test]
    fn list_services_stable_order_by_priority_then_name() {
        let mut registry = DefaultServiceRegistry::new();
        for (name, priority) in [("zeta", 2u32), ("alpha", 1), ("beta", 1), ("mid", 2)] {
            registry
                .register_service(ServiceMetadata {
                    name: name.to_string(),
                    version: "1.0.0".to_string(),
                    description: None,
                    dependencies: vec![],
                    provides: vec![],
                    priority,
                })
                .unwrap();
        }

        let names: Vec<&str> = registry
            .list_services()
            .into_iter()
            .map(|e| e.metadata.name.as_str())
            .collect();
        assert_eq!(names, vec!["alpha", "beta", "mid", "zeta"]);
    }
}
