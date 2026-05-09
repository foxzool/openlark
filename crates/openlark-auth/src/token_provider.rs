//! openlark-auth 的 TokenProvider 实现
//!
//! `openlark-core` 通过 `TokenProvider` 抽象获取 token，而不关心具体获取/刷新/缓存策略。
//! 这里提供一个带缓存的实现：缓存 token 并在过期前复用。

use openlark_core::{
    SDKResult,
    auth::{TokenProvider, TokenRequest},
    config::Config,
    constants::{AccessTokenType, AppType},
    error::{api_error, configuration_error},
};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::future::Future;
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
                format!("{token_type:?}_{app_type:?}_{app_ticket}")
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

    async fn fetch_token_via_http(
        &self,
        endpoint: &str,
        payload: Value,
        token_field: &str,
    ) -> SDKResult<(String, i64)> {
        let url = format!(
            "{}/{}",
            self.config.base_url().trim_end_matches('/'),
            endpoint.trim_start_matches('/')
        );

        let response = self
            .config
            .http_client()
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| api_error(500, endpoint, format!("请求飞书认证接口失败: {e}"), None))?;

        let status = response.status().as_u16();
        let body: Value = response
            .json()
            .await
            .map_err(|e| api_error(status, endpoint, format!("解析飞书认证响应失败: {e}"), None))?;

        let code = body.get("code").and_then(Value::as_i64).unwrap_or(-1);
        if code != 0 {
            let msg = body
                .get("msg")
                .and_then(Value::as_str)
                .unwrap_or("未知错误");
            return Err(api_error(
                status,
                endpoint,
                format!("飞书认证接口返回错误: code={code}, msg={msg}"),
                None,
            ));
        }

        let token = body
            .get(token_field)
            .and_then(Value::as_str)
            .ok_or_else(|| configuration_error(format!("飞书认证响应缺少字段: {token_field}")))?
            .to_string();

        let expires_in = body.get("expire").and_then(Value::as_i64).unwrap_or(7200);

        Ok((token, expires_in))
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
                                self.fetch_token_via_http(
                                    "/open-apis/auth/v3/app_access_token/internal",
                                    json!({
                                        "app_id": self.config.app_id(),
                                        "app_secret": self.config.app_secret(),
                                    }),
                                    "app_access_token",
                                )
                                .await?
                            }
                            AppType::Marketplace => {
                                self.fetch_token_via_http(
                                    "/open-apis/auth/v3/app_access_token",
                                    json!({
                                        "app_id": self.config.app_id(),
                                        "app_secret": self.config.app_secret(),
                                    }),
                                    "app_access_token",
                                )
                                .await?
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
                            self.fetch_token_via_http(
                                "/open-apis/auth/v3/tenant_access_token/internal",
                                json!({
                                    "app_id": self.config.app_id(),
                                    "app_secret": self.config.app_secret(),
                                }),
                                "tenant_access_token",
                            )
                            .await?
                        }
                        AppType::Marketplace => {
                            let app_ticket = request.app_ticket.clone().ok_or_else(|| {
                                configuration_error(
                                    "token_provider: marketplace app requires app_ticket to fetch tenant_access_token",
                                )
                            })?;

                            self.fetch_token_via_http(
                                "/open-apis/auth/v3/tenant_access_token",
                                json!({
                                    "app_id": self.config.app_id(),
                                    "app_secret": self.config.app_secret(),
                                    "app_ticket": app_ticket,
                                }),
                                "tenant_access_token",
                            )
                            .await?
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
#[allow(unused_imports)]
mod tests {
    use super::AuthTokenProvider;
    use openlark_core::{
        auth::{TokenProvider, TokenRequest},
        config::Config,
        constants::AppType,
    };

    #[tokio::test]
    async fn tenant_token_fetch_no_longer_uses_noop_provider() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("http://127.0.0.1:9")
            .build();

        let provider = AuthTokenProvider::new(config);
        let err = provider
            .get_token(TokenRequest::tenant())
            .await
            .expect_err("should fail on unreachable test endpoint");

        assert!(!err.to_string().contains("NoOpTokenProvider"));
    }

    #[tokio::test]
    async fn tenant_cache_key_should_include_tenant_key() {
        let request = TokenRequest::tenant().tenant_key("tenant_key_001");

        let key = AuthTokenProvider::cache_key(
            &openlark_core::constants::AccessTokenType::Tenant,
            &AppType::SelfBuild,
            &request,
        );

        assert_eq!(key, "Tenant_SelfBuild_tenant_key_001");
    }

    #[tokio::test]
    async fn app_cache_key_should_include_app_ticket_for_marketplace() {
        let request = TokenRequest::app().app_ticket("ticket_001");

        let key = AuthTokenProvider::cache_key(
            &openlark_core::constants::AccessTokenType::App,
            &AppType::Marketplace,
            &request,
        );

        assert_eq!(key, "App_Marketplace_ticket_001");
    }
}
