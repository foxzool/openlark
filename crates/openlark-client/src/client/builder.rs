use super::Client;
use crate::{Result, client_build_config::ClientBuildConfig};
use openlark_core::error::ErrorTrait;

/// 🏗️ 客户端构建器 - 流畅API
///
/// 提供链式调用的客户端构建方式
///
/// # 示例
/// ```rust,no_run
/// use openlark_client::Client;
/// use openlark_client::Result;
/// use std::time::Duration;
///
/// fn main() -> Result<()> {
///     let _client = Client::builder()
///         .app_id("your_app_id")
///         .app_secret("your_app_secret")
///         .base_url("<https://open.feishu.cn>")
///         .timeout(Duration::from_secs(30))
///         .build()?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    pub(super) config: ClientBuildConfig,
}

impl ClientBuilder {
    /// 🆕 创建新的构建器实例
    pub fn new() -> Self {
        Self {
            config: ClientBuildConfig::default(),
        }
    }

    /// 🆔 设置应用ID
    pub fn app_id<S: Into<String>>(mut self, app_id: S) -> Self {
        self.config.app_id(app_id);
        self
    }

    /// 🔑 设置应用密钥
    pub fn app_secret<S: Into<String>>(mut self, app_secret: S) -> Self {
        self.config.app_secret(app_secret);
        self
    }

    /// 🏷️ 设置应用类型（自建 / 商店）
    pub fn app_type(mut self, app_type: openlark_core::constants::AppType) -> Self {
        self.config.app_type(app_type);
        self
    }

    /// 🔐 设置是否允许自动获取 token（默认 true）
    pub fn enable_token_cache(mut self, enable: bool) -> Self {
        self.config.enable_token_cache(enable);
        self
    }

    /// 🌐 设置基础URL
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.config.base_url(base_url);
        self
    }

    /// 🔓 允许自定义 base_url 域名
    pub fn allow_custom_base_url(mut self, allow: bool) -> Self {
        self.config.allow_custom_base_url(allow);
        self
    }

    /// ⏱️ 设置请求超时时间
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.config.timeout(timeout);
        self
    }

    /// 🔄 设置重试次数
    pub fn retry_count(mut self, retry_count: u32) -> Self {
        self.config.retry_count(retry_count);
        self
    }

    /// 📝 启用或禁用日志
    pub fn enable_log(mut self, enable: bool) -> Self {
        self.config.enable_log(enable);
        self
    }

    /// 设置响应体最大大小限制（字节）
    pub fn max_response_size(mut self, size: u64) -> Self {
        self.config.max_response_size(size);
        self
    }

    /// 🔧 添加自定义 HTTP header
    pub fn add_header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.config.add_header(key, value);
        self
    }

    /// 🌍 从环境变量加载配置
    pub fn from_env(mut self) -> Self {
        self.config.load_from_env();
        self
    }

    /// 🔨 构建客户端实例
    ///
    /// # 返回值
    /// 返回配置好的客户端实例或验证错误
    ///
    /// # 错误
    /// 如果配置验证失败，会返回相应的错误信息，包含用户友好的恢复建议
    pub fn build(self) -> Result<Client> {
        let result = self.config.validate().and_then(|()| {
            Client::with_validated_core_config(
                self.config.build_core_config(),
                "ClientBuilder::build",
            )
        });
        if let Err(ref error) = result {
            tracing::error!(
                "客户端构建失败: {}",
                error.user_message().unwrap_or("未知错误")
            );
        }
        result
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
