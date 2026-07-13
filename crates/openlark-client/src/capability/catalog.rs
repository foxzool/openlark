//! 编译期能力目录（单一事实来源）
//!
//! #434 tracer：目前仅 `bot` 走本目录；其余业务域仍使用
//! `declare_client!` + `registry/catalog.rs` 双声明旧路径。
//! 后续 #435 / #436 将把其余域迁入 [`for_each_compiled_capability`]。
//! 在 #435 真正扩容前不再增加新的通用宏层。
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
    fn catalog_names_reflect_bot_feature() {
        let names = catalog_capability_names();

        #[cfg(feature = "bot")]
        {
            assert_eq!(names, vec!["bot"]);
        }

        #[cfg(not(feature = "bot"))]
        {
            assert!(names.is_empty());
        }
    }

    #[test]
    fn register_catalog_capabilities_matches_bot_feature() {
        let mut registry = DefaultServiceRegistry::new();
        register_catalog_capabilities(&mut registry).unwrap();

        #[cfg(feature = "bot")]
        {
            assert!(registry.has_service("bot"));
            let entry = registry.get_service("bot").unwrap();
            assert_eq!(entry.metadata.name, "bot");
            assert_eq!(
                entry.metadata.description.as_deref(),
                Some("飞书机器人服务，提供机器人搜索等功能")
            );
            assert_eq!(entry.metadata.dependencies, vec!["auth".to_string()]);
            assert_eq!(entry.metadata.provides, vec!["bot".to_string()]);
            assert_eq!(entry.metadata.priority, 4);
            assert!(entry.instance.is_none());
        }

        #[cfg(not(feature = "bot"))]
        {
            assert!(!registry.has_service("bot"));
        }
    }
}
