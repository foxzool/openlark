use super::Client;
use crate::Result;
use openlark_core::config::{Config, ConfigBuilder};
use openlark_core::error::ErrorTrait;
use std::fmt;
use std::time::Duration;

/// 🏗️ 客户端构建器 - 流畅API
///
/// 提供链式调用的客户端构建方式。配置状态委托给 core [`ConfigBuilder`]，
/// Client 仅叠加 30 秒默认超时与构建期严格校验策略。
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
#[derive(Clone)]
pub struct ClientBuilder {
    config: ConfigBuilder,
}

impl fmt::Debug for ClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 复用 ConfigBuilder 脱敏 Debug（secret / header 值 / token provider 不输出）
        f.debug_struct("ClientBuilder")
            .field("config", &self.config)
            .finish()
    }
}

impl ClientBuilder {
    /// 🆕 创建新的构建器实例
    ///
    /// 在 core 默认值之上应用 Client 策略：默认请求超时 30 秒。
    pub fn new() -> Self {
        Self {
            config: Config::builder().req_timeout(Duration::from_secs(30)),
        }
    }

    /// 🆔 设置应用ID
    pub fn app_id<S: Into<String>>(self, app_id: S) -> Self {
        Self {
            config: self.config.app_id(app_id),
        }
    }

    /// 🔑 设置应用密钥
    pub fn app_secret<S: Into<String>>(self, app_secret: S) -> Self {
        Self {
            config: self.config.app_secret(app_secret),
        }
    }

    /// 🏷️ 设置应用类型（自建 / 商店）
    pub fn app_type(self, app_type: openlark_core::constants::AppType) -> Self {
        Self {
            config: self.config.app_type(app_type),
        }
    }

    /// 🔐 设置是否允许自动获取 token（默认 true）
    pub fn enable_token_cache(self, enable: bool) -> Self {
        Self {
            config: self.config.enable_token_cache(enable),
        }
    }

    /// 🌐 设置基础URL
    pub fn base_url<S: Into<String>>(self, base_url: S) -> Self {
        Self {
            config: self.config.base_url(base_url),
        }
    }

    /// 🔓 允许自定义 base_url 域名
    pub fn allow_custom_base_url(self, allow: bool) -> Self {
        Self {
            config: self.config.allow_custom_base_url(allow),
        }
    }

    /// ⏱️ 设置请求超时时间
    pub fn timeout(self, timeout: Duration) -> Self {
        Self {
            config: self.config.req_timeout(timeout),
        }
    }

    /// 🔄 设置重试次数
    pub fn retry_count(self, retry_count: u32) -> Self {
        Self {
            config: self.config.retry_count(retry_count),
        }
    }

    /// 📝 启用或禁用日志
    pub fn enable_log(self, enable: bool) -> Self {
        Self {
            config: self.config.enable_log(enable),
        }
    }

    /// 设置响应体最大大小限制（字节）
    pub fn max_response_size(self, size: u64) -> Self {
        Self {
            config: self.config.max_response_size(size),
        }
    }

    /// 🔧 添加自定义 HTTP header
    pub fn add_header<K, V>(self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        Self {
            config: self.config.add_header(key, value),
        }
    }

    /// 🌍 从环境变量加载配置
    ///
    /// 在链式调用当前位置叠加 `OPENLARK_*` 环境变量；后写覆盖前写。
    pub fn from_env(self) -> Self {
        Self {
            config: self.config.load_from_env(),
        }
    }

    /// 🔨 构建客户端实例
    ///
    /// 与 [`Client::with_core_config`] 共用同一私有 checked 构造 seam。
    ///
    /// # 返回值
    /// 返回配置好的客户端实例或验证错误
    ///
    /// # 错误
    /// 如果配置验证失败，会返回相应的错误信息，包含用户友好的恢复建议
    pub fn build(self) -> Result<Client> {
        let result = Client::with_checked_core_config(self.config.build(), "ClientBuilder::build");
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
