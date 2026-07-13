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
    error::{with_context, with_operation_context},
    traits::LarkClient,
    validation_error,
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

// legacy 业务域（#436 待迁）仍走 declare_client 双声明；
// catalog 域（#434 bot + #435 foundational）通过 for_each_compiled_capability
// callback 追加进同一 Client 结构体。
//
// 命名：本宏是「包裹 declare_client! 并追加 catalog 条目」，不是「声明 catalog 本身」。
// 死匹配（name/description/...）：统一条目同时含构造与诊断字段，本侧只消费构造字段；
// 与 generate_catalog_registry! 对称，是双投影的固有成本，而非重复逻辑。
macro_rules! append_catalog_entries {
    ($({
        feature: $c_feature:literal,
        field: $c_field:ident,
        ty: $c_ty:ty,
        doc: $c_doc:literal,
        init: |$c_core:ident, $c_base:ident| $c_init:block,
        // 诊断字段：由 registry 投影消费；此处仅匹配统一条目形状
        name: $_name:literal,
        description: $_description:literal,
        dependencies: [$( $_dep:literal ),* $(,)?],
        provides: [$( $_cap:literal ),* $(,)?],
        priority: $_priority:literal $(,)?
    }),* $(,)?) => {
        declare_client! {
            // --- legacy（#436）---
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
            // capability catalog 条目（#434 bot + #435 foundational；#436 继续只改 catalog）
            $(
                {
                    feature: $c_feature,
                    field: $c_field,
                    ty: $c_ty,
                    doc: $c_doc,
                    init: |$c_core, $c_base| $c_init,
                },
            )*
        }
    };
}

crate::capability::for_each_compiled_capability!(append_catalog_entries);

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
    ///
    /// 与 [`ClientBuilder::build`] 共用私有构造 seam：`Config::validate`（含域名白名单）
    /// + Client 零超时规则 + registry / token provider 装配。
    pub fn with_core_config(config: openlark_core::config::Config) -> Result<Self> {
        Self::with_checked_core_config(config, "Client::with_core_config")
    }

    /// 已校验构造 seam：`ClientBuilder::build` 与 [`Self::with_core_config`] 的唯一入口。
    ///
    /// 顺序：
    /// 1. [`openlark_core::config::Config::validate`]（凭据 / URL / 白名单 / retry）
    /// 2. Client 特有：拒绝 `req_timeout == Some(Duration::ZERO)`（`None` 允许）
    /// 3. 校验错误附加 `operation` context
    /// 4. registry 初始化（失败时保留 operation + `service_loading`）
    /// 5. token provider 注入
    /// 6. 组装 [`Client`]
    pub(crate) fn with_checked_core_config(
        base_core_config: openlark_core::config::Config,
        operation: &str,
    ) -> Result<Self> {
        if let Err(err) = base_core_config.validate() {
            return with_context(Err(err), "operation", operation);
        }
        if base_core_config
            .req_timeout()
            .is_some_and(|timeout| timeout.is_zero())
        {
            return with_context(
                Err(validation_error("timeout", "timeout必须大于0")),
                "operation",
                operation,
            );
        }

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
