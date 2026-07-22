//! 编译期能力目录（单一事实来源）
//!
//! 全部业务域的 Client 字段均由本目录生成（#434–#437）。统一声明为
//! `feature` / `field` / `ty` / `doc` / `init` 五元组，由 `append_catalog_entries!`
//! 投影成 `declare_client!` 的 Client 构造字段。
//!
//! `dependencies` 关系由 Cargo `[features]` 表达；本目录不再维护 registry
//! 诊断元数据（#471 移除 registry 半边）。

/// 对每个已声明的编译期能力调用 `$callback! { ...entries }`。
///
/// 唯一 callback：`append_catalog_entries` — 生成 `declare_client!` 的 Client 构造投影。
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
            },
            {
                feature: "docs",
                field: docs,
                ty: openlark_docs::DocsClient,
                doc: "Docs meta 调用链入口：client.docs.config() / 各 async helper（如 search_bitable_records_all）...",
                init: |_core_config, _base_core_config| {
                    openlark_docs::DocsClient::new(_core_config.clone())
                },
            },
            {
                feature: "communication",
                field: communication,
                ty: openlark_communication::CommunicationClient,
                doc: "Communication meta 调用链入口：client.communication.im / client.communication.contact ...",
                init: |_core_config, _base_core_config| {
                    openlark_communication::CommunicationClient::new(_core_config.clone())
                },
            },
            {
                feature: "hr",
                field: hr,
                ty: openlark_hr::HrClient,
                doc: "HR 入口：client.hr.config() 直达 leaf 构造 Request（7 域 config-holder facade 已砍，#474），client.hr.okr.v2() 保留 fluent",
                init: |_core_config, _base_core_config| {
                    openlark_hr::HrClient::new(_core_config.clone())
                },
            },
            {
                feature: "meeting",
                field: meeting,
                ty: openlark_meeting::MeetingClient,
                doc: "Meeting meta 调用链入口：client.meeting.vc.v1.note.get(...) 等（ADR 0001：room/meeting/reserve 空壳已砍，真实 builder 经 strict 路径）",
                init: |_core_config, _base_core_config| {
                    openlark_meeting::MeetingClient::new(_core_config.clone())
                },
            },
            {
                feature: "ai",
                field: ai,
                ty: openlark_ai::AiClient,
                doc: "AI meta 调用链入口：client.ai.chat.create() ...",
                init: |_core_config, _base_core_config| {
                    openlark_ai::AiClient::new(_core_config.clone())
                },
            },
            {
                feature: "workflow",
                field: workflow,
                ty: crate::WorkflowClient,
                doc: "Workflow meta 调用链入口：client.workflow.v2().task().create() ...",
                init: |_core_config, _base_core_config| {
                    crate::WorkflowClient::new(_core_config.clone())
                },
            },
            {
                feature: "platform",
                field: platform,
                ty: crate::PlatformClient,
                doc: "Platform meta 调用链入口：client.platform.app_engine... ...",
                init: |_core_config, _base_core_config| {
                    crate::PlatformClient::new(_core_config.clone())
                },
            },
            {
                feature: "application",
                field: application,
                ty: crate::ApplicationClient,
                doc: "Application meta 调用链入口：client.application.applet... ...",
                init: |_core_config, _base_core_config| {
                    crate::ApplicationClient::new(_core_config.clone())
                },
            },
            {
                feature: "helpdesk",
                field: helpdesk,
                ty: crate::HelpdeskClient,
                doc: "Helpdesk meta 调用链入口：client.helpdesk.ticket... ...",
                init: |_core_config, _base_core_config| {
                    crate::HelpdeskClient::new(_core_config.clone())
                },
            },
            {
                feature: "mail",
                field: mail,
                ty: crate::MailClient,
                doc: "Mail meta 调用链入口：client.mail.group... ...",
                init: |_core_config, _base_core_config| {
                    crate::MailClient::new(_core_config.clone())
                },
            },
            {
                feature: "analytics",
                field: analytics,
                ty: crate::AnalyticsClient,
                doc: "Analytics meta 调用链入口：client.analytics.report... ...",
                init: |_core_config, _base_core_config| {
                    crate::AnalyticsClient::new(_core_config.clone())
                },
            },
            {
                feature: "user",
                field: user,
                ty: crate::UserClient,
                doc: "User meta 调用链入口：client.user.system_status... ...",
                init: |_core_config, _base_core_config| {
                    crate::UserClient::new(_core_config.clone())
                },
            },
            {
                feature: "security",
                field: security,
                ty: crate::SecurityClient,
                doc: "Security meta 调用链入口：client.security.acs... ...",
                init: |_core_config, _base_core_config| {
                    // 使用 canonical core Config 直传（#444/#447 收口），完整保留 token_provider / headers / timeout 等。
                    // 遵循命名规范使用 new(config)。
                    openlark_security::SecurityClient::new(_core_config.clone())
                },
            },
            {
                feature: "bot",
                field: bot,
                ty: openlark_bot::BotClient,
                doc: "Bot meta 调用链入口：client.bot.search_bot() ...",
                init: |_core_config, _base_core_config| {
                    openlark_bot::BotClient::new(_core_config.clone())
                },
            },
        }
    };
}

pub(crate) use for_each_compiled_capability;

/// 投影 callback：提取 `feature` + `field` 调用 [`assert_capability_catalog_unique!`]
/// （生成期字段去重 + feature↔field 不漂移；#471 review P1）。
macro_rules! assert_catalog_fields_unique {
    ($({
        feature: $feature:literal,
        field: $field:ident,
        ty: $_ty:ty,
        doc: $_doc:literal,
        init: |$_core:ident, $_base:ident| $_init:block $(,)?
    }),* $(,)?) => {
        assert_capability_catalog_unique! {
            $( { feature: $feature, field: $field }, )*
        }
    };
}

// 生成期校验全部 catalog 条目的字段标识符唯一（#423 / #455 / #471）。
for_each_compiled_capability!(assert_catalog_fields_unique);
