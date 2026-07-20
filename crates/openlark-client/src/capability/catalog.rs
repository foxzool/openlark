//! 编译期能力目录（单一事实来源）
//!
//! #434–#436：全部业务域 Client 字段与 registry 诊断元数据均由本目录生成。
//! 统一声明同时包含：
//! - Client 构造：`feature` / `field` / `ty` / `doc` / `init`
//! - 诊断元数据：`name` / `description` / `dependencies` / `provides` / `priority`
//!
//! 两个 callback 各自只消费一半字段（另一半为死匹配），以保证「单源」而非「两份声明」。
//!
//! `dependencies` 必须与 `openlark-client` Cargo feature 关系一致（#423 / #436）。

/// 对每个已声明的编译期能力调用 `$callback! { ...entries }`。
///
/// 现有 callback：
/// - `generate_catalog_registry` — registry 诊断投影
/// - `append_catalog_entries` — 生成 `declare_client!` 的 Client 构造投影
macro_rules! for_each_compiled_capability {
    ($callback:ident) => {
        $callback! {
            // 保持 0.17 公开 Client 字段顺序（#423 明确排除未经决策的重排）。
            {
                feature: "cardkit",
                field: cardkit,
                ty: openlark_cardkit::CardkitClient,
                doc: "CardKit meta 调用链：client.cardkit.v1.card.create(...)",
                init: |_core_config, _base_core_config| {
                    openlark_cardkit::CardkitClient::new(_core_config.clone())
                },
                name: "cardkit",
                description: "飞书卡片服务，提供卡片渲染与交互能力",
                dependencies: ["auth"],
                provides: ["card"],
                priority: 3,
            },
            {
                feature: "auth",
                field: auth,
                ty: AuthClient,
                doc: "Auth meta 调用链入口：client.auth.app / client.auth.user / client.auth.oauth",
                init: |_core_config, _base_core_config| {
                    // Auth 自身构造使用 base config（token provider 尚未注入）
                    AuthClient::new(_base_core_config.clone())
                },
                name: "auth",
                description: "飞书认证服务，提供令牌管理、身份验证等功能",
                dependencies: [],
                provides: ["token-management", "permission-control"],
                priority: 1,
            },
            {
                feature: "docs",
                field: docs,
                ty: openlark_docs::DocsClient,
                doc: "Docs meta 调用链入口：client.docs.config() / 各 async helper（如 search_bitable_records_all）...",
                init: |_core_config, _base_core_config| {
                    openlark_docs::DocsClient::new(_core_config.clone())
                },
                name: "docs",
                description: "飞书文档服务，提供云文档、表格、知识库等功能",
                dependencies: ["auth"],
                provides: ["cloud-docs", "sheets", "wiki"],
                priority: 2,
            },
            {
                feature: "communication",
                field: communication,
                ty: openlark_communication::CommunicationClient,
                doc: "Communication meta 调用链入口：client.communication.im / client.communication.contact ...",
                init: |_core_config, _base_core_config| {
                    openlark_communication::CommunicationClient::new(_core_config.clone())
                },
                name: "communication",
                description: "飞书通讯服务，提供消息、联系人、群组等功能",
                dependencies: ["auth"],
                provides: ["im", "contacts", "groups"],
                priority: 2,
            },
            {
                feature: "hr",
                field: hr,
                ty: openlark_hr::HrClient,
                doc: "HR meta 调用链入口：client.hr.attendance / client.hr.corehr / client.hr.hire ...",
                init: |_core_config, _base_core_config| {
                    openlark_hr::HrClient::new(_core_config.clone())
                },
                name: "hr",
                description: "飞书人力资源服务，提供员工、考勤、薪酬等功能",
                dependencies: ["auth"],
                provides: ["attendance", "corehr", "ehr"],
                priority: 4,
            },
            {
                feature: "meeting",
                field: meeting,
                ty: openlark_meeting::MeetingClient,
                doc: "Meeting meta 调用链入口：client.meeting.vc.v1.note.get(...) 等（ADR 0001：room/meeting/reserve 空壳已砍，真实 builder 经 strict 路径）",
                init: |_core_config, _base_core_config| {
                    openlark_meeting::MeetingClient::new(_core_config.clone())
                },
                name: "meeting",
                description: "飞书会议服务，提供视频会议与会议室管理能力",
                dependencies: ["auth"],
                provides: ["vc"],
                priority: 3,
            },
            {
                feature: "ai",
                field: ai,
                ty: openlark_ai::AiClient,
                doc: "AI meta 调用链入口：client.ai.chat.create() ...",
                init: |_core_config, _base_core_config| {
                    openlark_ai::AiClient::new(_core_config.clone())
                },
                name: "ai",
                description: "飞书AI服务，提供智能助手、AI分析等功能",
                // Cargo: ai = ["auth", "dep:openlark-ai"] — 不依赖 communication feature
                dependencies: ["auth"],
                provides: ["chatbot", "smart-analysis"],
                priority: 4,
            },
            {
                feature: "workflow",
                field: workflow,
                ty: crate::WorkflowClient,
                doc: "Workflow meta 调用链入口：client.workflow.v2().task().create() ...",
                init: |_core_config, _base_core_config| {
                    crate::WorkflowClient::new(_core_config.clone())
                },
                name: "workflow",
                description: "飞书工作流服务，提供审批、任务、看板等功能",
                dependencies: ["auth"],
                provides: ["approval", "task", "board"],
                priority: 4,
            },
            {
                feature: "platform",
                field: platform,
                ty: crate::PlatformClient,
                doc: "Platform meta 调用链入口：client.platform.app_engine... ...",
                init: |_core_config, _base_core_config| {
                    crate::PlatformClient::new(_core_config.clone())
                },
                name: "platform",
                description: "飞书平台服务，提供应用平台相关功能",
                dependencies: ["auth"],
                provides: ["app-platform"],
                priority: 4,
            },
            {
                feature: "application",
                field: application,
                ty: crate::ApplicationClient,
                doc: "Application meta 调用链入口：client.application.applet... ...",
                init: |_core_config, _base_core_config| {
                    crate::ApplicationClient::new(_core_config.clone())
                },
                name: "application",
                description: "飞书应用服务，提供应用管理相关功能",
                dependencies: ["auth"],
                provides: ["app-management"],
                priority: 4,
            },
            {
                feature: "helpdesk",
                field: helpdesk,
                ty: crate::HelpdeskClient,
                doc: "Helpdesk meta 调用链入口：client.helpdesk.ticket... ...",
                init: |_core_config, _base_core_config| {
                    crate::HelpdeskClient::new(_core_config.clone())
                },
                name: "helpdesk",
                description: "飞书帮助台服务，提供工单管理相关功能",
                dependencies: ["auth"],
                provides: ["ticket"],
                priority: 4,
            },
            {
                feature: "mail",
                field: mail,
                ty: crate::MailClient,
                doc: "Mail meta 调用链入口：client.mail.group... ...",
                init: |_core_config, _base_core_config| {
                    crate::MailClient::new(_core_config.clone())
                },
                name: "mail",
                description: "飞书邮件服务，提供邮件相关功能",
                dependencies: ["auth"],
                provides: ["email"],
                priority: 4,
            },
            {
                feature: "analytics",
                field: analytics,
                ty: crate::AnalyticsClient,
                doc: "Analytics meta 调用链入口：client.analytics.report... ...",
                init: |_core_config, _base_core_config| {
                    crate::AnalyticsClient::new(_core_config.clone())
                },
                name: "analytics",
                description: "飞书分析服务，提供数据分析相关功能",
                dependencies: ["auth"],
                provides: ["report"],
                priority: 4,
            },
            {
                feature: "user",
                field: user,
                ty: crate::UserClient,
                doc: "User meta 调用链入口：client.user.system_status... ...",
                init: |_core_config, _base_core_config| {
                    crate::UserClient::new(_core_config.clone())
                },
                name: "user",
                description: "飞书用户服务，提供用户设置相关功能",
                dependencies: ["auth"],
                provides: ["system_status"],
                priority: 4,
            },
            {
                feature: "security",
                field: security,
                ty: crate::SecurityClient,
                doc: "Security meta 调用链入口：client.security.acs... ...",
                init: |_core_config, _base_core_config| {
                    // 使用 canonical core Config 直传（#444/#447 收口），完整保留 token_provider / headers 等。
                    openlark_security::SecurityClient::from_config(_core_config.clone())
                },
                name: "security",
                description: "飞书安全服务，提供安全审计与风控相关能力",
                dependencies: ["auth"],
                provides: ["security"],
                priority: 3,
            },
            {
                feature: "bot",
                field: bot,
                ty: openlark_bot::BotClient,
                doc: "Bot meta 调用链入口：client.bot.search_bot() ...",
                init: |_core_config, _base_core_config| {
                    openlark_bot::BotClient::new(_core_config.clone())
                },
                name: "bot",
                description: "飞书机器人服务，提供机器人搜索等功能",
                dependencies: ["auth"],
                provides: ["bot"],
                priority: 4,
            },
        }
    };
}

pub(crate) use for_each_compiled_capability;

// 由统一声明生成 registry 注册与测试辅助
for_each_compiled_capability!(generate_catalog_registry);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{DefaultServiceRegistry, ServiceRegistry};

    #[test]
    fn catalog_names_reflect_enabled_features() {
        // 生成结果 vs 独立 feature oracle（不互为唯一来源）
        let generated = super::catalog_capability_names();
        let expected = crate::capability::expected_capability_names_from_features();
        assert_eq!(
            generated, expected,
            "catalog 生成名必须与独立 feature oracle 一致（含声明顺序）"
        );
    }

    #[test]
    fn register_catalog_capabilities_matches_features() {
        let mut registry = DefaultServiceRegistry::new();
        register_catalog_capabilities(&mut registry).unwrap();

        let expected = crate::capability::expected_capability_names_from_features();
        assert_eq!(registry.list_services().len(), expected.len());
        for name in &expected {
            assert!(
                registry.has_service(name),
                "feature 启用时 registry 应注册 {name}"
            );
        }

        // 已知关闭域：oracle 不含则不得注册
        #[cfg(not(feature = "hr"))]
        assert!(!registry.has_service("hr"));
        #[cfg(not(feature = "bot"))]
        assert!(!registry.has_service("bot"));

        #[cfg(feature = "auth")]
        {
            let entry = registry.get_service("auth").unwrap();
            assert_eq!(
                entry.metadata.description.as_deref(),
                Some("飞书认证服务，提供令牌管理、身份验证等功能")
            );
            assert!(entry.metadata.dependencies.is_empty());
            assert_eq!(
                entry.metadata.provides,
                vec![
                    "token-management".to_string(),
                    "permission-control".to_string()
                ]
            );
            assert_eq!(entry.metadata.priority, 1);
        }

        // #436：AI deps 与 Cargo `ai = ["auth", ...]` 一致（不再误报 communication）
        #[cfg(feature = "ai")]
        {
            let entry = registry.get_service("ai").unwrap();
            assert_eq!(
                entry.metadata.dependencies,
                vec!["auth".to_string()],
                "ai catalog dependencies 必须与 Cargo feature 关系一致"
            );
            assert!(
                !entry
                    .metadata
                    .dependencies
                    .iter()
                    .any(|d| d == "communication")
            );
            assert_eq!(
                entry.metadata.provides,
                vec!["chatbot".to_string(), "smart-analysis".to_string()]
            );
        }

        #[cfg(feature = "bot")]
        {
            let entry = registry.get_service("bot").unwrap();
            assert_eq!(
                entry.metadata.description.as_deref(),
                Some("飞书机器人服务，提供机器人搜索等功能")
            );
            assert_eq!(entry.metadata.dependencies, vec!["auth".to_string()]);
            assert_eq!(entry.metadata.provides, vec!["bot".to_string()]);
            assert_eq!(entry.metadata.priority, 4);
        }
    }
}
