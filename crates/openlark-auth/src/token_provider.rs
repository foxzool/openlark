//! openlark-auth 的 TokenProvider 实现
//!
//! `openlark-core` 通过 `TokenProvider` 抽象获取 token，而不关心具体获取/刷新/缓存策略。
//! 这里提供一个带缓存的实现：缓存 token 并在过期前复用。

use crate::auth::auth::v3::auth::{
    AppAccessTokenInternalRequestBuilder, AppAccessTokenRequestBuilder,
    TenantAccessTokenInternalRequestBuilder, TenantAccessTokenRequestBuilder,
};
use openlark_core::{
    SDKResult,
    auth::{TokenProvider, TokenRequest},
    config::Config,
    constants::{AccessTokenType, AppType},
    error::configuration_error,
};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

/// 缓存的 token 信息
#[derive(Clone)]
struct CachedToken {
    /// token 值
    token: String,
    /// 过期时间戳（Unix 时间戳，秒）
    expires_at: i64,
}

impl std::fmt::Debug for CachedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CachedToken")
            .field("token", &"***")
            .field("expires_at", &self.expires_at)
            .finish()
    }
}

impl CachedToken {
    fn now_epoch_secs() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }

    /// 创建新的缓存 token
    fn new(token: String, expires_in_seconds: i64) -> Self {
        let now = Self::now_epoch_secs();
        // 提前 60 秒过期，避免临界情况（小于 60 秒则视为立即过期）
        let expires_at = now.saturating_add(expires_in_seconds.saturating_sub(60));

        Self { token, expires_at }
    }

    /// 检查 token 是否已过期
    fn is_expired(&self) -> bool {
        Self::now_epoch_secs() >= self.expires_at
    }
}

/// 基于 openlark-auth API 的 TokenProvider（带缓存）
#[derive(Debug)]
pub struct AuthTokenProvider {
    config: Config,
    /// token 缓存：key 为 token 类型字符串，value 为缓存的 token
    cache: Arc<RwLock<HashMap<String, CachedToken>>>,
}

impl Clone for AuthTokenProvider {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            cache: Arc::clone(&self.cache),
        }
    }
}

impl AuthTokenProvider {
    /// 创建基于 openlark-auth API 的 TokenProvider 实例
    ///
    /// # 参数
    /// - `config`: SDK 配置信息
    pub fn new(config: Config) -> Self {
        Self {
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 生成缓存键
    fn cache_key_component(value: &str) -> String {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

    fn cache_key(
        token_type: &AccessTokenType,
        app_type: &AppType,
        request: &TokenRequest,
    ) -> String {
        match token_type {
            AccessTokenType::Tenant => {
                let tenant_key = request.tenant_key.as_deref().unwrap_or("default");
                format!("{token_type:?}_{app_type:?}_{tenant_key}")
            }
            AccessTokenType::App if app_type == &AppType::Marketplace => {
                let app_ticket = request.app_ticket.as_deref().unwrap_or("default");
                let app_ticket_key = Self::cache_key_component(app_ticket);
                format!("{token_type:?}_{app_type:?}_{app_ticket_key}")
            }
            _ => format!("{token_type:?}_{app_type:?}"),
        }
    }

    async fn get_cached(&self, cache_key: &str) -> Option<String> {
        let cache = self.cache.read().await;
        cache
            .get(cache_key)
            .filter(|cached| !cached.is_expired())
            .map(|cached| cached.token.clone())
    }

    async fn set_cached(&self, cache_key: String, token: String, expires_in_seconds: i64) {
        let cached = CachedToken::new(token, expires_in_seconds);
        self.cache.write().await.insert(cache_key, cached);
    }

    async fn get_or_fetch<F, Fut>(&self, cache_key: String, fetch: F) -> SDKResult<String>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = SDKResult<(String, i64)>>,
    {
        if let Some(token) = self.get_cached(&cache_key).await {
            return Ok(token);
        }

        let (token, expires_in_seconds) = fetch().await?;
        self.set_cached(cache_key, token.clone(), expires_in_seconds)
            .await;
        Ok(token)
    }
}

impl TokenProvider for AuthTokenProvider {
    fn get_token(
        &self,
        request: TokenRequest,
    ) -> Pin<Box<dyn Future<Output = SDKResult<String>> + Send + '_>> {
        Box::pin(async move {
            match request.token_type {
                AccessTokenType::App => {
                    let cache_key =
                        Self::cache_key(&AccessTokenType::App, &self.config.app_type(), &request);
                    self.get_or_fetch(cache_key, || async {
                        let (token, expires_in) = match self.config.app_type() {
                            AppType::SelfBuild => {
                                let resp = AppAccessTokenInternalRequestBuilder::new(
                                    self.config.clone(),
                                )
                                .app_id(self.config.app_id())
                                .app_secret(self.config.app_secret())
                                .execute()
                                .await?;
                                (resp.data.app_access_token, resp.data.expires_in as i64)
                            }
                            AppType::Marketplace => {
                                let app_ticket = request.app_ticket.clone().ok_or_else(|| {
                                    configuration_error(
                                        "token_provider: marketplace app requires app_ticket to fetch app_access_token",
                                    )
                                })?;
                                let resp =
                                    AppAccessTokenRequestBuilder::new(self.config.clone())
                                        .app_id(self.config.app_id())
                                        .app_secret(self.config.app_secret())
                                        .app_ticket(app_ticket)
                                        .execute()
                                        .await?;
                                (resp.data.app_access_token, resp.data.expires_in as i64)
                            }
                        };
                        Ok((token, expires_in))
                    })
                    .await
                }
                AccessTokenType::Tenant => {
                    let cache_key = Self::cache_key(
                        &AccessTokenType::Tenant,
                        &self.config.app_type(),
                        &request,
                    );
                    self.get_or_fetch(cache_key, || async {
                        let (token, expires_in) = match self.config.app_type() {
                            AppType::SelfBuild => {
                                let resp = TenantAccessTokenInternalRequestBuilder::new(
                                    self.config.clone(),
                                )
                                .app_id(self.config.app_id())
                                .app_secret(self.config.app_secret())
                                .execute()
                                .await?;
                                (resp.data.tenant_access_token, resp.data.expires_in as i64)
                            }
                            AppType::Marketplace => {
                                let tenant_key = request.tenant_key.clone().ok_or_else(|| {
                                    configuration_error(
                                        "token_provider: marketplace app requires tenant_key to fetch tenant_access_token",
                                    )
                                })?;
                                let app_ticket = request.app_ticket.clone().ok_or_else(|| {
                                    configuration_error(
                                        "token_provider: marketplace app requires app_ticket to fetch tenant_access_token",
                                    )
                                })?;
                                let app_access_token =
                                    self.get_token(TokenRequest::app().app_ticket(app_ticket))
                                        .await?;

                                let resp =
                                    TenantAccessTokenRequestBuilder::new(self.config.clone())
                                        .app_access_token(app_access_token)
                                        .tenant_key(tenant_key)
                                        .execute()
                                        .await?;
                                (resp.data.tenant_access_token, resp.data.expires_in as i64)
                            }
                        };
                        Ok((token, expires_in))
                    })
                    .await
                }
                AccessTokenType::User => Err(configuration_error(
                    "token_provider: user token 不应由 core 自动获取，请在 RequestOption 中显式传入 user_access_token（或由上层自行实现 TokenProvider 扩展）。",
                )),
                AccessTokenType::None => Err(configuration_error(
                    "token_provider: AccessTokenType::None 不应触发 token 获取",
                )),
            }
        })
    }
}

#[cfg(test)]
mod tests;
