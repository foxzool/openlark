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
#[allow(clippy::field_reassign_with_default)]
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

    // --- determine_token_type / validate_token_type 测试（从 http.rs 迁入；ADR-0002「测试跟着搬」）---

    #[test]
    fn test_validate_token_type_empty_list_no_panic() {
        let empty_types: Vec<AccessTokenType> = vec![];
        let option = RequestOption::default();

        // 空列表不应 panic，且不做额外校验。
        let result = validate_token_type(&empty_types, &option);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_token_type_non_empty_list_returns_ok() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let option = RequestOption::default();

        // 列表非空时应进行校验（当前仅对 Tenant/App 的 token 冲突做约束）
        let result = validate_token_type(&types, &option);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_token_type_tenant_with_user_token() {
        let types = vec![AccessTokenType::Tenant];
        let option = RequestOption {
            user_access_token: Some("user_token".to_string()),
            ..Default::default()
        };

        let result = validate_token_type(&types, &option);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_token_type_app_with_tenant_token() {
        let types = vec![AccessTokenType::App];
        let option = RequestOption {
            tenant_access_token: Some("tenant_token".to_string()),
            ..Default::default()
        };

        let result = validate_token_type(&types, &option);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_token_type_valid_combinations() {
        let types = vec![AccessTokenType::User];
        let mut option = RequestOption::default();
        option.user_access_token = Some("user_token".to_string());

        let result = validate_token_type(&types, &option);
        assert!(result.is_ok());
    }

    #[test]
    fn test_determine_token_type_no_cache_user() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let mut option = RequestOption::default();
        option.user_access_token = Some("user_token".to_string());

        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::User);
    }

    #[test]
    fn test_determine_token_type_no_cache_tenant() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let option = RequestOption {
            tenant_access_token: Some("tenant_token".to_string()),
            ..Default::default()
        };

        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_determine_token_type_no_cache_app() {
        let types = vec![AccessTokenType::App, AccessTokenType::Tenant];
        let option = RequestOption {
            app_access_token: Some("app_token".to_string()),
            ..Default::default()
        };

        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::App);
    }

    #[test]
    fn test_determine_token_type_no_cache_none() {
        let types = vec![AccessTokenType::None];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::None);
    }

    #[test]
    fn test_determine_token_type_with_cache_defaults_to_tenant() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_determine_token_type_with_cache_tenant_key() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let mut option = RequestOption::default();
        option.tenant_key = Some("tenant_key".to_string());

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_determine_token_type_with_cache_user_access_token() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let mut option = RequestOption::default();
        option.user_access_token = Some("user_token".to_string());

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::User);
    }

    #[test]
    fn test_determine_token_type_first_is_tenant() {
        let types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_determine_token_type_no_tenant_in_list() {
        let types = vec![AccessTokenType::User, AccessTokenType::App];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, true);
        // Should use first type when no Tenant type is available
        assert_eq!(token_type, AccessTokenType::User);
    }

    #[test]
    fn test_validate_token_type_edge_case_single_element() {
        let types = vec![AccessTokenType::None];
        let mut option = RequestOption::default();
        option.user_access_token = Some("user_token".to_string());

        let result = validate_token_type(&types, &option);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_token_type_non_empty_list_ok() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let option = RequestOption::default();

        let result = validate_token_type(&types, &option);
        assert!(result.is_ok());
    }

    #[test]
    fn test_determine_token_type_priority_with_multiple_tokens() {
        let types = vec![
            AccessTokenType::User,
            AccessTokenType::Tenant,
            AccessTokenType::App,
        ];
        let mut option = RequestOption::default();
        option.user_access_token = Some("user_token".to_string());
        option.tenant_key = Some("tenant_key".to_string());

        // User token should take priority when present
        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::User);
    }

    #[test]
    fn test_determine_token_type_tenant_key_without_tenant_type() {
        let types = vec![AccessTokenType::User, AccessTokenType::App];
        let mut option = RequestOption::default();
        option.tenant_key = Some("tenant_key".to_string());

        // Should default to first type when Tenant not available
        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::User);
    }

    #[test]
    fn test_determine_token_type_user_token_without_user_type() {
        let types = vec![AccessTokenType::Tenant, AccessTokenType::App];
        let mut option = RequestOption::default();
        option.user_access_token = Some("user_token".to_string());

        // Should use Tenant when User type not available
        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_determine_token_type_cache_disabled_fallback_priority() {
        let types = vec![
            AccessTokenType::User,
            AccessTokenType::Tenant,
            AccessTokenType::App,
        ];
        let mut option = RequestOption::default();
        option.tenant_access_token = Some("tenant_token".to_string());
        option.app_access_token = Some("app_token".to_string());

        // Tenant should be chosen over App when cache disabled
        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::Tenant);
    }

    #[test]
    fn test_determine_token_type_cache_disabled_all_empty() {
        let types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let option = RequestOption::default();

        // Should return None when no tokens provided and cache disabled
        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::None);
    }

    #[test]
    fn test_determine_token_type_empty_types_list_no_panic() {
        let types: Vec<AccessTokenType> = vec![];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::None);
    }

    #[test]
    fn test_determine_token_type_empty_types_list_no_cache() {
        // When cache is disabled, empty types list works fine
        let types: Vec<AccessTokenType> = vec![];
        let option = RequestOption::default();

        // This works because cache-disabled path returns None without accessing types[0]
        let token_type = determine_token_type(&types, &option, false);
        assert_eq!(token_type, AccessTokenType::None);
    }

    #[test]
    fn test_determine_token_type_single_app_type() {
        let types = vec![AccessTokenType::App];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::App);
    }

    #[test]
    fn test_determine_token_type_single_none_type() {
        let types = vec![AccessTokenType::None];
        let option = RequestOption::default();

        let token_type = determine_token_type(&types, &option, true);
        assert_eq!(token_type, AccessTokenType::None);
    }

    #[test]
    fn test_validate_token_type_with_mismatched_tokens_simulation() {
        // types 与 option 中显式 token 类型不匹配时，应返回错误（避免静默放过错误 token）。
        let types = vec![AccessTokenType::Tenant];
        let mut option = RequestOption::default();
        option.user_access_token = Some("user_token".to_string()); // Mismatch!

        let result = validate_token_type(&types, &option);
        assert!(result.is_err());
    }
}
