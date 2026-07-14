//! #436：剩余业务域 Client 字段与 registry 一致性（按域拆分；AI deps 仅 auth）。

use super::catalog_contract_support::{CatalogDomainMeta, test_client};
use crate::registry::ServiceRegistry;

#[test]
fn hr_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "hr",
        &client.hr,
        CatalogDomainMeta {
            name: "hr",
            description: "飞书人力资源服务，提供员工、考勤、薪酬等功能",
            dependencies: &["auth"],
            provides: &["attendance", "corehr", "ehr"],
            priority: 4,
        }
    );
}

#[test]
fn ai_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "ai",
        &client.ai,
        CatalogDomainMeta {
            name: "ai",
            description: "飞书AI服务，提供智能助手、AI分析等功能",
            dependencies: &["auth"],
            provides: &["chatbot", "smart-analysis"],
            priority: 4,
        }
    );
}

#[test]
fn workflow_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "workflow",
        &client.workflow,
        CatalogDomainMeta {
            name: "workflow",
            description: "飞书工作流服务，提供审批、任务、看板等功能",
            dependencies: &["auth"],
            provides: &["approval", "task", "board"],
            priority: 4,
        }
    );
}

#[test]
fn platform_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "platform",
        &client.platform,
        CatalogDomainMeta {
            name: "platform",
            description: "飞书平台服务，提供应用平台相关功能",
            dependencies: &["auth"],
            provides: &["app-platform"],
            priority: 4,
        }
    );
}

#[test]
fn application_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "application",
        &client.application,
        CatalogDomainMeta {
            name: "application",
            description: "飞书应用服务，提供应用管理相关功能",
            dependencies: &["auth"],
            provides: &["app-management"],
            priority: 4,
        }
    );
}

#[test]
fn helpdesk_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "helpdesk",
        &client.helpdesk,
        CatalogDomainMeta {
            name: "helpdesk",
            description: "飞书帮助台服务，提供工单管理相关功能",
            dependencies: &["auth"],
            provides: &["ticket"],
            priority: 4,
        }
    );
}

#[test]
fn mail_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "mail",
        &client.mail,
        CatalogDomainMeta {
            name: "mail",
            description: "飞书邮件服务，提供邮件相关功能",
            dependencies: &["auth"],
            provides: &["email"],
            priority: 4,
        }
    );
}

#[test]
fn analytics_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "analytics",
        &client.analytics,
        CatalogDomainMeta {
            name: "analytics",
            description: "飞书分析服务，提供数据分析相关功能",
            dependencies: &["auth"],
            provides: &["report"],
            priority: 4,
        }
    );
}

#[test]
fn user_capability_client_and_registry_agree() {
    let client = test_client();
    assert_catalog_domain!(
        client,
        "user",
        &client.user,
        CatalogDomainMeta {
            name: "user",
            description: "飞书用户服务，提供用户设置相关功能",
            dependencies: &["auth"],
            provides: &["system_status"],
            priority: 4,
        }
    );
}
