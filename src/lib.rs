//! OpenLark 官方入口 crate。
//!
//! 普通用户应优先使用 `openlark`，通过业务 feature 开启所需能力：
//!
//! ```toml
//! [dependencies]
//! openlark = { version = "0.16.0", default-features = false, features = ["auth", "docs-drive", "docs-bitable", "webhook-signature"] }
//! ```
//!
//! - 统一客户端入口：[`Client`]
//! - 高级业务模块入口：[`auth`]、[`communication`]、[`docs`]、[`workflow`] 等
//! - 统一预导出：[`prelude`]
//!
//! 若只想要单一业务域的最小依赖，再直接使用 `openlark-{domain}` 子 crate。
//!
//! Canonical public API 入口规则见 `docs/PUBLIC_REEXPORT_POLICY.md`。
//!
//! 推荐顺序：
//!
//! - 运行时入口：[`Client`] / [`ClientBuilder`]
//! - 导入入口：[`prelude`]
//! - 业务命名空间：`open_lark::auth`、`open_lark::communication`、`open_lark::docs`
//! - 最小依赖场景：直接依赖对应 `openlark-{domain}` crate
//!
//! 顶层 `AuthClient`、`DocsClient`、`HrClient` 等类型 re-export 会继续保留，
//! 但它们的定位是 compatibility alias，而不是普通用户的主入口。
//!
//! 根 crate `prelude` 有意不暴露客户端实现层细节：
//!
//! ```rust,compile_fail
//! use open_lark::prelude::*;
//!
//! let _registry: ServiceRegistry;
//! ```

// 允许测试模块中的未使用导入（测试桩代码常见模式）
#![allow(unused_imports)]

// 原始 crate-name passthrough re-export 仅用于兼容历史路径，
// 不再作为根 crate 的推荐公开入口出现在文档中。
#[deprecated(
    since = "0.15.0",
    note = "Use open_lark::{Client, ClientBuilder, prelude} or depend on openlark-client directly; open_lark::openlark_client is a legacy compatibility entrypoint."
)]
#[doc(hidden)]
pub use openlark_client;
pub use openlark_client::{Client, ClientBuilder, Config, Error, Result};
#[deprecated(
    since = "0.15.0",
    note = "Use the root common types (CoreConfig, CoreError, RequestOption, SDKResult) or depend on openlark-core directly; open_lark::openlark_core is a legacy compatibility entrypoint."
)]
#[doc(hidden)]
pub use openlark_core;
#[deprecated(
    since = "0.15.0",
    note = "Use the root common types (CoreConfig, CoreError, RequestOption, SDKResult) or depend on openlark-core directly; open_lark::core is a legacy compatibility entrypoint."
)]
#[doc(hidden)]
pub use openlark_core as core;
pub use openlark_core::SDKResult;
pub use openlark_core::config::Config as CoreConfig;
pub use openlark_core::error::{CoreError, ErrorCode, ErrorSeverity, ErrorTrait, ErrorType};
pub use openlark_core::req_option::RequestOption;

#[cfg(feature = "websocket")]
/// WebSocket 客户端相关类型导出。
pub mod ws_client {
    pub use openlark_client::ws_client::*;
}

#[cfg(feature = "auth")]
#[deprecated(
    since = "0.15.0",
    note = "Use open_lark::auth or Client.auth for runtime access, or depend on the auth crate directly; open_lark::openlark_auth is a legacy compatibility entrypoint."
)]
#[doc(hidden)]
pub use openlark_auth;

#[cfg(feature = "auth")]
pub use openlark_auth as auth;

#[cfg(feature = "communication")]
#[deprecated(
    since = "0.15.0",
    note = "Use open_lark::communication or Client.communication for runtime access, or depend on the communication crate directly; open_lark::openlark_communication is a legacy compatibility entrypoint."
)]
#[doc(hidden)]
pub use openlark_communication;

#[cfg(feature = "communication")]
pub use openlark_communication as communication;

#[cfg(any(
    feature = "docs",
    feature = "docs-ccm",
    feature = "docs-base",
    feature = "docs-bitable",
    feature = "docs-drive",
    feature = "docs-explorer",
    feature = "docs-sheets",
    feature = "docs-sheets-v2",
    feature = "docs-sheets-v3",
    feature = "docs-full"
))]
#[deprecated(
    since = "0.15.0",
    note = "Use open_lark::docs or Client.docs for runtime access, or depend on the docs crate directly; open_lark::openlark_docs is a legacy compatibility entrypoint."
)]
#[doc(hidden)]
pub use openlark_docs;

#[cfg(any(
    feature = "docs",
    feature = "docs-ccm",
    feature = "docs-base",
    feature = "docs-bitable",
    feature = "docs-drive",
    feature = "docs-explorer",
    feature = "docs-sheets",
    feature = "docs-sheets-v2",
    feature = "docs-sheets-v3",
    feature = "docs-full"
))]
pub use openlark_docs as docs;

#[cfg(feature = "hr")]
pub use openlark_hr as hr;

#[cfg(feature = "ai")]
pub use openlark_ai as ai;

#[cfg(feature = "helpdesk")]
pub use openlark_helpdesk as helpdesk;

#[cfg(feature = "mail")]
pub use openlark_mail as mail;

#[cfg(feature = "meeting")]
pub use openlark_meeting as meeting;

#[cfg(feature = "application")]
pub use openlark_application as application;

#[cfg(feature = "security")]
pub use openlark_security as security;

#[cfg(feature = "workflow")]
pub use openlark_workflow as workflow;

#[cfg(feature = "platform")]
pub use openlark_platform as platform;

#[cfg(feature = "analytics")]
pub use openlark_analytics as analytics;

#[cfg(feature = "user")]
pub use openlark_user as user;

#[cfg(feature = "webhook")]
#[deprecated(
    since = "0.15.0",
    note = "Use open_lark::webhook or depend on openlark-webhook directly; open_lark::openlark_webhook is a legacy compatibility entrypoint."
)]
#[doc(hidden)]
pub use openlark_webhook;

#[cfg(feature = "webhook")]
pub use openlark_webhook as webhook;

#[cfg(feature = "cardkit")]
pub use openlark_cardkit as cardkit;

// 顶层 meta client 类型 re-export 保留为兼容层；
// 普通 `openlark` 用户优先通过 `Client` 字段访问业务能力。
#[cfg(feature = "auth")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.auth for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::AuthClient;

#[cfg(any(
    feature = "docs",
    feature = "docs-ccm",
    feature = "docs-base",
    feature = "docs-bitable",
    feature = "docs-drive",
    feature = "docs-explorer",
    feature = "docs-sheets",
    feature = "docs-sheets-v2",
    feature = "docs-sheets-v3",
    feature = "docs-full"
))]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.docs for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::DocsClient;

#[cfg(feature = "communication")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.communication for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::CommunicationClient;

#[cfg(feature = "hr")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.hr for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::HrClient;

#[cfg(feature = "meeting")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.meeting for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::MeetingClient;

#[cfg(feature = "cardkit")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.cardkit for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::CardkitClient;

#[cfg(feature = "ai")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.ai for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::AiClient;

#[cfg(feature = "workflow")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.workflow for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::WorkflowClient;

#[cfg(feature = "platform")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.platform for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::PlatformClient;

#[cfg(feature = "application")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.application for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::ApplicationClient;

#[cfg(feature = "helpdesk")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.helpdesk for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::HelpdeskClient;

#[cfg(feature = "mail")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.mail for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::MailClient;

#[cfg(feature = "analytics")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.analytics for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::AnalyticsClient;

#[cfg(feature = "user")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.user for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::UserClient;

#[cfg(feature = "security")]
#[deprecated(
    since = "0.15.0",
    note = "Use Client.security for runtime access, or use openlark-client / business-crate paths for explicit types; root-level *Client aliases are legacy compatibility entrypoints."
)]
#[doc(hidden)]
pub use openlark_client::SecurityClient;

/// 面向 `openlark` 用户的统一预导出。
///
/// 该模块只导出“创建客户端 + 顶层业务入口”所需的稳定公共类型。
/// registry / feature loader / traits 等高级客户端层能力保留在 `openlark-client`。
#[allow(deprecated)]
pub mod prelude {
    #[cfg(feature = "ai")]
    #[doc(hidden)]
    pub use crate::AiClient;
    #[cfg(feature = "analytics")]
    #[doc(hidden)]
    pub use crate::AnalyticsClient;
    #[cfg(feature = "application")]
    #[doc(hidden)]
    pub use crate::ApplicationClient;
    #[cfg(feature = "auth")]
    #[doc(hidden)]
    pub use crate::AuthClient;
    #[cfg(feature = "communication")]
    #[doc(hidden)]
    pub use crate::CommunicationClient;
    #[cfg(any(
        feature = "docs",
        feature = "docs-ccm",
        feature = "docs-base",
        feature = "docs-bitable",
        feature = "docs-drive",
        feature = "docs-explorer",
        feature = "docs-sheets",
        feature = "docs-sheets-v2",
        feature = "docs-sheets-v3",
        feature = "docs-full"
    ))]
    #[doc(hidden)]
    pub use crate::DocsClient;
    #[cfg(feature = "helpdesk")]
    #[doc(hidden)]
    pub use crate::HelpdeskClient;
    #[cfg(feature = "hr")]
    #[doc(hidden)]
    pub use crate::HrClient;
    #[cfg(feature = "mail")]
    #[doc(hidden)]
    pub use crate::MailClient;
    #[cfg(feature = "meeting")]
    #[doc(hidden)]
    pub use crate::MeetingClient;
    #[cfg(feature = "platform")]
    #[doc(hidden)]
    pub use crate::PlatformClient;
    pub use crate::SDKResult;
    #[cfg(feature = "security")]
    #[doc(hidden)]
    pub use crate::SecurityClient;
    #[cfg(feature = "user")]
    #[doc(hidden)]
    pub use crate::UserClient;
    #[cfg(feature = "workflow")]
    #[doc(hidden)]
    pub use crate::WorkflowClient;
    pub use crate::{Client, ClientBuilder, Config, CoreConfig, Error, Result};
    pub use crate::{CoreError, ErrorCode, ErrorSeverity, ErrorTrait, ErrorType, RequestOption};
    pub use openlark_core::prelude::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_client() -> Result<Client> {
        Client::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn root_prelude_exposes_canonical_core_entrypoints() {
        use crate::prelude::*;

        let _builder: ClientBuilder = Client::builder();
        let _config: Config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
            .expect("test config should build");
        let _request_option: Option<RequestOption> = None;
    }

    #[test]
    fn root_minimal_builder_works_without_service_features() {
        let client = build_test_client().expect("client should build with minimal features");
        assert_eq!(client.config().app_id, "test_app");
    }

    #[cfg(feature = "auth")]
    #[test]
    #[allow(deprecated)]
    fn root_client_exposes_auth_entrypoint() {
        let client = build_test_client().expect("client should build with auth feature");

        let _auth = &client.auth;
        let alias = std::any::type_name::<crate::AuthClient>();
        let compat = std::any::type_name::<openlark_client::AuthClient>();

        assert_eq!(alias, compat);
    }

    #[cfg(feature = "communication")]
    #[test]
    fn root_client_exposes_communication_namespace() {
        let client = build_test_client().expect("client should build with communication feature");

        let _communication = &client.communication;
        let _endpoint = crate::communication::endpoints::IM_V1_MESSAGES;
    }

    #[cfg(any(
        feature = "docs",
        feature = "docs-ccm",
        feature = "docs-base",
        feature = "docs-bitable",
        feature = "docs-drive",
        feature = "docs-explorer",
        feature = "docs-sheets",
        feature = "docs-sheets-v2",
        feature = "docs-sheets-v3",
        feature = "docs-full"
    ))]
    #[test]
    #[allow(deprecated)]
    fn root_client_exposes_docs_namespace() {
        let client = build_test_client().expect("client should build with docs feature");

        let _docs = &client.docs;
        let alias = std::any::type_name::<crate::DocsClient>();
        let compat = std::any::type_name::<openlark_client::DocsClient>();

        assert_eq!(alias, compat);
    }

    #[cfg(feature = "hr")]
    #[test]
    #[allow(deprecated)]
    fn root_client_exposes_hr_entrypoint() {
        let client = build_test_client().expect("client should build with hr feature");

        let _hr = &client.hr;
        let alias = std::any::type_name::<crate::HrClient>();
        let compat = std::any::type_name::<openlark_client::HrClient>();

        assert_eq!(alias, compat);
    }

    #[cfg(feature = "security")]
    #[test]
    #[allow(deprecated)]
    fn root_client_exposes_security_entrypoint() {
        let client = build_test_client().expect("client should build with security feature");

        let _security = &client.security;
        let alias = std::any::type_name::<crate::SecurityClient>();
        let compat = std::any::type_name::<openlark_client::SecurityClient>();

        assert_eq!(alias, compat);
    }

    #[cfg(feature = "docs-bitable")]
    #[test]
    fn root_docs_bitable_feature_exposes_query_helper() {
        let query = crate::docs::BitableRecordQuery::new("app_token", "table_id")
            .where_equals("状态", "进行中");

        assert_eq!(query.app_token, "app_token");
        assert_eq!(query.table_id, "table_id");
    }

    #[cfg(feature = "docs-drive")]
    #[test]
    fn root_docs_drive_feature_exposes_drive_helpers() {
        let upload = crate::docs::DriveUploadFile::new("demo.txt", vec![1, 2, 3]);
        let range = crate::docs::DriveDownloadRange::from_start(0).with_end(9);

        assert_eq!(upload.file_name, "demo.txt");
        assert_eq!(upload.size(), 3);
        assert_eq!(range.to_header_value(), "bytes=0-9");
    }

    #[cfg(feature = "essential")]
    #[test]
    fn root_essential_feature_combines_docs_and_communication_paths() {
        let client = build_test_client().expect("client should build with essential feature");

        let _communication = &client.communication;
        let _docs = &client.docs;
        let recipient = crate::communication::MessageRecipient::open_id("ou_xxx");
        let query = crate::docs::BitableRecordQuery::new("app_token", "table_id");

        assert_eq!(recipient.receive_id, "ou_xxx");
        assert_eq!(query.table_id, "table_id");
    }

    #[cfg(feature = "enterprise")]
    #[test]
    fn root_enterprise_feature_combines_quality_critical_domains() {
        let client = build_test_client().expect("client should build with enterprise feature");

        let _security = &client.security;
        let _hr = &client.hr;
        let _workflow = &client.workflow;
        let action = crate::workflow::ApprovalTaskAction::new(
            "approval_code",
            "instance_code",
            "ou_xxx",
            "task_123",
        )
        .comment("同意");

        assert_eq!(action.task_id, "task_123");
        assert_eq!(action.comment.as_deref(), Some("同意"));
    }

    #[cfg(feature = "webhook-full")]
    #[test]
    fn root_webhook_full_feature_exposes_signature_and_robot_client() {
        let client = crate::webhook::WebhookClient::new();
        let signature = crate::webhook::common::signature::sign(1_700_000_000, "secret");

        let _ = client;
        assert!(!signature.is_empty());
    }
}
