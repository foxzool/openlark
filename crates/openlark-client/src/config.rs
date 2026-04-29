//! OpenLark Client 配置管理
//!
//! 提供灵活的配置系统，支持环境变量、验证和默认值

use crate::Result;
use std::time::Duration;

use openlark_core::config::Config as CoreConfig;
use openlark_core::constants::AppType;

/// Check if the base_url points to a known Lark/Feishu domain
fn is_known_base_url(url: &str) -> bool {
    let allowed_suffixes = ["feishu.cn", "larksuite.com", "larkoffice.com"];
    // Parse URL, extract host, check suffix
    if let Ok(parsed) = url::Url::parse(url)
        && let Some(host) = parsed.host_str()
    {
        return allowed_suffixes
            .iter()
            .any(|suffix| host == *suffix || host.ends_with(&format!(".{suffix}")));
    }
    false
}

/// 🔧 OpenLark客户端配置
///
/// 支持从环境变量自动加载配置
///
/// # 环境变量
/// - `OPENLARK_APP_ID`: 应用ID（必需）
/// - `OPENLARK_APP_SECRET`: 应用密钥（必需）
/// - `OPENLARK_APP_TYPE`: 应用类型（可选：self_build / marketplace，默认 self_build）
/// - `OPENLARK_BASE_URL`: API基础URL（可选，默认：`https://open.feishu.cn`，国际版 Lark 使用 `https://open.larksuite.com`）
/// - `OPENLARK_ENABLE_TOKEN_CACHE`: 是否允许自动获取 token（可选，默认 true）
///
/// # 示例
/// ```rust,no_run
/// use openlark_client::Config;
///
/// // 从环境变量创建配置
/// let config = Config::from_env();
///
/// // 手动构建配置
/// let config = Config::builder()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret")
///     .base_url("https://open.feishu.cn")  // 默认值，国际版 Lark 使用 https://open.larksuite.com
///     .build();
/// ```
#[derive(Clone)]
pub struct Config {
    /// 🆔 飞书应用ID
    pub app_id: String,
    /// 🔑 飞书应用密钥
    pub app_secret: String,
    /// 🏷️ 应用类型（自建 / 商店）
    pub app_type: AppType,
    /// 🔐 是否允许 SDK 自动获取 token（通过 openlark-core 的 TokenProvider）
    pub enable_token_cache: bool,
    /// 🌐 API基础URL
    pub base_url: String,
    /// 🔓 是否允许自定义 base_url 域名
    pub allow_custom_base_url: bool,
    /// ⏱️ 请求超时时间
    pub timeout: Duration,
    /// 🔄 默认重试次数
    pub retry_count: u32,
    /// 📝 是否启用日志记录
    pub enable_log: bool,
    /// 📋 自定义HTTP headers
    pub headers: std::collections::HashMap<String, String>,
    /// 响应体最大大小限制（字节），默认 100MB
    pub max_response_size: u64,
    /// 🔧 底层 core 配置（按需生成）
    #[doc(hidden)]
    pub(crate) core_config: Option<CoreConfig>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("app_id", &self.app_id)
            .field("app_secret", &"***")
            .field("app_type", &self.app_type)
            .field("enable_token_cache", &self.enable_token_cache)
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("retry_count", &self.retry_count)
            .field("enable_log", &self.enable_log)
            .field("headers", &format!("{} headers", self.headers.len()))
            .field("max_response_size", &self.max_response_size)
            .finish()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            app_type: AppType::SelfBuild,
            enable_token_cache: true,
            base_url: "https://open.feishu.cn".to_string(),
            allow_custom_base_url: false,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            enable_log: true,
            headers: std::collections::HashMap::new(),
            max_response_size: 100 * 1024 * 1024, // 100MB
            core_config: None,
        }
    }
}

impl Config {
    /// 🆕 创建新的配置实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 🌍 从环境变量创建配置
    ///
    /// # 环境变量
    /// - `OPENLARK_APP_ID`: 应用ID（必需）
    /// - `OPENLARK_APP_SECRET`: 应用密钥（必需）
    /// - `OPENLARK_APP_TYPE`: 应用类型（可选：self_build / marketplace）
    /// - `OPENLARK_BASE_URL`: API基础URL（可选，默认 `https://open.feishu.cn`，国际版 Lark 使用 `https://open.larksuite.com`）
    /// - `OPENLARK_ENABLE_TOKEN_CACHE`: 是否允许自动获取 token（可选）
    ///
    /// # 返回值
    /// 返回配置实例，环境变量缺失时使用默认值
    pub fn from_env() -> Self {
        let mut config = Self::default();
        config.load_from_env();
        config
    }

    /// 📥 从环境变量加载配置到当前实例
    ///
    /// 只设置存在且非空的环境变量
    pub fn load_from_env(&mut self) {
        for (key, value) in std::env::vars() {
            self.apply_env_var(&key, &value);
        }
    }

    fn apply_env_var(&mut self, key: &str, value: &str) {
        match key {
            "OPENLARK_APP_ID" if !value.is_empty() => self.app_id = value.to_string(),
            "OPENLARK_APP_SECRET" if !value.is_empty() => self.app_secret = value.to_string(),
            "OPENLARK_APP_TYPE" => {
                let v = value.trim().to_lowercase();
                match v.as_str() {
                    "self_build" | "selfbuild" | "self" => self.app_type = AppType::SelfBuild,
                    "marketplace" | "store" => self.app_type = AppType::Marketplace,
                    _ => {}
                }
            }
            "OPENLARK_BASE_URL" if !value.is_empty() => self.base_url = value.to_string(),
            "OPENLARK_ENABLE_TOKEN_CACHE" => {
                let s = value.trim().to_lowercase();
                if !s.is_empty() {
                    self.enable_token_cache = !(s.starts_with('f') || s == "0");
                }
            }
            "OPENLARK_TIMEOUT" => {
                if let Ok(timeout_secs) = value.parse::<u64>() {
                    self.timeout = Duration::from_secs(timeout_secs);
                }
            }
            "OPENLARK_RETRY_COUNT" => {
                if let Ok(retry_count) = value.parse::<u32>() {
                    self.retry_count = retry_count;
                }
            }
            "OPENLARK_MAX_RESPONSE_SIZE" => {
                if let Ok(size) = value.parse::<u64>() {
                    self.max_response_size = size;
                }
            }
            // 日志开关（默认启用，只有设置为"false"时才禁用）
            "OPENLARK_ENABLE_LOG" => {
                self.enable_log = !value.to_lowercase().starts_with('f');
            }
            _ => {}
        }
    }

    /// ✅ 验证配置的有效性
    ///
    /// # 验证规则
    /// - app_id和app_secret不能为空
    /// - base_url必须是有效的HTTP/HTTPS URL
    /// - timeout必须大于0
    /// - retry_count不能超过合理的范围
    ///
    /// # 错误
    /// 返回验证失败的详细错误信息
    pub fn validate(&self) -> Result<()> {
        if self.app_id.is_empty() {
            return Err(crate::error::validation_error("app_id", "app_id不能为空"));
        }

        if self.app_secret.is_empty() {
            return Err(crate::error::validation_error(
                "app_secret",
                "app_secret不能为空",
            ));
        }

        if self.base_url.is_empty() {
            return Err(crate::error::validation_error(
                "base_url",
                "base_url不能为空",
            ));
        }

        // 验证URL格式
        if !self.base_url.starts_with("http://") && !self.base_url.starts_with("https://") {
            return Err(crate::error::validation_error(
                "base_url",
                "base_url必须以http://或https://开头",
            ));
        }

        // Check base_url domain against whitelist
        if !self.allow_custom_base_url && !is_known_base_url(&self.base_url) {
            tracing::warn!(
                "base_url '{}' is not a known Feishu/Lark domain.
                If this is intentional, set allow_custom_base_url(true) in config.",
                self.base_url
            );
            return Err(crate::error::validation_error(
                "base_url",
                "base_url 域名不在白名单中，已知域名: *.feishu.cn, *.larksuite.com, *.larkoffice.com。如需使用自定义域名，请设置 allow_custom_base_url(true)",
            ));
        }

        // 验证超时时间
        if self.timeout.is_zero() {
            return Err(crate::error::validation_error(
                "timeout",
                "timeout必须大于0",
            ));
        }

        // 验证重试次数
        if self.retry_count > 10 {
            return Err(crate::error::validation_error(
                "retry_count",
                "retry_count不能超过10",
            ));
        }

        Ok(())
    }

    /// 🏗️ 创建配置构建器
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// 🔧 添加自定义HTTP header
    pub fn add_header<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
    }

    /// 🧹 清除所有自定义headers
    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }

    /// 🔍 检查配置是否完整（可用于API调用）
    pub fn is_complete(&self) -> bool {
        !self.app_id.is_empty() && !self.app_secret.is_empty()
    }

    /// 📋 获取配置摘要（不包含敏感信息）
    pub fn summary(&self) -> ConfigSummary {
        ConfigSummary {
            app_id: self.app_id.clone(),
            app_secret_set: !self.app_secret.is_empty(),
            app_type: self.app_type,
            enable_token_cache: self.enable_token_cache,
            base_url: self.base_url.clone(),
            allow_custom_base_url: self.allow_custom_base_url,
            timeout: self.timeout,
            retry_count: self.retry_count,
            enable_log: self.enable_log,
            header_count: self.headers.len(),
            max_response_size: self.max_response_size,
        }
    }

    /// 🔄 更新配置，只更新非空字段
    pub fn update_with(&mut self, other: &Config) {
        if !other.app_id.is_empty() {
            self.app_id = other.app_id.clone();
        }
        if !other.app_secret.is_empty() {
            self.app_secret = other.app_secret.clone();
        }
        if other.app_type != AppType::SelfBuild {
            self.app_type = other.app_type;
        }
        if other.enable_token_cache != self.enable_token_cache {
            self.enable_token_cache = other.enable_token_cache;
        }
        if !other.base_url.is_empty() {
            self.base_url = other.base_url.clone();
        }
        if !other.timeout.is_zero() {
            self.timeout = other.timeout;
        }
        if other.retry_count != 3 {
            self.retry_count = other.retry_count;
        }
        if other.max_response_size != 100 * 1024 * 1024 {
            self.max_response_size = other.max_response_size;
        }
        if other.enable_log != self.enable_log {
            self.enable_log = other.enable_log;
        }
        // 合并headers
        for (key, value) in &other.headers {
            self.headers.insert(key.clone(), value.clone());
        }
    }

    /// 🔧 构建底层 core 配置（不含 TokenProvider）
    pub fn build_core_config(&self) -> CoreConfig {
        CoreConfig::builder()
            .app_id(self.app_id.clone())
            .app_secret(self.app_secret.clone())
            .base_url(self.base_url.clone())
            .app_type(self.app_type)
            .enable_token_cache(self.enable_token_cache)
            .req_timeout(self.timeout)
            .max_response_size(self.max_response_size)
            .header(self.headers.clone())
            .build()
    }

    /// 🔧 构建带有默认 TokenProvider 的 core 配置
    #[cfg(feature = "auth")]
    pub fn build_core_config_with_token_provider(&self) -> CoreConfig {
        use openlark_auth::AuthTokenProvider;
        let base_config = self.build_core_config();
        let provider = AuthTokenProvider::new(base_config.clone());
        base_config.with_token_provider(provider)
    }

    /// 🔧 获取或构建 core 配置
    pub fn get_or_build_core_config(&self) -> CoreConfig {
        if let Some(ref core_config) = self.core_config {
            return core_config.clone();
        }
        self.build_core_config()
    }

    /// 🔧 获取或构建带有 TokenProvider 的 core 配置
    #[cfg(feature = "auth")]
    pub fn get_or_build_core_config_with_token_provider(&self) -> CoreConfig {
        if let Some(ref core_config) = self.core_config {
            return core_config.clone();
        }
        self.build_core_config_with_token_provider()
    }
}

/// 🏗️ 配置构建器 - 提供流畅的API
///
/// # 示例
/// ```rust,no_run
/// use openlark_client::Config;
/// use std::time::Duration;
///
/// let config = Config::builder()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret")
///     .base_url("https://open.feishu.cn")
///     .timeout(Duration::from_secs(60))
///     .retry_count(5)
///     .enable_log(true)
///     .add_header("X-Custom-Header", "value")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// 🆕 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// 🆔 设置应用ID
    pub fn app_id<S: Into<String>>(mut self, app_id: S) -> Self {
        self.config.app_id = app_id.into();
        self
    }

    /// 🔑 设置应用密钥
    pub fn app_secret<S: Into<String>>(mut self, app_secret: S) -> Self {
        self.config.app_secret = app_secret.into();
        self
    }

    /// 🏷️ 设置应用类型（自建 / 商店）
    pub fn app_type(mut self, app_type: AppType) -> Self {
        self.config.app_type = app_type;
        self
    }

    /// 🔐 设置是否允许自动获取 token（默认 true）
    pub fn enable_token_cache(mut self, enable: bool) -> Self {
        self.config.enable_token_cache = enable;
        self
    }

    /// 🌐 设置API基础URL
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.config.base_url = base_url.into();
        self
    }

    /// ⏱️ 设置请求超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// 🔄 设置重试次数
    pub fn retry_count(mut self, count: u32) -> Self {
        self.config.retry_count = count;
        self
    }

    /// 📝 设置是否启用日志
    pub fn enable_log(mut self, enable: bool) -> Self {
        self.config.enable_log = enable;
        self
    }

    /// 设置响应体最大大小限制（字节），默认 100MB
    pub fn max_response_size(mut self, size: u64) -> Self {
        self.config.max_response_size = size;
        self
    }

    /// 🔧 添加自定义HTTP header
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

    /// 🔨 构建最终的配置实例
    ///
    /// # 错误
    /// 如果配置验证失败，返回相应的错误
    pub fn build(self) -> Result<Config> {
        self.config.validate()?;
        Ok(self.config)
    }

    /// 🔨 构建配置实例（跳过验证）
    ///
    /// 注意：使用此方法时配置可能无效，后续使用时需要验证
    pub fn build_unvalidated(self) -> Config {
        self.config
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 📋 配置摘要（不包含敏感信息）
#[derive(Debug, Clone)]
pub struct ConfigSummary {
    /// 🆔 应用ID
    pub app_id: String,
    /// 🔑 应用密钥是否已设置
    pub app_secret_set: bool,
    /// 🏷️ 应用类型
    pub app_type: AppType,
    /// 🔐 是否允许自动获取 token
    pub enable_token_cache: bool,
    /// 🌐 API基础URL
    pub base_url: String,
    /// 🔓 是否允许自定义 base_url 域名
    pub allow_custom_base_url: bool,
    /// ⏱️ 请求超时时间
    pub timeout: Duration,
    /// 🔄 重试次数
    pub retry_count: u32,
    /// 📝 是否启用日志
    pub enable_log: bool,
    /// 📋 自定义headers数量
    pub header_count: usize,
    /// 响应体最大大小限制
    pub max_response_size: u64,
}

impl ConfigSummary {
    /// 📋 获取友好的配置描述
    pub fn friendly_description(&self) -> String {
        format!(
            "应用ID: {}, 基础URL: {}, 超时: {:?}, 重试: {}, 日志: {}, Headers: {}, 最大响应: {}",
            self.app_id,
            self.base_url,
            self.timeout,
            self.retry_count,
            if self.enable_log { "启用" } else { "禁用" },
            self.header_count,
            self.max_response_size
        )
    }
}

impl std::fmt::Display for ConfigSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Config {{ app_id: {}, app_secret_set: {}, base_url: {}, timeout: {:?}, retry_count: {}, enable_log: {}, header_count: {}, max_response_size: {} }}",
            self.app_id,
            self.app_secret_set,
            self.base_url,
            self.timeout,
            self.retry_count,
            self.enable_log,
            self.header_count,
            self.max_response_size
        )
    }
}

/// 从环境变量创建配置的便利方法
impl From<std::env::Vars> for Config {
    fn from(env_vars: std::env::Vars) -> Self {
        let mut config = Config::default();
        for (key, value) in env_vars {
            config.apply_env_var(&key, &value);
        }
        config
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.app_id, "");
        assert_eq!(config.app_secret, "");
        assert_eq!(config.base_url, "https://open.feishu.cn");
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.retry_count, 3);
        assert!(config.enable_log);
        assert!(config.headers.is_empty());
    }

    #[test]
    fn test_config_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.feishu.cn")
            .timeout(Duration::from_secs(60))
            .retry_count(5)
            .enable_log(false)
            .build();

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.app_id, "test_app_id");
        assert_eq!(config.app_secret, "test_app_secret");
        assert_eq!(config.base_url, "https://test.feishu.cn");
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.retry_count, 5);
        assert!(!config.enable_log);
    }

    #[test]
    fn test_config_from_env() {
        crate::test_utils::with_env_vars(
            &[
                ("OPENLARK_APP_ID", Some("test_app_id")),
                ("OPENLARK_APP_SECRET", Some("test_app_secret")),
                ("OPENLARK_APP_TYPE", Some("marketplace")),
                ("OPENLARK_BASE_URL", Some("https://test.feishu.cn")),
                ("OPENLARK_ENABLE_TOKEN_CACHE", Some("false")),
                ("OPENLARK_TIMEOUT", Some("60")),
                ("OPENLARK_RETRY_COUNT", Some("5")),
                ("OPENLARK_ENABLE_LOG", Some("false")),
            ],
            || {
                let config = Config::from_env();
                assert_eq!(config.app_id, "test_app_id");
                assert_eq!(config.app_secret, "test_app_secret");
                assert_eq!(config.app_type, AppType::Marketplace);
                assert_eq!(config.base_url, "https://test.feishu.cn");
                assert!(!config.enable_token_cache);
                assert_eq!(config.timeout, Duration::from_secs(60));
                assert_eq!(config.retry_count, 5);
                assert!(!config.enable_log);
            },
        );
    }

    #[test]
    fn test_config_validation() {
        // 有效配置
        let config = Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            app_type: AppType::SelfBuild,
            enable_token_cache: true,
            base_url: "https://open.feishu.cn".to_string(),
            allow_custom_base_url: false,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            enable_log: true,
            headers: std::collections::HashMap::new(),
            max_response_size: 100 * 1024 * 1024,
            core_config: None,
        };
        assert!(config.validate().is_ok());

        // 无效配置 - 空app_id
        let invalid_config = Config {
            app_id: String::new(),
            ..config.clone()
        };
        assert!(invalid_config.validate().is_err());

        // 无效配置 - 空app_secret
        let invalid_config = Config {
            app_secret: String::new(),
            ..config
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_config_headers() {
        let mut config = Config::default();

        // 添加header
        config.add_header("X-Custom-Header", "custom-value");
        assert_eq!(config.headers.len(), 1);
        assert_eq!(
            config.headers.get("X-Custom-Header"),
            Some(&"custom-value".to_string())
        );

        // 清除headers
        config.clear_headers();
        assert!(config.headers.is_empty());
    }

    #[test]
    fn test_config_update_with() {
        let mut base_config = Config::default();
        let update_config = Config {
            app_id: "updated_app_id".to_string(),
            app_secret: "updated_app_secret".to_string(),
            timeout: Duration::from_secs(60),
            ..Default::default()
        };

        base_config.update_with(&update_config);
        assert_eq!(base_config.app_id, "updated_app_id");
        assert_eq!(base_config.app_secret, "updated_app_secret");
        assert_eq!(base_config.timeout, Duration::from_secs(60));
        // 其他字段保持默认值
        assert_eq!(base_config.base_url, "https://open.feishu.cn");
    }

    #[test]
    fn test_config_summary() {
        let config = Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            app_type: AppType::SelfBuild,
            enable_token_cache: true,
            base_url: "https://open.feishu.cn".to_string(),
            allow_custom_base_url: false,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            enable_log: true,
            headers: std::collections::HashMap::new(),
            max_response_size: 100 * 1024 * 1024,
            core_config: None,
        };

        let summary = config.summary();
        assert_eq!(summary.app_id, "test_app_id");
        assert!(summary.app_secret_set);
        assert_eq!(summary.base_url, "https://open.feishu.cn");
        assert_eq!(summary.timeout, Duration::from_secs(30));
        assert_eq!(summary.retry_count, 3);
        assert!(summary.enable_log);
        assert_eq!(summary.header_count, 0);
    }

    #[test]
    fn test_config_is_complete() {
        let mut config = Config::default();
        assert!(!config.is_complete());

        config.app_id = "test_app_id".to_string();
        assert!(!config.is_complete());

        config.app_secret = "test_app_secret".to_string();
        assert!(config.is_complete());
    }
}

#[test]
fn test_config_validation_known_base_url() {
    // 测试已知的 Feishu/Lark 域名
    let known_urls = vec![
        "https://open.feishu.cn",
        "https://api.feishu.cn",
        "https://custom.feishu.cn",
        "https://open.larksuite.com",
        "https://api.larksuite.com",
        "https://custom.larksuite.com",
        "https://open.larkoffice.com",
        "https://custom.larkoffice.com",
    ];

    for url in known_urls {
        let config = Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            app_type: AppType::SelfBuild,
            enable_token_cache: true,
            base_url: url.to_string(),
            allow_custom_base_url: false,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            enable_log: true,
            headers: std::collections::HashMap::new(),
            max_response_size: 100 * 1024 * 1024,
            core_config: None,
        };
        assert!(config.validate().is_ok(), "URL {url} should be valid");
    }
}

#[test]
fn test_config_validation_unknown_base_url_rejected() {
    // 测试未知域名被拒绝
    let unknown_urls = vec![
        "https://evil.com",
        "https://malicious-site.com",
        "https://example.com",
        "https://not-feishu.cn",
        "https://fake-larksuite.com",
    ];

    for url in unknown_urls {
        let config = Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            app_type: AppType::SelfBuild,
            enable_token_cache: true,
            base_url: url.to_string(),
            allow_custom_base_url: false,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            enable_log: true,
            headers: std::collections::HashMap::new(),
            max_response_size: 100 * 1024 * 1024,
            core_config: None,
        };
        assert!(config.validate().is_err(), "URL {url} should be rejected");
    }
}

#[test]
fn test_config_validation_custom_base_url_allowed() {
    // 测试 allow_custom_base_url 允许使用自定义域名
    let config = Config {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
        app_type: AppType::SelfBuild,
        enable_token_cache: true,
        base_url: "https://open.feishu.cn".to_string(),
        allow_custom_base_url: true,
        timeout: Duration::from_secs(30),
        retry_count: 3,
        enable_log: true,
        headers: std::collections::HashMap::new(),
        max_response_size: 100 * 1024 * 1024,
        core_config: None,
    };
    assert!(config.validate().is_ok());
}
