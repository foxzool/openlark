//! Catalog 公开 listing / bot 域合约。

use super::catalog_contract_support::{CatalogDomainMeta, test_client};
use crate::registry::ServiceRegistry;

#[test]
fn bot_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "bot",
        &client.bot,
        CatalogDomainMeta {
            name: "bot",
            description: "飞书机器人服务，提供机器人搜索等功能",
            dependencies: &["auth"],
            provides: &["bot"],
            priority: 4,
        }
    );
}

/// 公开 seam：`Client::registry().list_services()` 集合与顺序。
/// 期望来自独立 feature oracle + 公开 metadata.priority，不读宏内部生成列表。
#[test]
fn registry_listing_matches_catalog_capability_set() {
    use crate::capability::expected_capability_names_from_features;

    let client = test_client();

    let listed: Vec<&str> = client
        .registry()
        .list_services()
        .into_iter()
        .map(|e| e.metadata.name.as_str())
        .collect();

    let mut expected = expected_capability_names_from_features();
    expected.sort_by(|a, b| {
        let pa = client.registry().get_service(a).unwrap().metadata.priority;
        let pb = client.registry().get_service(b).unwrap().metadata.priority;
        pa.cmp(&pb).then_with(|| a.cmp(b))
    });

    assert_eq!(
        listed, expected,
        "registry listing 必须与独立 feature oracle 一致且顺序稳定"
    );
}
