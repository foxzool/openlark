macro_rules! declare_client {
    ($(
        {
            feature: $feature:literal,
            field: $field:ident,
            ty: $ty:ty,
            doc: $doc:literal,
            init: |$core_config:ident, $base_core_config:ident| $init:block $(,)?
        }
    ),+ $(,)?) => {
        /// 🚀 OpenLark客户端 - 极简设计
        ///
        /// # 特性
        /// - 零配置启动：`Client::from_env()`
        /// - 单入口：meta 链式字段访问（`client.docs/...`）
        /// - 编译时feature优化
        /// - 高性能异步
        /// - 现代化错误处理
        ///
        /// # 示例
        /// ```rust,no_run
        /// use openlark_client::prelude::*;
        ///
        /// #[tokio::main]
        /// async fn main() -> Result<()> {
        ///     // 从环境变量创建客户端
        ///     let client = Client::from_env()?;
        ///
        ///     // meta 链式入口（需要对应 feature）
        ///     // - 通讯：client.communication.im...
        ///     // - 文档：client.docs.ccm...
        ///     // - 认证：client.auth.app / client.auth.user / client.auth.oauth
        ///
        ///     Ok(())
        /// }
        /// ```
        #[derive(Clone)]
        pub struct Client {
            /// 服务注册表
            registry: std::sync::Arc<crate::DefaultServiceRegistry>,
            /// 统一配置（Arc 零拷贝共享，供所有 meta client 复用）
            config: openlark_core::config::Config,

            $(
                #[doc = $doc]
                #[cfg(feature = $feature)]
                pub $field: $ty,
            )*
        }

        impl std::fmt::Debug for Client {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Client")
                    .field("config", &"<CoreConfig>")
                    .field("registry", &"<Registry>")
                    $(
                        .field(stringify!($field), &cfg!(feature = $feature))
                    )*
                    .finish()
            }
        }

        impl Client {
            fn from_parts(
                registry: std::sync::Arc<crate::DefaultServiceRegistry>,
                _base_core_config: openlark_core::config::Config,
                core_config: openlark_core::config::Config,
            ) -> crate::Result<Self> {
                $(
                    #[cfg(feature = $feature)]
                    let $field: $ty = (|
                        $core_config: openlark_core::config::Config,
                        $base_core_config: openlark_core::config::Config,
                    | -> crate::Result<$ty> { Ok($init) })(
                        core_config.clone(),
                        _base_core_config.clone(),
                    )?;
                )*

                Ok(Self {
                    config: core_config.clone(),
                    registry,
                    $(
                        #[cfg(feature = $feature)]
                        $field,
                    )*
                })
            }
        }
    };
}
