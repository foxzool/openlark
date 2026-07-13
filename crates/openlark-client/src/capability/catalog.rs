//! 编译期能力目录（单一事实来源）
//!
//! #434 tracer `bot`；#435 迁入 foundational 域：
//! `auth` / `communication` / `docs` / `cardkit` / `meeting` / `security`。
//! 其余业务域仍使用 `declare_client!` + `registry/catalog.rs` 双声明旧路径
//!（见 #436）。
//!
//! 统一声明同时包含：
//! - Client 构造：`feature` / `field` / `ty` / `doc` / `init`
//! - 诊断元数据：`name` / `description` / `dependencies` / `provides` / `priority`
//!
//! 两个 callback 各自只消费一半字段（另一半为死匹配），以保证「单源」而非「两份声明」。

/// 对每个已声明的编译期能力调用 `$callback! { ...entries }`。
///
/// 现有 callback：
/// - `generate_catalog_registry` — registry 诊断投影
/// - `append_catalog_entries` — 追加进 `declare_client!` 的 Client 构造投影
macro_rules! for_each_compiled_capability {
    ($callback:ident) => {
        $callback! {
            // --- foundational（#435）---
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
                feature: "security",
                field: security,
                ty: crate::SecurityClient,
                doc: "Security meta 调用链入口：client.security.acs... ...",
                init: |_core_config, _base_core_config| {
                    let security_config = openlark_security::config::SecurityConfig::new(
                        _core_config.app_id().to_string(),
                        _core_config.app_secret().to_string(),
                    )
                    .with_base_url(_core_config.base_url());
                    openlark_security::SecurityClient::new(security_config)
                },
                name: "security",
                description: "飞书安全服务，提供安全审计与风控相关能力",
                dependencies: ["auth"],
                provides: ["security"],
                priority: 3,
            },
            // --- tracer（#434）---
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
        let names = catalog_capability_names();

        // 精确集合：仅 cfg 启用的 feature，顺序与 catalog 声明一致
        let expected: Vec<&str> = [
            None::<&str>,
            #[cfg(feature = "auth")]
            Some("auth"),
            #[cfg(feature = "communication")]
            Some("communication"),
            #[cfg(feature = "docs")]
            Some("docs"),
            #[cfg(feature = "cardkit")]
            Some("cardkit"),
            #[cfg(feature = "meeting")]
            Some("meeting"),
            #[cfg(feature = "security")]
            Some("security"),
            #[cfg(feature = "bot")]
            Some("bot"),
        ]
        .into_iter()
        .flatten()
        .collect();

        assert_eq!(
            names, expected,
            "catalog names 必须与启用 feature 的精确集合一致（含顺序）"
        );
    }

    #[test]
    fn register_catalog_capabilities_matches_features() {
        let mut registry = DefaultServiceRegistry::new();
        register_catalog_capabilities(&mut registry).unwrap();

        macro_rules! assert_registered {
            ($feature:literal, $name:literal) => {
                #[cfg(feature = $feature)]
                assert!(
                    registry.has_service($name),
                    "feature {} 启用时 registry 应注册 {}",
                    $feature,
                    $name
                );

                #[cfg(not(feature = $feature))]
                assert!(
                    !registry.has_service($name),
                    "feature {} 禁用时 registry 不得注册 {}",
                    $feature,
                    $name
                );
            };
        }

        assert_registered!("auth", "auth");
        assert_registered!("communication", "communication");
        assert_registered!("docs", "docs");
        assert_registered!("cardkit", "cardkit");
        assert_registered!("meeting", "meeting");
        assert_registered!("security", "security");
        assert_registered!("bot", "bot");

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
            assert!(entry.instance.is_none());
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
            assert!(entry.instance.is_none());
        }
    }
}
