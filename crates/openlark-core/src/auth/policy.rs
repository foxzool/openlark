//! 鉴权策略（ADR-0002）：token 类型决策 + 授权校验。
//!
//! 「给定请求选项，用哪种 token；选定的 token 鉴权是否齐备」——从 `http.rs` 收敛至此，
//! 使 `Transport` 聚焦 HTTP，鉴权 concern 与获取（`acquisition`）/ 恢复（`app_ticket`）同居 `auth/`。

use std::collections::HashSet;

use crate::{
    config::Config,
    constants::{AccessTokenType, AppType},
    error::CoreError,
    req_option::RequestOption,
};

/// 校验声明的可用 token 类型与显式传入的 token 是否冲突（原 `http.rs::validate_token_type`）。
pub(crate) fn validate_token_type(
    access_token_types: &[AccessTokenType],
    option: &RequestOption,
) -> Result<(), CoreError> {
    // 未指定可用 token 类型时，不做额外校验。
    // 旧实现误将"非空"作为提前返回条件，并在空列表时访问 [0] 导致 panic。
    if access_token_types.is_empty() {
        return Ok(());
    }

    let access_token_type = access_token_types[0];

    if access_token_type == AccessTokenType::Tenant && option.user_access_token.is_some() {
        return Err(crate::error::validation_error(
            "access_token_type",
            "tenant token type not match user access token",
        ));
    }

    if access_token_type == AccessTokenType::App && option.tenant_access_token.is_some() {
        return Err(crate::error::validation_error(
            "access_token_type",
            "user token type not match tenant access token",
        ));
    }

    Ok(())
}

/// 决定本次请求使用哪种 token（原 `http.rs::determine_token_type`）。
pub(crate) fn determine_token_type(
    access_token_types: &[AccessTokenType],
    option: &RequestOption,
    enable_token_cache: bool,
) -> AccessTokenType {
    if !enable_token_cache {
        if !access_token_types.is_empty() {
            for access_token_type in access_token_types.iter() {
                match access_token_type {
                    AccessTokenType::User => {
                        if option.user_access_token.is_some() {
                            return AccessTokenType::User;
                        }
                    }
                    AccessTokenType::Tenant => {
                        if option.tenant_access_token.is_some() || option.tenant_key.is_some() {
                            return AccessTokenType::Tenant;
                        }
                    }
                    AccessTokenType::App => {
                        if option.app_access_token.is_some() {
                            return AccessTokenType::App;
                        }
                    }
                    AccessTokenType::None => {}
                }
            }

            return AccessTokenType::None;
        }

        if option.user_access_token.is_some() {
            return AccessTokenType::User;
        }
        if option.tenant_access_token.is_some() {
            return AccessTokenType::Tenant;
        }
        if option.app_access_token.is_some() {
            return AccessTokenType::App;
        }

        return AccessTokenType::None;
    }

    // 缓存开启但未指定 token 类型时，退回到“按显式传入的 token”推断，避免空列表 panic。
    if access_token_types.is_empty() {
        if option.user_access_token.is_some() {
            return AccessTokenType::User;
        }
        if option.tenant_access_token.is_some() || option.tenant_key.is_some() {
            return AccessTokenType::Tenant;
        }
        if option.app_access_token.is_some() {
            return AccessTokenType::App;
        }
        return AccessTokenType::None;
    }

    let mut accessible_token_type_set: HashSet<AccessTokenType> = HashSet::new();
    let mut access_token_type = access_token_types[0];

    for t in access_token_types {
        if *t == AccessTokenType::Tenant {
            access_token_type = *t; // 默认值
        }
        accessible_token_type_set.insert(*t);
    }

    if option.tenant_key.is_some() && accessible_token_type_set.contains(&AccessTokenType::Tenant) {
        access_token_type = AccessTokenType::Tenant;
    }

    if option.user_access_token.is_some()
        && accessible_token_type_set.contains(&AccessTokenType::User)
    {
        access_token_type = AccessTokenType::User;
    }

    access_token_type
}

/// 授权校验：给定选定的 token 类型，请求的鉴权材料是否齐备。
///
/// 原 `http.rs::validate` 的三块 auth 授权校验（无缓存时 token 必在 /
/// Marketplace+Tenant 必带 tenant_key / User 类型必带 user_access_token）。
/// `validate()` 自身仍保留在 `http.rs` 做 config + header 前置校验并委托本函数。
pub(crate) fn validate_authorization(
    config: &Config,
    option: &RequestOption,
    access_token_type: AccessTokenType,
) -> Result<(), CoreError> {
    if !config.enable_token_cache {
        if access_token_type == AccessTokenType::None {
            return Ok(());
        }
        if option.user_access_token.is_none()
            && option.tenant_access_token.is_none()
            && option.app_access_token.is_none()
        {
            return Err(crate::error::validation_error(
                "access_token",
                "accessToken is empty",
            ));
        }
    }

    if config.app_type == AppType::Marketplace
        && access_token_type == AccessTokenType::Tenant
        && option.tenant_key.is_none()
    {
        return Err(crate::error::validation_error(
            "access_token",
            "accessToken is empty",
        ));
    }

    if access_token_type == AccessTokenType::User && option.user_access_token.is_none() {
        return Err(crate::error::validation_error(
            "user_access_token",
            "user access token is empty",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::AppType;
    use crate::req_option::RequestOption;

    fn config(cache: bool, app_type: AppType) -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(app_type)
            .enable_token_cache(cache)
            .build()
    }

    #[test]
    fn validate_authorization_none_type_no_cache_is_ok() {
        // 无缓存 + None 类型：提前返回 Ok（不要求 token）。
        let cfg = config(false, AppType::SelfBuild);
        assert!(
            validate_authorization(&cfg, &RequestOption::default(), AccessTokenType::None).is_ok()
        );
    }

    #[test]
    fn validate_authorization_user_type_missing_token_errors() {
        let cfg = config(true, AppType::SelfBuild);
        let res = validate_authorization(&cfg, &RequestOption::default(), AccessTokenType::User);
        assert!(matches!(res, Err(CoreError::Validation { .. })));
    }

    #[test]
    fn validate_authorization_user_type_with_token_is_ok() {
        let cfg = config(true, AppType::SelfBuild);
        let option = RequestOption {
            user_access_token: Some("u".to_string()),
            ..Default::default()
        };
        assert!(validate_authorization(&cfg, &option, AccessTokenType::User).is_ok());
    }

    #[test]
    fn validate_authorization_marketplace_tenant_without_key_errors() {
        let cfg = config(true, AppType::Marketplace);
        let res = validate_authorization(&cfg, &RequestOption::default(), AccessTokenType::Tenant);
        assert!(matches!(res, Err(CoreError::Validation { .. })));
    }

    #[test]
    fn validate_authorization_marketplace_tenant_with_key_is_ok() {
        let cfg = config(true, AppType::Marketplace);
        let option = RequestOption {
            tenant_key: Some("tk".to_string()),
            ..Default::default()
        };
        assert!(validate_authorization(&cfg, &option, AccessTokenType::Tenant).is_ok());
    }

    #[test]
    fn validate_authorization_no_cache_app_type_requires_token() {
        let cfg = config(false, AppType::SelfBuild);
        // 无 token → err
        assert!(
            validate_authorization(&cfg, &RequestOption::default(), AccessTokenType::App).is_err()
        );
        // 有 token → ok
        let option = RequestOption {
            app_access_token: Some("a".to_string()),
            ..Default::default()
        };
        assert!(validate_authorization(&cfg, &option, AccessTokenType::App).is_ok());
    }
}
