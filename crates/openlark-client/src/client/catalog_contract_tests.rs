//! Capability catalog 合约测试（#434–#437）
//!
//! 与 `tests.rs` 分离：公开 seam 是 `Client::registry()` 与 feature-gated 字段。

use super::super::Client;

// ===== #434–#436: compiled-capability catalog =====
// 最高运行时 seam：`Client::registry()` 与 Client 字段对 catalog 域 feature 的一致性。

macro_rules! assert_catalog_domain {
    ($client:expr, $feature:literal, $name:literal, $access:expr, $desc:expr, $deps:expr, $provides:expr, $priority:expr) => {
        #[cfg(feature = $feature)]
        {
            let _field = $access;
            assert!(
                $client.registry().has_service($name),
                "{} feature 启用时 registry 必须报告 {}",
                $name,
                $name
            );
            let entry = $client.registry().get_service($name).unwrap();
            assert_eq!(entry.metadata.name, $name);
            assert_eq!(entry.metadata.description.as_deref(), Some($desc));
            assert_eq!(
                entry.metadata.dependencies,
                $deps
                    .iter()
                    .map(|s: &&str| (*s).to_string())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                entry.metadata.provides,
                $provides
                    .iter()
                    .map(|s: &&str| (*s).to_string())
                    .collect::<Vec<_>>()
            );
            assert_eq!(entry.metadata.priority, $priority);
        }

        #[cfg(not(feature = $feature))]
        {
            assert!(
                !$client.registry().has_service($name),
                "{} feature 禁用时 registry 不得报告 {}",
                $name,
                $name
            );
        }
    };
}

#[test]
fn bot_capability_client_and_registry_agree() {
    use crate::registry::ServiceRegistry;

    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    assert_catalog_domain!(
        client,
        "bot",
        "bot",
        &client.bot,
        "飞书机器人服务，提供机器人搜索等功能",
        &["auth"],
        &["bot"],
        4
    );
}

/// #435：foundational 域启用时 Client 字段与 registry 各恰有一条；禁用时两处均无。
#[test]
fn foundational_capability_client_and_registry_agree() {
    use crate::registry::ServiceRegistry;

    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    assert_catalog_domain!(
        client,
        "auth",
        "auth",
        &client.auth,
        "飞书认证服务，提供令牌管理、身份验证等功能",
        &[] as &[&str],
        &["token-management", "permission-control"],
        1
    );
    assert_catalog_domain!(
        client,
        "communication",
        "communication",
        &client.communication,
        "飞书通讯服务，提供消息、联系人、群组等功能",
        &["auth"],
        &["im", "contacts", "groups"],
        2
    );
    assert_catalog_domain!(
        client,
        "docs",
        "docs",
        &client.docs,
        "飞书文档服务，提供云文档、表格、知识库等功能",
        &["auth"],
        &["cloud-docs", "sheets", "wiki"],
        2
    );
    assert_catalog_domain!(
        client,
        "cardkit",
        "cardkit",
        &client.cardkit,
        "飞书卡片服务，提供卡片渲染与交互能力",
        &["auth"],
        &["card"],
        3
    );
    assert_catalog_domain!(
        client,
        "meeting",
        "meeting",
        &client.meeting,
        "飞书会议服务，提供视频会议与会议室管理能力",
        &["auth"],
        &["vc"],
        3
    );
    assert_catalog_domain!(
        client,
        "security",
        "security",
        &client.security,
        "飞书安全服务，提供安全审计与风控相关能力",
        &["auth"],
        &["security"],
        3
    );
}

/// #436：剩余业务域走 catalog；AI deps 与 Cargo feature 一致（仅 auth）。
#[test]
fn remaining_capability_client_and_registry_agree() {
    use crate::registry::ServiceRegistry;

    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

    assert_catalog_domain!(
        client,
        "hr",
        "hr",
        &client.hr,
        "飞书人力资源服务，提供员工、考勤、薪酬等功能",
        &["auth"],
        &["attendance", "corehr", "ehr"],
        4
    );
    assert_catalog_domain!(
        client,
        "ai",
        "ai",
        &client.ai,
        "飞书AI服务，提供智能助手、AI分析等功能",
        &["auth"],
        &["chatbot", "smart-analysis"],
        4
    );
    assert_catalog_domain!(
        client,
        "workflow",
        "workflow",
        &client.workflow,
        "飞书工作流服务，提供审批、任务、看板等功能",
        &["auth"],
        &["approval", "task", "board"],
        4
    );
    assert_catalog_domain!(
        client,
        "platform",
        "platform",
        &client.platform,
        "飞书平台服务，提供应用平台相关功能",
        &["auth"],
        &["app-platform"],
        4
    );
    assert_catalog_domain!(
        client,
        "application",
        "application",
        &client.application,
        "飞书应用服务，提供应用管理相关功能",
        &["auth"],
        &["app-management"],
        4
    );
    assert_catalog_domain!(
        client,
        "helpdesk",
        "helpdesk",
        &client.helpdesk,
        "飞书帮助台服务，提供工单管理相关功能",
        &["auth"],
        &["ticket"],
        4
    );
    assert_catalog_domain!(
        client,
        "mail",
        "mail",
        &client.mail,
        "飞书邮件服务，提供邮件相关功能",
        &["auth"],
        &["email"],
        4
    );
    assert_catalog_domain!(
        client,
        "analytics",
        "analytics",
        &client.analytics,
        "飞书分析服务，提供数据分析相关功能",
        &["auth"],
        &["report"],
        4
    );
    assert_catalog_domain!(
        client,
        "user",
        "user",
        &client.user,
        "飞书用户服务，提供用户设置相关功能",
        &["auth"],
        &["system_status"],
        4
    );
}

/// 公开 seam：`Client::registry().list_services()` 集合与顺序。
/// 期望来自独立 feature oracle + 公开 metadata.priority，不读宏内部生成列表。
#[test]
fn registry_listing_matches_catalog_capability_set() {
    use crate::capability::expected_capability_names_from_features;
    use crate::registry::ServiceRegistry;

    let client = Client::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap();

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
