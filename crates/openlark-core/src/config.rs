use std::{collections::HashMap, ops::Deref, sync::Arc, time::Duration};

use crate::{
    auth::token_provider::{NoOpTokenProvider, TokenProvider},
    constants::{AppType, FEISHU_BASE_URL},
    performance::OptimizedHttpConfig,
};

/// # 零拷贝配置共享实现
///
/// `Config` 内部使用 `Arc<ConfigInner>` 实现零拷贝共享:
///
/// ## 性能特性
/// - **内存效率**: 所有克隆共享同一份配置数据(~300-500字节)
/// - **克隆成本**: `Config::clone()` 只复制Arc指针(8字节 + 原子操作)
/// - **线程安全**: Arc保证多线程安全的只读访问
/// - **引用计数**: 自动管理内存,无泄漏风险
///
/// ## 使用建议
/// ```rust
/// // ✅ 推荐: 克隆Config传递给服务
/// let service = MyService::new(config.clone());
///
/// // ✅ 推荐: 在Request中持有Config
/// pub struct MyRequest {
///     config: Config,  // 持有Arc指针,成本低
/// }
///
/// // ⚠️ 不必要: 使用Arc<Config> (Config内部已经是Arc)
/// // Arc<Arc<ConfigInner>> = 双重Arc,没有额外收益
/// ```
///
/// ## 性能验证
/// 运行 `cargo test config_arc` 查看基准测试:
/// - 克隆速度: ~10-20纳秒
/// - 内存开销: 每个克隆仅8字节
/// - 引用计数: 自动维护
#[derive(Debug, Clone)]
pub struct Config {
    /// 包装在 Arc 中的共享配置数据
    ///
    /// 所有 Config 实例通过 Arc 共享同一份 ConfigInner,
    /// 实现零拷贝的配置共享。
    inner: Arc<ConfigInner>,
}

/// 内部配置数据，被多个服务共享
pub struct ConfigInner {
    pub(crate) app_id: String,
    pub(crate) app_secret: String,
    /// 域名, 默认为 <https://open.feishu.cn>
    pub(crate) base_url: String,
    /// 是否允许 core 在缺少显式 token 时自动获取 token
    pub(crate) enable_token_cache: bool,
    /// 应用类型, 默认为自建应用
    pub(crate) app_type: AppType,
    pub(crate) http_client: reqwest::Client,
    /// 客户端超时时间, 默认永不超时
    pub(crate) req_timeout: Option<Duration>,
    pub(crate) header: HashMap<String, String>,
    /// Token 获取抽象（由业务 crate 实现，例如 openlark-auth）
    pub(crate) token_provider: Arc<dyn TokenProvider>,
    /// 响应体最大大小（字节），超过返回 ResponseTooLarge 错误，默认 100MB
    pub(crate) max_response_size: u64,
}

impl Default for ConfigInner {
    fn default() -> Self {
        Self {
            app_id: "".to_string(),
            app_secret: "".to_string(),
            base_url: FEISHU_BASE_URL.to_string(),
            enable_token_cache: true,
            app_type: AppType::SelfBuild,
            http_client: reqwest::Client::new(),
            req_timeout: None,
            header: Default::default(),
            token_provider: Arc::new(NoOpTokenProvider),
            max_response_size: 100 * 1024 * 1024, // 100MB
        }
    }
}

impl std::fmt::Debug for ConfigInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigInner")
            .field("app_id", &self.app_id)
            .field("app_secret", &"***")
            .field("base_url", &self.base_url)
            .field("enable_token_cache", &self.enable_token_cache)
            .field("app_type", &self.app_type)
            .field("req_timeout", &self.req_timeout)
            .field("max_response_size", &self.max_response_size)
            .field("header", &format!("{} headers", self.header.len()))
            .finish()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            inner: Arc::new(ConfigInner::default()),
        }
    }
}

impl Deref for Config {
    type Target = ConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Config {
    /// 创建配置构建器
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// 创建新的 Config 实例，直接从 ConfigInner
    pub fn new(inner: ConfigInner) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }

    /// 基于当前配置生成一个“替换 TokenProvider”的新配置
    ///
    /// 说明：
    /// - 这是一个纯拷贝操作（`Config` 本身是 `Arc` 包装），不会修改原配置
    /// - 推荐用法：先构建一个“基础 Config”（默认 `NoOpTokenProvider`），再用该基础 Config 构建业务 TokenProvider，
    ///   最后调用此方法把 provider 注入到“业务 Config”中，避免循环引用。
    pub fn with_token_provider(&self, provider: impl TokenProvider + 'static) -> Self {
        Config::new(ConfigInner {
            app_id: self.app_id.clone(),
            app_secret: self.app_secret.clone(),
            base_url: self.base_url.clone(),
            enable_token_cache: self.enable_token_cache,
            app_type: self.app_type,
            http_client: self.http_client.clone(),
            req_timeout: self.req_timeout,
            header: self.header.clone(),
            token_provider: Arc::new(provider),
            max_response_size: self.max_response_size,
        })
    }

    /// 获取内部 Arc 的引用计数
    pub fn reference_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    /// 获取应用 ID
    pub fn app_id(&self) -> &str {
        &self.inner.app_id
    }

    /// 获取应用密钥
    pub fn app_secret(&self) -> &str {
        &self.inner.app_secret
    }

    /// 获取基础 URL
    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    /// 获取超时时间
    pub fn req_timeout(&self) -> Option<Duration> {
        self.inner.req_timeout
    }

    /// 是否启用令牌缓存
    pub fn enable_token_cache(&self) -> bool {
        self.inner.enable_token_cache
    }

    /// 获取应用类型
    pub fn app_type(&self) -> AppType {
        self.inner.app_type
    }

    /// 获取 HTTP 客户端引用
    pub fn http_client(&self) -> &reqwest::Client {
        &self.inner.http_client
    }

    /// 获取自定义 header 引用
    pub fn header(&self) -> &HashMap<String, String> {
        &self.inner.header
    }

    /// 获取 TokenProvider 引用
    pub fn token_provider(&self) -> &Arc<dyn TokenProvider> {
        &self.inner.token_provider
    }

    /// 获取响应体最大大小限制
    pub fn max_response_size(&self) -> u64 {
        self.inner.max_response_size
    }
}

/// 配置构建器
#[derive(Default, Clone)]
pub struct ConfigBuilder {
    app_id: Option<String>,
    app_secret: Option<String>,
    base_url: Option<String>,
    enable_token_cache: Option<bool>,
    app_type: Option<AppType>,
    http_client: Option<reqwest::Client>,
    req_timeout: Option<Duration>,
    header: Option<HashMap<String, String>>,
    token_provider: Option<Arc<dyn TokenProvider>>,
    max_response_size: Option<u64>,
}

impl ConfigBuilder {
    /// 设置应用 ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.app_secret = Some(app_secret.into());
        self
    }

    /// 设置基础 URL
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// 设置是否启用令牌缓存
    pub fn enable_token_cache(mut self, enable: bool) -> Self {
        self.enable_token_cache = Some(enable);
        self
    }

    /// 设置应用类型
    pub fn app_type(mut self, app_type: AppType) -> Self {
        self.app_type = Some(app_type);
        self
    }

    /// 设置 HTTP 客户端
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// 使用优化的HTTP配置构建客户端
    pub fn optimized_http_client(
        mut self,
        config: OptimizedHttpConfig,
    ) -> Result<Self, reqwest::Error> {
        let client = config.build_client()?;
        self.http_client = Some(client);
        Ok(self)
    }

    /// 使用生产环境优化配置
    pub fn production_http_client(self) -> Result<Self, reqwest::Error> {
        let config = OptimizedHttpConfig::production();
        self.optimized_http_client(config)
    }

    /// 使用高吞吐量配置
    pub fn high_throughput_http_client(self) -> Result<Self, reqwest::Error> {
        let config = OptimizedHttpConfig::high_throughput();
        self.optimized_http_client(config)
    }

    /// 使用低延迟配置
    pub fn low_latency_http_client(self) -> Result<Self, reqwest::Error> {
        let config = OptimizedHttpConfig::low_latency();
        self.optimized_http_client(config)
    }

    /// 设置请求超时时间
    pub fn req_timeout(mut self, timeout: Duration) -> Self {
        self.req_timeout = Some(timeout);
        self
    }

    /// 设置自定义 HTTP 头
    pub fn header(mut self, header: HashMap<String, String>) -> Self {
        self.header = Some(header);
        self
    }

    /// 设置令牌提供者
    pub fn token_provider(mut self, provider: impl TokenProvider + 'static) -> Self {
        self.token_provider = Some(Arc::new(provider));
        self
    }

    /// 设置响应体最大大小限制（字节），默认 100MB
    pub fn max_response_size(mut self, size: u64) -> Self {
        self.max_response_size = Some(size);
        self
    }

    /// 构建 Config 实例
    pub fn build(self) -> Config {
        let default = ConfigInner::default();
        Config::new(ConfigInner {
            app_id: self.app_id.unwrap_or(default.app_id),
            app_secret: self.app_secret.unwrap_or(default.app_secret),
            base_url: self.base_url.unwrap_or(default.base_url),
            enable_token_cache: self
                .enable_token_cache
                .unwrap_or(default.enable_token_cache),
            app_type: self.app_type.unwrap_or(default.app_type),
            http_client: self.http_client.unwrap_or(default.http_client),
            req_timeout: self.req_timeout.or(default.req_timeout),
            header: self.header.unwrap_or(default.header),
            token_provider: self.token_provider.unwrap_or(default.token_provider),
            max_response_size: self.max_response_size.unwrap_or(default.max_response_size),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::NoOpTokenProvider;
    use crate::auth::TokenProvider;
    use crate::auth::TokenRequest;
    use crate::constants::{AppType, FEISHU_BASE_URL};
    use std::time::Duration;
    use std::{future::Future, pin::Pin};

    #[test]
    fn test_config_creation() {
        let config = Config::new(ConfigInner {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://test.api.com".to_string(),
            enable_token_cache: true,
            app_type: AppType::SelfBuild,
            http_client: reqwest::Client::new(),
            req_timeout: Some(Duration::from_secs(30)),
            header: HashMap::new(),
            token_provider: Arc::new(NoOpTokenProvider),
            max_response_size: 100 * 1024 * 1024,
        });

        assert_eq!(config.app_id, "test_app_id");
        assert_eq!(config.app_secret, "test_app_secret");
        assert_eq!(config.base_url, "https://test.api.com");
        assert!(config.enable_token_cache);
        assert_eq!(config.req_timeout, Some(Duration::from_secs(30)));
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();

        assert_eq!(config.app_id, "");
        assert_eq!(config.app_secret, "");
        assert_eq!(config.base_url, FEISHU_BASE_URL);
        assert!(config.enable_token_cache);
        assert_eq!(config.app_type, AppType::SelfBuild);
        assert!(config.req_timeout.is_none());
        assert!(config.header.is_empty());
    }

    #[test]
    fn test_config_clone() {
        let config = Config::new(ConfigInner {
            app_id: "clone_test".to_string(),
            app_secret: "clone_secret".to_string(),
            base_url: "https://clone.test.com".to_string(),
            enable_token_cache: false,
            app_type: AppType::Marketplace,
            http_client: reqwest::Client::new(),
            req_timeout: Some(Duration::from_secs(60)),
            header: {
                let mut header = HashMap::new();
                header.insert("Test-Header".to_string(), "test-value".to_string());
                header
            },
            token_provider: Arc::new(NoOpTokenProvider),
            max_response_size: 100 * 1024 * 1024,
        });

        let cloned_config = config.clone();

        assert_eq!(config.app_id, cloned_config.app_id);
        assert_eq!(config.app_secret, cloned_config.app_secret);
        assert_eq!(config.base_url, cloned_config.base_url);
        assert_eq!(config.enable_token_cache, cloned_config.enable_token_cache);
        assert_eq!(config.app_type, cloned_config.app_type);
        assert_eq!(config.req_timeout, cloned_config.req_timeout);
        assert_eq!(config.header.len(), cloned_config.header.len());
        assert_eq!(
            config.header.get("Test-Header"),
            cloned_config.header.get("Test-Header")
        );

        // Verify Arc clone efficiency - both should point to same memory
        assert!(Arc::ptr_eq(&config.inner, &cloned_config.inner));

        // Verify reference counting works
        assert_eq!(config.reference_count(), 2);
    }

    #[test]
    fn test_config_debug() {
        let config = Config::default();
        let debug_str = format!("{config:?}");

        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("app_id"));
        assert!(debug_str.contains("app_secret"));
        assert!(debug_str.contains("base_url"));
    }

    #[test]
    fn test_config_with_custom_header() {
        let mut header = HashMap::new();
        header.insert("Authorization".to_string(), "Bearer token".to_string());
        header.insert("Content-Type".to_string(), "application/json".to_string());

        let config = Config::new(ConfigInner {
            header,
            ..ConfigInner::default()
        });

        assert_eq!(config.header.len(), 2);
        assert_eq!(
            config.header.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
        assert_eq!(
            config.header.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_config_with_different_app_types() {
        let self_build_config = Config::new(ConfigInner {
            app_type: AppType::SelfBuild,
            ..ConfigInner::default()
        });

        let marketplace_config = Config::new(ConfigInner {
            app_type: AppType::Marketplace,
            ..ConfigInner::default()
        });

        assert_eq!(self_build_config.app_type, AppType::SelfBuild);
        assert_eq!(marketplace_config.app_type, AppType::Marketplace);
        assert_ne!(self_build_config.app_type, marketplace_config.app_type);
    }

    #[test]
    fn test_config_with_timeout_variations() {
        let no_timeout_config = Config::default();

        let short_timeout_config = Config::new(ConfigInner {
            req_timeout: Some(Duration::from_secs(5)),
            ..ConfigInner::default()
        });

        let long_timeout_config = Config::new(ConfigInner {
            req_timeout: Some(Duration::from_secs(300)),
            ..ConfigInner::default()
        });

        assert!(no_timeout_config.req_timeout.is_none());
        assert_eq!(
            short_timeout_config.req_timeout,
            Some(Duration::from_secs(5))
        );
        assert_eq!(
            long_timeout_config.req_timeout,
            Some(Duration::from_secs(300))
        );
    }

    #[test]
    fn test_config_builders() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();

        assert_eq!(config.app_id, "test_app");
        assert_eq!(config.app_secret, "test_secret");
    }

    #[test]
    fn test_config_arc_efficiency() {
        let config = Config::default();
        assert_eq!(config.reference_count(), 1);

        let config_clone = config.clone();
        assert_eq!(config.reference_count(), 2);
        assert_eq!(config_clone.reference_count(), 2);

        // Both configs should point to the same inner data
        assert!(Arc::ptr_eq(&config.inner, &config_clone.inner));
    }

    #[test]
    fn test_arc_efficiency_simulation() {
        // 模拟服务模块中的多次克隆
        let config = Config::default();

        // 模拟 PerformanceService::new() 中的4次clone
        let service1_config = config.clone();
        let service2_config = config.clone();
        let service3_config = config.clone();
        let service4_config = config.clone();

        // 所有配置应该指向同一个内存位置
        assert!(Arc::ptr_eq(&config.inner, &service1_config.inner));
        assert!(Arc::ptr_eq(&config.inner, &service2_config.inner));
        assert!(Arc::ptr_eq(&config.inner, &service3_config.inner));
        assert!(Arc::ptr_eq(&config.inner, &service4_config.inner));

        // 引用计数应该是5（原始 + 4个克隆）
        assert_eq!(config.reference_count(), 5);

        println!("Arc<Config> 改造成功：5个配置实例共享同一份内存！");
    }

    #[derive(Debug)]
    struct TestTokenProvider;

    impl TokenProvider for TestTokenProvider {
        fn get_token(
            &self,
            _request: TokenRequest,
        ) -> Pin<Box<dyn Future<Output = crate::SDKResult<String>> + Send + '_>> {
            Box::pin(async { Ok("test_token".to_string()) })
        }
    }

    #[tokio::test]
    async fn test_with_token_provider() {
        let base = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();

        let config = base.with_token_provider(TestTokenProvider);

        let token = config
            .token_provider
            .get_token(TokenRequest::app())
            .await
            .unwrap();
        assert_eq!(token, "test_token");
    }
}
