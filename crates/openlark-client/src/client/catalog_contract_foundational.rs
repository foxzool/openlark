//! #435：foundational 域 Client 字段与 registry 一致性（按域拆分）。

use super::catalog_contract_support::{CatalogDomainMeta, test_client};
use crate::registry::ServiceRegistry;

#[test]
fn auth_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "auth",
        &client.auth,
        CatalogDomainMeta {
            name: "auth",
            description: "飞书认证服务，提供令牌管理、身份验证等功能",
            dependencies: &[],
            provides: &["token-management", "permission-control"],
            priority: 1,
        }
    );
}

#[test]
fn communication_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "communication",
        &client.communication,
        CatalogDomainMeta {
            name: "communication",
            description: "飞书通讯服务，提供消息、联系人、群组等功能",
            dependencies: &["auth"],
            provides: &["im", "contacts", "groups"],
            priority: 2,
        }
    );
}

#[test]
fn docs_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "docs",
        &client.docs,
        CatalogDomainMeta {
            name: "docs",
            description: "飞书文档服务，提供云文档、表格、知识库等功能",
            dependencies: &["auth"],
            provides: &["cloud-docs", "sheets", "wiki"],
            priority: 2,
        }
    );
}

#[test]
fn cardkit_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "cardkit",
        &client.cardkit,
        CatalogDomainMeta {
            name: "cardkit",
            description: "飞书卡片服务，提供卡片渲染与交互能力",
            dependencies: &["auth"],
            provides: &["card"],
            priority: 3,
        }
    );
}

#[test]
fn meeting_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "meeting",
        &client.meeting,
        CatalogDomainMeta {
            name: "meeting",
            description: "飞书会议服务，提供视频会议与会议室管理能力",
            dependencies: &["auth"],
            provides: &["vc"],
            priority: 3,
        }
    );
}

#[test]
fn security_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "security",
        &client.security,
        CatalogDomainMeta {
            name: "security",
            description: "飞书安全服务，提供安全审计与风控相关能力",
            dependencies: &["auth"],
            provides: &["security"],
            priority: 3,
        }
    );
}
