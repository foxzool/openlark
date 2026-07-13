//! Catalog 合约测试共享辅助（#434–#437）。

use super::super::Client;

/// 单域 registry 元数据期望（避免 8 元组参数散落）。
///
/// 域 feature 关闭时宏只断言 `name` 不在 registry；其余字段仍随调用方字面量保留，
/// 以便 feature 矩阵下同一调用点可编译。
#[allow(dead_code)]
pub(super) struct CatalogDomainMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub dependencies: &'static [&'static str],
    pub provides: &'static [&'static str],
    pub priority: u32,
}

/// feature-gated 域：Client 字段可达 ⇔ registry 有对应 metadata。
macro_rules! assert_catalog_domain {
    ($client:expr, $feature:literal, $access:expr, $meta:expr) => {
        #[cfg(feature = $feature)]
        {
            let meta: &super::catalog_contract_support::CatalogDomainMeta = &$meta;
            let _field = $access;
            assert!(
                $client.registry().has_service(meta.name),
                "{} feature 启用时 registry 必须报告 {}",
                meta.name,
                meta.name
            );
            let entry = $client.registry().get_service(meta.name).unwrap();
            assert_eq!(entry.metadata.name, meta.name);
            assert_eq!(
                entry.metadata.description.as_deref(),
                Some(meta.description)
            );
            assert_eq!(
                entry.metadata.dependencies,
                meta.dependencies
                    .iter()
                    .map(|s| (*s).to_string())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                entry.metadata.provides,
                meta.provides
                    .iter()
                    .map(|s| (*s).to_string())
                    .collect::<Vec<_>>()
            );
            assert_eq!(entry.metadata.priority, meta.priority);
        }

        #[cfg(not(feature = $feature))]
        {
            let meta: &super::catalog_contract_support::CatalogDomainMeta = &$meta;
            assert!(
                !$client.registry().has_service(meta.name),
                "{} feature 禁用时 registry 不得报告 {}",
                meta.name,
                meta.name
            );
        }
    };
}

pub(super) fn test_client() -> Client {
    Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap()
}
