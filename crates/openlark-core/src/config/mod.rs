use std::{collections::HashMap, ops::Deref, sync::Arc, time::Duration};

use crate::{
    auth::token_provider::{NoOpTokenProvider, TokenProvider},
    constants::{AppType, FEISHU_BASE_URL},
    error::CoreError,
    performance::OptimizedHttpConfig,
};

/// 解析 `OPENLARK_*` 布尔环境值。
///
/// - 空 / 全空白 → `None`（保留当前配置状态）
/// - `f*` / `0` / `false`（不区分大小写，trim 后）→ `Some(false)`
/// - 其它非空 → `Some(true)`
///
/// `OPENLARK_ENABLE_TOKEN_CACHE` 与 `OPENLARK_ENABLE_LOG` 共用此规则。
fn parse_env_bool(value: &str) -> Option<bool> {
    let s = value.trim().to_lowercase();
    if s.is_empty() {
        return None;
    }
    Some(!(s.starts_with('f') || s == "0"))
}

/// 检查 base_url 是否指向已知的飞书/Lark 域名（白名单 SSRF 防护）
///
/// 已知域名后缀：`feishu.cn`、`larksuite.com`、`larkoffice.com`。
/// [`Config::validate`](Config::validate) 用本函数做 base_url 白名单校验；
/// `allow_custom_base_url` 为 true 时跳过该校验。公开以便下游（如 client 构建校验）
/// 复用同一白名单判定。
pub fn is_known_base_url(url: &str) -> bool {
    let allowed_suffixes = ["feishu.cn", "larksuite.com", "larkoffice.com"];
    if let Ok(parsed) = url::Url::parse(url)
        && let Some(host) = parsed.host_str()
    {
        return allowed_suffixes
            .iter()
            .any(|suffix| host == *suffix || host.ends_with(&format!(".{suffix}")));
    }
    false
}

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
#[derive(Clone)]
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
    /// 默认重试次数，默认 3
    pub(crate) retry_count: u32,
    /// 是否启用日志记录，默认 true
    pub(crate) enable_log: bool,
    /// 是否允许自定义 base_url 域名（绕过飞书白名单 SSRF 防护），默认 false
    pub(crate) allow_custom_base_url: bool,
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
            retry_count: 3,
            enable_log: true,
            allow_custom_base_url: false,
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
            .field("retry_count", &self.retry_count)
            .field("enable_log", &self.enable_log)
            .field("allow_custom_base_url", &self.allow_custom_base_url)
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
        let mut inner = (*self.inner).clone();
        inner.token_provider = Arc::new(provider);
        Config::new(inner)
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

    /// 获取重试次数
    pub fn retry_count(&self) -> u32 {
        self.inner.retry_count
    }

    /// 是否启用日志记录
    pub fn enable_log(&self) -> bool {
        self.inner.enable_log
    }

    /// 是否允许自定义 base_url 域名（绕过白名单 SSRF 防护）
    pub fn allow_custom_base_url(&self) -> bool {
        self.inner.allow_custom_base_url
    }

    /// 校验配置有效性
    ///
    /// # 校验规则
    /// - `app_id` / `app_secret` 非空
    /// - `base_url` 非空且以 `<http://>` / `<https://>` 开头
    /// - `base_url` 域名在飞书/Lark 白名单内（`*.feishu.cn` / `*.larksuite.com` /
    ///   `*.larkoffice.com`）；`allow_custom_base_url` 为 true 时豁免（SSRF 防护）
    /// - `retry_count` 不超过 10
    ///
    /// `builder().build()` **不会**自动调用本方法——与 core 现有行为一致，避免破坏
    /// 所有现有 `core::Config` 用户；`from_env()` 会在内部调用本方法但失败时仅记录、
    /// 不阻塞返回。需要强校验的调用方应显式调用 `validate()`。
    pub fn validate(&self) -> Result<(), CoreError> {
        if self.app_id.is_empty() {
            return Err(CoreError::validation_builder()
                .field("app_id")
                .message("app_id 不能为空")
                .build());
        }
        if self.app_secret.is_empty() {
            return Err(CoreError::validation_builder()
                .field("app_secret")
                .message("app_secret 不能为空")
                .build());
        }
        if self.base_url.is_empty() {
            return Err(CoreError::validation_builder()
                .field("base_url")
                .message("base_url 不能为空")
                .build());
        }
        if !self.base_url.starts_with("http://") && !self.base_url.starts_with("https://") {
            return Err(CoreError::validation_builder()
                .field("base_url")
                .message("base_url 必须以 http:// 或 https:// 开头")
                .build());
        }
        if !self.allow_custom_base_url && !is_known_base_url(&self.base_url) {
            tracing::warn!(
                "base_url '{}' 不在飞书/Lark 已知域名白名单中。\
                 如需使用自定义域名，请设置 allow_custom_base_url(true)。",
                self.base_url
            );
            return Err(CoreError::validation_builder()
                .field("base_url")
                .message(
                    "base_url 域名不在白名单中，已知域名: *.feishu.cn, *.larksuite.com, \
                     *.larkoffice.com。如需使用自定义域名，请设置 allow_custom_base_url(true)",
                )
                .build());
        }
        if self.retry_count > 10 {
            return Err(CoreError::validation_builder()
                .field("retry_count")
                .message("retry_count 不能超过 10")
                .build());
        }
        Ok(())
    }

    /// 从 `OPENLARK_*` 环境变量加载配置
    ///
    /// 读取环境变量构建 Config，缺失变量用默认值。内部调用 [`validate`](Self::validate)，
    /// 校验失败仅记录警告、不阻塞返回（与 `builder().build()` 不校验语义一致）。
    /// 返回的 Config 可能未通过校验，关键路径应追加显式 `.validate()` 确认。
    ///
    /// # 环境变量
    /// - `OPENLARK_APP_ID` / `OPENLARK_APP_SECRET` / `OPENLARK_APP_TYPE`
    ///   （`self_build`/`selfbuild`/`self` 或 `marketplace`/`store`）
    /// - `OPENLARK_BASE_URL` / `OPENLARK_ENABLE_TOKEN_CACHE`
    /// - `OPENLARK_TIMEOUT`（秒）→ `req_timeout(Some(Duration))`；未设保持 `None`
    /// - `OPENLARK_RETRY_COUNT` / `OPENLARK_MAX_RESPONSE_SIZE` / `OPENLARK_ENABLE_LOG`
    pub fn from_env() -> Config {
        let mut inner = ConfigInner::default();
        Self::apply_env_vars(&mut inner);
        let config = Config::new(inner);
        if let Err(e) = config.validate() {
            tracing::warn!("from_env 加载的配置未通过校验: {e}");
        }
        config
    }

    /// 从 `OPENLARK_*` 环境变量加载到当前实例（写时复制：独占引用时原地修改）
    ///
    /// 仅设置存在且非空的环境变量（`OPENLARK_APP_TYPE` 除外，它对非法值静默忽略）。
    pub fn load_from_env(&mut self) {
        let inner = Arc::make_mut(&mut self.inner);
        Self::apply_env_vars(inner);
    }

    fn apply_env_vars(inner: &mut ConfigInner) {
        for (key, value) in std::env::vars() {
            Self::apply_env_var(inner, &key, &value);
        }
    }

    fn apply_env_var(inner: &mut ConfigInner, key: &str, value: &str) {
        match key {
            "OPENLARK_APP_ID" if !value.is_empty() => inner.app_id = value.to_string(),
            "OPENLARK_APP_SECRET" if !value.is_empty() => inner.app_secret = value.to_string(),
            "OPENLARK_APP_TYPE" => {
                let v = value.trim().to_lowercase();
                match v.as_str() {
                    "self_build" | "selfbuild" | "self" => inner.app_type = AppType::SelfBuild,
                    "marketplace" | "store" => inner.app_type = AppType::Marketplace,
                    _ => {}
                }
            }
            "OPENLARK_BASE_URL" if !value.is_empty() => inner.base_url = value.to_string(),
            "OPENLARK_ENABLE_TOKEN_CACHE" => {
                if let Some(v) = parse_env_bool(value) {
                    inner.enable_token_cache = v;
                }
            }
            // 分叉 5：秒数 → req_timeout(Some)；未设保持 None
            "OPENLARK_TIMEOUT" => {
                if let Ok(secs) = value.parse::<u64>() {
                    inner.req_timeout = Some(Duration::from_secs(secs));
                }
            }
            "OPENLARK_RETRY_COUNT" => {
                if let Ok(n) = value.parse::<u32>() {
                    inner.retry_count = n;
                }
            }
            "OPENLARK_MAX_RESPONSE_SIZE" => {
                if let Ok(size) = value.parse::<u64>() {
                    inner.max_response_size = size;
                }
            }
            "OPENLARK_ENABLE_LOG" => {
                if let Some(v) = parse_env_bool(value) {
                    inner.enable_log = v;
                }
            }
            _ => {}
        }
    }

    /// 生成不含敏感信息的配置摘要
    ///
    /// `app_secret` 仅以布尔「是否已设置」表示，不泄露明文。
    pub fn summary(&self) -> ConfigSummary {
        ConfigSummary {
            app_id: self.app_id.clone(),
            app_secret_set: !self.app_secret.is_empty(),
            app_type: self.app_type,
            enable_token_cache: self.enable_token_cache,
            base_url: self.base_url.clone(),
            allow_custom_base_url: self.allow_custom_base_url,
            req_timeout: self.req_timeout,
            retry_count: self.retry_count,
            enable_log: self.enable_log,
            header_count: self.header.len(),
            max_response_size: self.max_response_size,
        }
    }
}

/// 配置构建器
///
/// 直接持有一份 canonical [`ConfigInner`] 状态：所有 setter、环境覆盖与 header
/// 合并都写同一份状态，`build()` 仅将其移入 [`Config`]。
///
/// 不自动调用 [`Config::validate`]——保持 core 宽松构建契约。
#[derive(Clone, Default)]
pub struct ConfigBuilder {
    inner: ConfigInner,
}

impl std::fmt::Debug for ConfigBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 复用 ConfigInner 的脱敏 Debug，避免字段列表双份漂移
        f.debug_struct("ConfigBuilder")
            .field("inner", &self.inner)
            .finish()
    }
}

impl ConfigBuilder {
    /// 设置应用 ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.inner.app_id = app_id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.inner.app_secret = app_secret.into();
        self
    }

    /// 设置基础 URL
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.inner.base_url = base_url.into();
        self
    }

    /// 设置是否启用令牌缓存
    pub fn enable_token_cache(mut self, enable: bool) -> Self {
        self.inner.enable_token_cache = enable;
        self
    }

    /// 设置应用类型
    pub fn app_type(mut self, app_type: AppType) -> Self {
        self.inner.app_type = app_type;
        self
    }

    /// 设置 HTTP 客户端
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.inner.http_client = client;
        self
    }

    /// 使用优化的HTTP配置构建客户端
    pub fn optimized_http_client(
        mut self,
        config: OptimizedHttpConfig,
    ) -> Result<Self, reqwest::Error> {
        let client = config.build_client()?;
        self.inner.http_client = client;
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
        self.inner.req_timeout = Some(timeout);
        self
    }

    /// 整体替换自定义 HTTP 头
    ///
    /// 与 [`Self::add_header`] 混用时按链式调用顺序生效：本方法清除先前增量写入。
    pub fn header(mut self, header: HashMap<String, String>) -> Self {
        self.inner.header = header;
        self
    }

    /// 增量添加单个 HTTP 头（同名键后写覆盖）
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner.header.insert(key.into(), value.into());
        self
    }

    /// 在当前链式位置叠加 `OPENLARK_*` 环境变量
    ///
    /// 与 [`Config::from_env`] / [`Config::load_from_env`] 共用同一套环境解释逻辑。
    /// 缺失、空值或不可解析的可选变量保留当前 builder 状态；有效值覆盖当前状态。
    pub fn load_from_env(mut self) -> Self {
        Config::apply_env_vars(&mut self.inner);
        self
    }

    /// 设置令牌提供者
    pub fn token_provider(mut self, provider: impl TokenProvider + 'static) -> Self {
        self.inner.token_provider = Arc::new(provider);
        self
    }

    /// 设置响应体最大大小限制（字节），默认 100MB
    pub fn max_response_size(mut self, size: u64) -> Self {
        self.inner.max_response_size = size;
        self
    }

    /// 设置默认重试次数，默认 3
    pub fn retry_count(mut self, count: u32) -> Self {
        self.inner.retry_count = count;
        self
    }

    /// 设置是否启用日志记录，默认 true
    pub fn enable_log(mut self, enable: bool) -> Self {
        self.inner.enable_log = enable;
        self
    }

    /// 设置是否允许自定义 base_url 域名（绕过白名单 SSRF 防护），默认 false
    pub fn allow_custom_base_url(mut self, allow: bool) -> Self {
        self.inner.allow_custom_base_url = allow;
        self
    }

    /// 构建 Config 实例（不自动校验）
    pub fn build(self) -> Config {
        Config::new(self.inner)
    }
}

/// 配置摘要（不含敏感信息）
///
/// 由 [`Config::summary`] 生成。`app_secret` 仅以 `app_secret_set` 布尔表示是否已设置，
/// 不泄露明文。便于日志、调试和展示。
#[derive(Debug, Clone)]
pub struct ConfigSummary {
    /// 应用 ID
    pub app_id: String,
    /// 应用密钥是否已设置（不泄露明文）
    pub app_secret_set: bool,
    /// 应用类型
    pub app_type: AppType,
    /// 是否允许自动获取 token
    pub enable_token_cache: bool,
    /// API 基础 URL
    pub base_url: String,
    /// 是否允许自定义 base_url 域名（绕过白名单 SSRF 防护）
    pub allow_custom_base_url: bool,
    /// 请求超时时间（None 表示永不超时）
    pub req_timeout: Option<Duration>,
    /// 默认重试次数
    pub retry_count: u32,
    /// 是否启用日志记录
    pub enable_log: bool,
    /// 自定义 headers 数量
    pub header_count: usize,
    /// 响应体最大大小限制（字节）
    pub max_response_size: u64,
}

impl ConfigSummary {
    /// 获取友好的中文配置描述
    pub fn friendly_description(&self) -> String {
        format!(
            "应用ID: {}, 基础URL: {}, 超时: {:?}, 重试: {}, 日志: {}, Headers: {}, 最大响应: {}",
            self.app_id,
            self.base_url,
            self.req_timeout,
            self.retry_count,
            if self.enable_log { "启用" } else { "禁用" },
            self.header_count,
            self.max_response_size
        )
    }
}

impl std::fmt::Display for ConfigSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 注意：只输出 app_secret_set 布尔，绝不输出 app_secret 明文
        write!(
            f,
            "Config {{ app_id: {}, app_secret_set: {}, base_url: {}, req_timeout: {:?}, retry_count: {}, enable_log: {}, header_count: {}, max_response_size: {} }}",
            self.app_id,
            self.app_secret_set,
            self.base_url,
            self.req_timeout,
            self.retry_count,
            self.enable_log,
            self.header_count,
            self.max_response_size
        )
    }
}

#[cfg(test)]
mod tests;
