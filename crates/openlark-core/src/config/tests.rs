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
        retry_count: 3,
        enable_log: true,
        allow_custom_base_url: false,
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
        retry_count: 3,
        enable_log: true,
        allow_custom_base_url: false,
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
fn test_config_allow_custom_base_url_default() {
    let config = Config::default();
    assert!(!config.allow_custom_base_url());
}

#[test]
fn test_config_builder_allow_custom_base_url() {
    let config = Config::builder().allow_custom_base_url(true).build();
    assert!(config.allow_custom_base_url());
}

// ===== T2: validate + is_known_base_url（SSRF 白名单）=====

#[test]
fn test_is_known_base_url_whitelist() {
    assert!(is_known_base_url("https://open.feishu.cn"));
    assert!(is_known_base_url("https://api.feishu.cn"));
    assert!(is_known_base_url("https://open.larksuite.com"));
    assert!(is_known_base_url("https://custom.larkoffice.com"));
}

#[test]
fn test_is_known_base_url_rejects_unknown() {
    assert!(!is_known_base_url("https://evil.com"));
    assert!(!is_known_base_url("https://example.com"));
    assert!(!is_known_base_url("https://fake-larksuite.com"));
    assert!(!is_known_base_url("not a url"));
}

#[test]
fn test_config_validate_whitelist_ok() {
    for url in [
        "https://open.feishu.cn",
        "https://open.larksuite.com",
        "https://open.larkoffice.com",
    ] {
        let config = Config::builder()
            .app_id("app")
            .app_secret("secret")
            .base_url(url)
            .build();
        assert!(config.validate().is_ok(), "{url} 应通过白名单校验");
    }
}

#[test]
fn test_config_validate_non_whitelist_rejected() {
    let config = Config::builder()
        .app_id("app")
        .app_secret("secret")
        .base_url("https://evil.com")
        .build();
    let err = config.validate().unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("allow_custom_base_url"),
        "错误消息应提示 allow_custom_base_url，实际: {msg}"
    );
}

#[test]
fn test_config_validate_allow_custom_exempts_whitelist() {
    let config = Config::builder()
        .app_id("app")
        .app_secret("secret")
        .base_url("https://evil.com")
        .allow_custom_base_url(true)
        .build();
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_validate_empty_app_id() {
    let config = Config::builder()
        .app_secret("secret")
        .base_url("https://open.feishu.cn")
        .build();
    assert!(config.app_id().is_empty());
    assert!(config.validate().is_err());
}

#[test]
fn test_config_validate_empty_app_secret() {
    let config = Config::builder()
        .app_id("app")
        .base_url("https://open.feishu.cn")
        .build();
    assert!(config.app_secret().is_empty());
    assert!(config.validate().is_err());
}

#[test]
fn test_config_validate_retry_count_too_high() {
    let config = Config::builder()
        .app_id("app")
        .app_secret("secret")
        .base_url("https://open.feishu.cn")
        .retry_count(11)
        .build();
    assert!(config.validate().is_err());
}

#[test]
fn test_config_build_does_not_validate() {
    // build() 不校验：app_id 空仍返回 Config，不抛错（分叉 1 回归保护）
    let config = Config::builder().app_id("").build();
    assert!(config.validate().is_err());
}

// ===== T3: from_env / load_from_env =====

/// 临时设置环境变量并在闭包返回后恢复（串行化避免并行测试污染）
fn with_env_vars<R>(vars: &[(&str, Option<&str>)], f: impl FnOnce() -> R) -> R {
    use std::sync::{Mutex, OnceLock};
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let _guard = LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner());

    struct Restore(Vec<(String, Option<String>)>);
    impl Drop for Restore {
        fn drop(&mut self) {
            for (k, v) in self.0.drain(..) {
                match v {
                    Some(v) => unsafe { std::env::set_var(k, v) },
                    None => unsafe { std::env::remove_var(k) },
                }
            }
        }
    }

    let saved = vars
        .iter()
        .map(|(k, _)| ((*k).to_string(), std::env::var(k).ok()))
        .collect::<Vec<_>>();
    for (k, v) in vars {
        match v {
            Some(v) => unsafe { std::env::set_var(k, v) },
            None => unsafe { std::env::remove_var(k) },
        }
    }
    let _restore = Restore(saved);
    f()
}

#[test]
fn test_config_from_env_reads_all_vars() {
    with_env_vars(
        &[
            ("OPENLARK_APP_ID", Some("env_app")),
            ("OPENLARK_APP_SECRET", Some("env_secret")),
            ("OPENLARK_APP_TYPE", Some("marketplace")),
            ("OPENLARK_BASE_URL", Some("https://open.larksuite.com")),
            ("OPENLARK_ENABLE_TOKEN_CACHE", Some("false")),
            ("OPENLARK_TIMEOUT", Some("60")),
            ("OPENLARK_RETRY_COUNT", Some("5")),
            ("OPENLARK_MAX_RESPONSE_SIZE", Some("12345")),
            ("OPENLARK_ENABLE_LOG", Some("false")),
        ],
        || {
            let config = Config::from_env();
            assert_eq!(config.app_id(), "env_app");
            assert_eq!(config.app_secret(), "env_secret");
            assert_eq!(config.app_type(), AppType::Marketplace);
            assert_eq!(config.base_url(), "https://open.larksuite.com");
            assert!(!config.enable_token_cache());
            // 分叉 5：OPENLARK_TIMEOUT → req_timeout(Some(Duration))
            assert_eq!(config.req_timeout(), Some(Duration::from_secs(60)));
            assert_eq!(config.retry_count(), 5);
            assert_eq!(config.max_response_size(), 12345);
            assert!(!config.enable_log());
        },
    );
}

#[test]
fn test_config_from_env_missing_uses_defaults() {
    with_env_vars(
        &[
            ("OPENLARK_APP_ID", None),
            ("OPENLARK_APP_SECRET", None),
            ("OPENLARK_APP_TYPE", None),
            ("OPENLARK_BASE_URL", None),
            ("OPENLARK_ENABLE_TOKEN_CACHE", None),
            ("OPENLARK_TIMEOUT", None),
            ("OPENLARK_RETRY_COUNT", None),
            ("OPENLARK_MAX_RESPONSE_SIZE", None),
            ("OPENLARK_ENABLE_LOG", None),
        ],
        || {
            let config = Config::from_env();
            assert_eq!(config.app_id(), "");
            assert_eq!(config.app_type(), AppType::SelfBuild);
            assert!(config.enable_token_cache());
            // 分叉 5：未设 TIMEOUT → req_timeout 保持 None（非 client 的 30s）
            assert_eq!(config.req_timeout(), None);
            assert_eq!(config.retry_count(), 3);
            assert!(config.enable_log());
        },
    );
}

#[test]
fn test_config_from_env_invalid_does_not_block() {
    // app_id 空（不设），from_env 仍返回 Config 不 panic；validate 才报错
    with_env_vars(
        &[
            ("OPENLARK_APP_ID", None),
            ("OPENLARK_APP_SECRET", Some("secret")),
            ("OPENLARK_BASE_URL", Some("https://open.feishu.cn")),
        ],
        || {
            let config = Config::from_env();
            assert!(config.app_id().is_empty());
            assert!(config.validate().is_err());
        },
    );
}

#[test]
fn test_config_load_from_env_mutates() {
    with_env_vars(&[("OPENLARK_APP_ID", Some("loaded_id"))], || {
        let mut config = Config::default();
        assert_eq!(config.app_id(), "");
        config.load_from_env();
        assert_eq!(config.app_id(), "loaded_id");
    });
}

// ===== T4: ConfigSummary + summary() =====

#[test]
fn test_config_summary_fields() {
    let config = Config::builder()
        .app_id("app")
        .app_secret("top-secret-value")
        .base_url("https://open.feishu.cn")
        .retry_count(7)
        .max_response_size(999)
        .req_timeout(Duration::from_secs(45))
        .build();
    let s = config.summary();
    assert_eq!(s.app_id, "app");
    assert!(s.app_secret_set);
    assert_eq!(s.app_type, AppType::SelfBuild);
    assert!(s.enable_token_cache);
    assert_eq!(s.base_url, "https://open.feishu.cn");
    assert!(!s.allow_custom_base_url);
    assert_eq!(s.req_timeout, Some(Duration::from_secs(45)));
    assert_eq!(s.retry_count, 7);
    assert!(s.enable_log);
    assert_eq!(s.header_count, 0);
    assert_eq!(s.max_response_size, 999);
}

#[test]
fn test_config_summary_secret_not_leaked() {
    let config = Config::builder().app_secret("top-secret-value").build();
    let s = config.summary();
    assert!(s.app_secret_set);
    // app_secret 明文不应出现在 summary 结构体或其 Display 输出
    let display = format!("{s}");
    assert!(!display.contains("top-secret-value"));
}

#[test]
fn test_config_summary_secret_unset() {
    let config = Config::default();
    assert!(!config.summary().app_secret_set);
}

#[test]
fn test_config_summary_header_count() {
    let mut h = HashMap::new();
    h.insert("X-Custom".to_string(), "v".to_string());
    let config = Config::builder().header(h).build();
    assert_eq!(config.summary().header_count, 1);
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

// ===== #414: ConfigBuilder as canonical configuration state =====

#[test]
fn test_config_builder_default_timeout_is_none() {
    let config = Config::builder().build();
    assert!(config.req_timeout().is_none());
}

#[test]
fn test_config_builder_load_from_env_overrides_prior_setters() {
    with_env_vars(
        &[
            ("OPENLARK_APP_ID", Some("env_app")),
            ("OPENLARK_TIMEOUT", Some("90")),
        ],
        || {
            let config = Config::builder()
                .app_id("code_app")
                .req_timeout(Duration::from_secs(10))
                .load_from_env()
                .build();
            assert_eq!(config.app_id(), "env_app");
            assert_eq!(config.req_timeout(), Some(Duration::from_secs(90)));
        },
    );
}

#[test]
fn test_config_builder_setter_after_load_from_env_overrides_env() {
    with_env_vars(
        &[
            ("OPENLARK_APP_ID", Some("env_app")),
            ("OPENLARK_TIMEOUT", Some("90")),
        ],
        || {
            let config = Config::builder()
                .load_from_env()
                .app_id("code_app")
                .req_timeout(Duration::from_secs(10))
                .build();
            assert_eq!(config.app_id(), "code_app");
            assert_eq!(config.req_timeout(), Some(Duration::from_secs(10)));
        },
    );
}

#[test]
fn test_config_builder_load_from_env_ignores_missing_empty_and_invalid() {
    with_env_vars(
        &[
            ("OPENLARK_APP_ID", Some("kept_if_empty_or_missing")),
            ("OPENLARK_APP_SECRET", Some("")),
            ("OPENLARK_TIMEOUT", Some("not-a-number")),
            ("OPENLARK_APP_TYPE", Some("unknown_type")),
            ("OPENLARK_RETRY_COUNT", Some("bad")),
        ],
        || {
            // 先写入已知状态，再 load：缺失/空/非法 env 不得擦除
            let config = Config::builder()
                .app_id("code_app")
                .app_secret("code_secret")
                .req_timeout(Duration::from_secs(15))
                .app_type(AppType::Marketplace)
                .retry_count(4)
                .load_from_env()
                .build();

            // OPENLARK_APP_ID 有效 → 覆盖
            assert_eq!(config.app_id(), "kept_if_empty_or_missing");
            // 空 secret → 保留
            assert_eq!(config.app_secret(), "code_secret");
            // 非法 timeout / retry → 保留
            assert_eq!(config.req_timeout(), Some(Duration::from_secs(15)));
            assert_eq!(config.retry_count(), 4);
            // 未知 app_type → 保留
            assert_eq!(config.app_type(), AppType::Marketplace);
        },
    );
}

#[test]
fn test_config_builder_add_header_incremental_and_last_write_wins() {
    let config = Config::builder()
        .add_header("X-A", "1")
        .add_header("X-B", "2")
        .add_header("X-A", "3")
        .build();
    assert_eq!(config.header().get("X-A"), Some(&"3".to_string()));
    assert_eq!(config.header().get("X-B"), Some(&"2".to_string()));
    assert_eq!(config.header().len(), 2);
}

#[test]
fn test_config_builder_header_map_replace_and_add_order() {
    let mut whole = HashMap::new();
    whole.insert("X-A".to_string(), "from_map".to_string());
    whole.insert("X-C".to_string(), "c".to_string());

    // add → replace → add：整体替换清除先前增量，后续增量叠在 map 上
    let config = Config::builder()
        .add_header("X-A", "incremental")
        .add_header("X-B", "b")
        .header(whole)
        .add_header("X-A", "after_map")
        .build();

    assert_eq!(config.header().get("X-A"), Some(&"after_map".to_string()));
    assert_eq!(config.header().get("X-C"), Some(&"c".to_string()));
    assert!(
        !config.header().contains_key("X-B"),
        "header(map) 应整体替换，清除先前的 X-B"
    );
}

#[test]
fn test_config_builder_retains_allow_custom_base_url() {
    let config = Config::builder()
        .base_url("https://proxy.example.com")
        .allow_custom_base_url(true)
        .build();
    assert!(config.allow_custom_base_url());
    assert_eq!(config.base_url(), "https://proxy.example.com");
}

#[test]
fn test_config_builder_clone_preserves_state() {
    let builder = Config::builder()
        .app_id("clone_app")
        .app_secret("clone_secret")
        .req_timeout(Duration::from_secs(12))
        .add_header("X-Clone", "yes")
        .allow_custom_base_url(true);
    let cloned = builder.clone();
    let a = builder.build();
    let b = cloned.build();
    assert_eq!(a.app_id(), b.app_id());
    assert_eq!(a.app_secret(), b.app_secret());
    assert_eq!(a.req_timeout(), b.req_timeout());
    assert_eq!(a.allow_custom_base_url(), b.allow_custom_base_url());
    assert_eq!(a.header().get("X-Clone"), b.header().get("X-Clone"));
}

#[test]
fn test_config_builder_debug_redacts_secrets_and_headers() {
    let builder = Config::builder()
        .app_id("debug_app")
        .app_secret("super-secret-value")
        .add_header("Authorization", "Bearer secret-token")
        .token_provider(TestTokenProvider);
    let debug = format!("{builder:?}");
    assert!(
        debug.starts_with("ConfigBuilder"),
        "Debug 应以 ConfigBuilder 开头: {debug}"
    );
    assert!(debug.contains("debug_app"), "Debug 应包含 app_id: {debug}");
    assert!(
        debug.contains("***"),
        "Debug 应对 app_secret 脱敏为 ***: {debug}"
    );
    assert!(
        !debug.contains("super-secret-value"),
        "Debug 不得泄露 app_secret: {debug}"
    );
    assert!(
        !debug.contains("Bearer secret-token"),
        "Debug 不得泄露 header 值: {debug}"
    );
    assert!(
        !debug.contains("TestTokenProvider"),
        "Debug 不得暴露 token provider 内部: {debug}"
    );
}

#[test]
fn test_config_builder_load_from_env_empty_enable_log_preserves() {
    with_env_vars(&[("OPENLARK_ENABLE_LOG", Some(""))], || {
        let config = Config::builder().enable_log(false).load_from_env().build();
        assert!(
            !config.enable_log(),
            "空 OPENLARK_ENABLE_LOG 应保留 builder 中的 false"
        );
    });
}

#[test]
fn test_config_builder_load_from_env_bool_zero_is_false() {
    // ENABLE_LOG 与 ENABLE_TOKEN_CACHE 共用 parse_env_bool："0" → false
    with_env_vars(
        &[
            ("OPENLARK_ENABLE_LOG", Some("0")),
            ("OPENLARK_ENABLE_TOKEN_CACHE", Some("0")),
        ],
        || {
            let config = Config::builder()
                .enable_log(true)
                .enable_token_cache(true)
                .load_from_env()
                .build();
            assert!(!config.enable_log());
            assert!(!config.enable_token_cache());
        },
    );
}
