//! OpenLark Client - 全新简化架构
//!
//! 极简设计：仅保留 meta 链式字段访问（单入口，KISS）

#[macro_use]
mod macros;

mod builder;
mod error_handling;
#[cfg(test)]
mod tests;

pub use builder::ClientBuilder;
pub use error_handling::ClientErrorHandling;

use crate::{
    DefaultServiceRegistry, Result,
    client_build_config::validate_core_config,
    error::{with_context, with_operation_context},
    traits::LarkClient,
};
use std::sync::Arc;

/// 🔐 认证 meta 入口：`client.auth.app / client.auth.user / client.auth.oauth`
#[cfg(feature = "auth")]
#[derive(Debug, Clone)]
pub struct AuthClient {
    /// 应用认证服务
    pub app: openlark_auth::AuthService,
    /// 用户身份认证服务
    pub user: openlark_auth::AuthenService,
    /// OAuth 授权服务
    pub oauth: openlark_auth::OAuthService,
}

#[cfg(feature = "auth")]
impl AuthClient {
    fn new(config: openlark_core::config::Config) -> Self {
        Self {
            app: openlark_auth::AuthService::new(config.clone()),
            user: openlark_auth::AuthenService::new(config.clone()),
            oauth: openlark_auth::OAuthService::new(config),
        }
    }
}

declare_client! {
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
            AuthClient::new(_base_core_config.clone())
        },
    },
    {
        feature: "docs",
        field: docs,
        ty: openlark_docs::DocsClient,
        doc: "Docs meta 调用链入口：client.docs.ccm / client.docs.base ...",
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
        doc: "HR meta 调用链入口：client.hr.attendance / client.hr.corehr / client.hr.hire ...",
        init: |_core_config, _base_core_config| {
            openlark_hr::HrClient::new(_core_config.clone())
        },
    },
    {
        feature: "meeting",
        field: meeting,
        ty: openlark_meeting::MeetingClient,
        doc: "Meeting meta 调用链入口：client.meeting.vc.v1.room.create() ...",
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
        doc: "Workflow meta 调用链入口：client.workflow.task.create() ...",
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
            crate::PlatformClient::new(_core_config.clone())?
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
            crate::AnalyticsClient::new(_core_config.clone())?
        },
    },
    {
        feature: "user",
        field: user,
        ty: crate::UserClient,
        doc: "User meta 调用链入口：client.user.setting... ...",
        init: |_core_config, _base_core_config| {
            crate::UserClient::new(_core_config.clone())?
        },
    },
    {
        feature: "security",
        field: security,
        ty: crate::SecurityClient,
        doc: "Security meta 调用链入口：client.security.acs... ...",
        init: |_core_config, _base_core_config| {
            let security_config = openlark_security::models::SecurityConfig::new(
                _core_config.app_id().to_string(),
                _core_config.app_secret().to_string(),
            )
            .with_base_url(_core_config.base_url());
            openlark_security::SecurityClient::new(security_config)
        },
    },
}

impl Client {
    /// 🔥 从环境变量创建客户端
    ///
    /// # 环境变量
    /// ```bash
    /// export OPENLARK_APP_ID=your_app_id
    /// export OPENLARK_APP_SECRET=your_app_secret
    /// export OPENLARK_BASE_URL=<https://open.feishu.cn>  # 可选
    /// ```
    ///
    ///
    /// # 返回值
    /// 返回配置好的客户端实例或错误
    ///
    /// # 示例
    /// ```rust,no_run
    /// use openlark_client::Client;
    ///
    /// let _client = Client::from_env();
    /// ```
    pub fn from_env() -> Result<Self> {
        Self::builder().from_env().build()
    }

    /// 🏗️ 创建构建器
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// 🔧 获取客户端配置（统一的 CoreConfig）
    pub fn config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    /// 📋 获取服务注册表
    pub fn registry(&self) -> &DefaultServiceRegistry {
        &self.registry
    }

    /// 🔧 获取底层 core 配置
    ///
    /// 与 [`Self::config`] 返回同一份配置。保留此别名是为了向后兼容。
    pub fn core_config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    /// 🔧 获取可直接传给函数式 API 的认证后配置
    ///
    /// 与 [`Self::config`] 返回同一份配置。保留此别名是为了让
    /// 业务侧更容易理解它的用途：可直接传给 `openlark_docs::*`、
    /// `openlark_auth::*` 等函数式 API。
    pub fn api_config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    /// ✅ 检查客户端是否已正确配置
    pub fn is_configured(&self) -> bool {
        !self.config.app_id().is_empty() && !self.config.app_secret().is_empty()
    }

    /// 🆕 使用统一 CoreConfig 创建客户端
    pub fn with_core_config(config: openlark_core::config::Config) -> Result<Self> {
        if let Err(err) = validate_core_config(&config) {
            return with_context(Err(err), "operation", "Client::with_core_config");
        }

        Self::with_validated_core_config(config, "Client::with_core_config")
    }

    pub(crate) fn with_validated_core_config(
        base_core_config: openlark_core::config::Config,
        operation: &str,
    ) -> Result<Self> {
        let mut registry = DefaultServiceRegistry::new();

        if let Err(err) = crate::registry::bootstrap::register_compiled_services(&mut registry) {
            return with_operation_context(Err(err), operation, "service_loading");
        }

        let registry = Arc::new(registry);

        #[cfg(feature = "auth")]
        let core_config = {
            use openlark_auth::AuthTokenProvider;
            let provider = AuthTokenProvider::new(base_core_config.clone());
            base_core_config.with_token_provider(provider)
        };
        #[cfg(not(feature = "auth"))]
        let core_config = base_core_config.clone();

        Self::from_parts(registry, base_core_config, core_config)
    }

    /// 🔧 执行带有错误上下文的操作
    pub async fn execute_with_context<F, T>(&self, operation: &str, f: F) -> Result<T>
    where
        F: std::future::Future<Output = Result<T>>,
    {
        let result = f.await;
        with_operation_context(result, operation, "Client")
    }
}

impl LarkClient for Client {
    fn config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    fn is_configured(&self) -> bool {
        self.is_configured()
    }
}
