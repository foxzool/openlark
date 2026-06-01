#![allow(deprecated)]

use crate::{Config, Result};
use openlark_core::{config::Config as CoreConfig, constants::AppType};
use std::{collections::HashMap, time::Duration};

#[derive(Debug, Clone)]
pub(crate) struct ClientBuildConfig {
    pub(crate) app_id: String,
    pub(crate) app_secret: String,
    app_type: AppType,
    enable_token_cache: bool,
    base_url: String,
    allow_custom_base_url: bool,
    timeout: Duration,
    retry_count: u32,
    enable_log: bool,
    headers: HashMap<String, String>,
    max_response_size: u64,
}

impl Default for ClientBuildConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            app_type: AppType::SelfBuild,
            enable_token_cache: true,
            base_url: openlark_core::constants::FEISHU_BASE_URL.to_string(),
            allow_custom_base_url: false,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            enable_log: true,
            headers: HashMap::new(),
            max_response_size: 100 * 1024 * 1024,
        }
    }
}

impl From<Config> for ClientBuildConfig {
    fn from(config: Config) -> Self {
        Self {
            app_id: config.app_id,
            app_secret: config.app_secret,
            app_type: config.app_type,
            enable_token_cache: config.enable_token_cache,
            base_url: config.base_url,
            allow_custom_base_url: config.allow_custom_base_url,
            timeout: config.timeout,
            retry_count: config.retry_count,
            enable_log: config.enable_log,
            headers: config.headers,
            max_response_size: config.max_response_size,
        }
    }
}

impl ClientBuildConfig {
    pub(crate) fn app_id(&mut self, app_id: impl Into<String>) {
        self.app_id = app_id.into();
    }

    pub(crate) fn app_secret(&mut self, app_secret: impl Into<String>) {
        self.app_secret = app_secret.into();
    }

    pub(crate) fn app_type(&mut self, app_type: AppType) {
        self.app_type = app_type;
    }

    pub(crate) fn enable_token_cache(&mut self, enable: bool) {
        self.enable_token_cache = enable;
    }

    pub(crate) fn base_url(&mut self, base_url: impl Into<String>) {
        self.base_url = base_url.into();
    }

    pub(crate) fn allow_custom_base_url(&mut self, allow: bool) {
        self.allow_custom_base_url = allow;
    }

    pub(crate) fn timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub(crate) fn retry_count(&mut self, retry_count: u32) {
        self.retry_count = retry_count;
    }

    pub(crate) fn enable_log(&mut self, enable: bool) {
        self.enable_log = enable;
    }

    pub(crate) fn max_response_size(&mut self, size: u64) {
        self.max_response_size = size;
    }

    pub(crate) fn add_header(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers.insert(key.into(), value.into());
    }

    pub(crate) fn load_from_env(&mut self) {
        for (key, value) in std::env::vars() {
            self.apply_env_var(&key, &value);
        }
    }

    pub(crate) fn validate(&self) -> Result<()> {
        if self.app_id.is_empty() {
            return Err(crate::error::validation_error("app_id", "app_id不能为空"));
        }

        if self.app_secret.is_empty() {
            return Err(crate::error::validation_error(
                "app_secret",
                "app_secret不能为空",
            ));
        }

        validate_base_url(&self.base_url)?;

        if !self.allow_custom_base_url && !crate::config::is_known_base_url(&self.base_url) {
            tracing::warn!(
                "base_url '{}' is not a known Feishu/Lark domain. If this is intentional, set allow_custom_base_url(true).",
                self.base_url
            );
            return Err(crate::error::validation_error(
                "base_url",
                "base_url 域名不在白名单中，已知域名: *.feishu.cn, *.larksuite.com, *.larkoffice.com。如需使用自定义域名，请设置 allow_custom_base_url(true)",
            ));
        }

        if self.timeout.is_zero() {
            return Err(crate::error::validation_error(
                "timeout",
                "timeout必须大于0",
            ));
        }

        if self.retry_count > 10 {
            return Err(crate::error::validation_error(
                "retry_count",
                "retry_count不能超过10",
            ));
        }

        Ok(())
    }

    pub(crate) fn build_core_config(&self) -> CoreConfig {
        CoreConfig::builder()
            .app_id(self.app_id.clone())
            .app_secret(self.app_secret.clone())
            .base_url(self.base_url.clone())
            .app_type(self.app_type)
            .enable_token_cache(self.enable_token_cache)
            .req_timeout(self.timeout)
            .max_response_size(self.max_response_size)
            .retry_count(self.retry_count)
            .enable_log(self.enable_log)
            .header(self.headers.clone())
            .build()
    }

    fn apply_env_var(&mut self, key: &str, value: &str) {
        match key {
            "OPENLARK_APP_ID" if !value.is_empty() => self.app_id(value),
            "OPENLARK_APP_SECRET" if !value.is_empty() => self.app_secret(value),
            "OPENLARK_APP_TYPE" => self.apply_app_type(value),
            "OPENLARK_BASE_URL" if !value.is_empty() => self.base_url(value),
            "OPENLARK_ENABLE_TOKEN_CACHE" => {
                let normalized = value.trim().to_lowercase();
                if !normalized.is_empty() {
                    self.enable_token_cache(!(normalized.starts_with('f') || normalized == "0"));
                }
            }
            "OPENLARK_TIMEOUT" => {
                if let Ok(timeout_secs) = value.parse::<u64>() {
                    self.timeout(Duration::from_secs(timeout_secs));
                }
            }
            "OPENLARK_RETRY_COUNT" => {
                if let Ok(retry_count) = value.parse::<u32>() {
                    self.retry_count(retry_count);
                }
            }
            "OPENLARK_MAX_RESPONSE_SIZE" => {
                if let Ok(size) = value.parse::<u64>() {
                    self.max_response_size(size);
                }
            }
            "OPENLARK_ENABLE_LOG" => self.enable_log(!value.to_lowercase().starts_with('f')),
            _ => {}
        }
    }

    fn apply_app_type(&mut self, value: &str) {
        match value.trim().to_lowercase().as_str() {
            "self_build" | "selfbuild" | "self" => self.app_type(AppType::SelfBuild),
            "marketplace" | "store" => self.app_type(AppType::Marketplace),
            _ => {}
        }
    }
}

pub(crate) fn validate_core_config(config: &CoreConfig) -> Result<()> {
    if config.app_id().is_empty() {
        return Err(crate::error::validation_error("app_id", "app_id不能为空"));
    }

    if config.app_secret().is_empty() {
        return Err(crate::error::validation_error(
            "app_secret",
            "app_secret不能为空",
        ));
    }

    validate_base_url(config.base_url())?;

    if config
        .req_timeout()
        .is_some_and(|timeout| timeout.is_zero())
    {
        return Err(crate::error::validation_error(
            "timeout",
            "timeout必须大于0",
        ));
    }

    if config.retry_count() > 10 {
        return Err(crate::error::validation_error(
            "retry_count",
            "retry_count不能超过10",
        ));
    }

    Ok(())
}

fn validate_base_url(base_url: &str) -> Result<()> {
    if base_url.is_empty() {
        return Err(crate::error::validation_error(
            "base_url",
            "base_url不能为空",
        ));
    }

    if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
        return Err(crate::error::validation_error(
            "base_url",
            "base_url必须以http://或https://开头",
        ));
    }

    Ok(())
}
