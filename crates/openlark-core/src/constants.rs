use std::fmt::Display;

/// 应用类型
#[derive(Default, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum AppType {
    /// 自建应用
    #[default]
    SelfBuild,
    /// 商店应用
    Marketplace,
}

impl AppType {
    /// 获取应用类型字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            AppType::SelfBuild => "self_build",
            AppType::Marketplace => "marketplace",
        }
    }
}

impl Display for AppType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 飞书 API 路径前缀
pub const API_PATH_PREFIX: &str = "/open-apis/";
/// 应用凭证内部接口 URL 路径
pub const APP_ACCESS_TOKEN_INTERNAL_URL_PATH: &str = "/open-apis/auth/v3/app_access_token/internal";
/// 应用凭证 URL 路径
pub const APP_ACCESS_TOKEN_URL_PATH: &str = "/open-apis/auth/v3/app_access_token";
/// 租户凭证内部接口 URL 路径
pub const TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH: &str =
    "/open-apis/auth/v3/tenant_access_token/internal";
/// 租户凭证 URL 路径
pub const TENANT_ACCESS_TOKEN_URL_PATH: &str = "/open-apis/auth/v3/tenant_access_token";
/// 申请应用票据 URL 路径
pub const APPLY_APP_TICKET_PATH: &str = "/open-apis/auth/v3/app_ticket/resend";

#[derive(Default, Hash, Eq, PartialEq, Debug, Copy, Clone)]
/// 访问令牌类型
///
/// 定义不同的访问令牌类型，用于API认证
pub enum AccessTokenType {
    /// 无访问令牌
    #[default]
    None,
    /// 应用访问令牌
    App,
    /// 租户访问令牌
    Tenant,
    /// 用户访问令牌
    User,
}

impl AccessTokenType {
    /// 获取访问令牌类型字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            AccessTokenType::None => "none_access_token",
            AccessTokenType::App => "app_access_token",
            AccessTokenType::Tenant => "tenant_access_token",
            AccessTokenType::User => "user_access_token",
        }
    }
}

impl Display for AccessTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 项目名称
pub const PROJECT: &str = "open-lark";
/// 项目版本号（来自 Cargo.toml）
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Domain
pub const FEISHU_BASE_URL: &str = "https://open.feishu.cn";
/// 飞书国际版 Lark 域名
pub const LARK_BASE_URL: &str = "https://open.larksuite.com";

/// 默认 Content-Type（JSON 编码）
pub const DEFAULT_CONTENT_TYPE: &str = "application/json; charset=utf-8";
/// 文件上传 Content-Type
pub const FILE_CONTENT_TYPE: &str = "multipart/form-data";
/// User-Agent HTTP 头名称
pub const USER_AGENT_HEADER: &str = "User-Agent";

/// X-Request-Id HTTP 请求头键名
pub const HTTP_HEADER_KEY_REQUEST_ID: &str = "X-Request-Id";
/// Request-Id HTTP 请求头键名
pub const HTTP_HEADER_REQUEST_ID: &str = "Request-Id";

/// X-Tt-Logid HTTP 请求头键名
pub const HTTP_HEADER_KEY_LOG_ID: &str = "X-Tt-Logid";
/// Content-Type HTTP 头名称
pub const CONTENT_TYPE_HEADER: &str = "Content-Type";
/// JSON Content-Type 值
pub const CONTENT_TYPE_JSON: &str = "application/json";
/// 自定义请求 ID 头名称
pub const CUSTOM_REQUEST_ID: &str = "Open-Lark-Request-Id";
/// 应用票据缓存键前缀
pub const APP_TICKET_KEY_PREFIX: &str = "app_ticket";
/// 应用访问令牌缓存键前缀
pub const APP_ACCESS_TOKEN_KEY_PREFIX: &str = "app_access_token";
/// 租户访问令牌缓存键前缀
pub const TENANT_ACCESS_TOKEN_KEY_PREFIX: &str = "tenant_access_token";
/// 令牌过期时间增量（秒），用于提前刷新令牌
pub const EXPIRY_DELTA: i32 = 60 * 3;
/// 应用票据无效错误码
pub const ERR_CODE_APP_TICKET_INVALID: i32 = 10012;
/// 访问令牌无效错误码
pub const ERR_CODE_ACCESS_TOKEN_INVALID: i32 = 99991671;
/// 应用访问令牌无效错误码
pub const ERR_CODE_APP_ACCESS_TOKEN_INVALID: i32 = 99991664;
/// 租户访问令牌无效错误码
pub const ERR_CODE_TENANT_ACCESS_TOKEN_INVALID: i32 = 99991663;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_type_default() {
        assert_eq!(AppType::default(), AppType::SelfBuild);
    }

    #[test]
    fn test_app_type_debug_clone() {
        let app_type = AppType::Marketplace;
        let cloned = app_type;
        assert_eq!(format!("{app_type:?}"), "Marketplace");
        assert_eq!(cloned, AppType::Marketplace);
    }

    #[test]
    fn test_app_type_equality() {
        assert_eq!(AppType::SelfBuild, AppType::SelfBuild);
        assert_eq!(AppType::Marketplace, AppType::Marketplace);
        assert_ne!(AppType::SelfBuild, AppType::Marketplace);
    }

    #[test]
    fn test_access_token_type_default() {
        assert_eq!(AccessTokenType::default(), AccessTokenType::None);
    }

    #[test]
    fn test_access_token_type_display() {
        assert_eq!(AccessTokenType::None.to_string(), "none_access_token");
        assert_eq!(AccessTokenType::App.to_string(), "app_access_token");
        assert_eq!(AccessTokenType::Tenant.to_string(), "tenant_access_token");
        assert_eq!(AccessTokenType::User.to_string(), "user_access_token");
    }

    #[test]
    fn test_access_token_type_debug_clone() {
        let token_type = AccessTokenType::App;
        let cloned = token_type;
        assert_eq!(format!("{token_type:?}"), "App");
        assert_eq!(cloned, AccessTokenType::App);
    }

    #[test]
    fn test_access_token_type_equality() {
        assert_eq!(AccessTokenType::None, AccessTokenType::None);
        assert_eq!(AccessTokenType::App, AccessTokenType::App);
        assert_eq!(AccessTokenType::Tenant, AccessTokenType::Tenant);
        assert_eq!(AccessTokenType::User, AccessTokenType::User);
        assert_ne!(AccessTokenType::App, AccessTokenType::Tenant);
    }

    #[test]
    fn test_constants_values() {
        // Test URL paths
        assert_eq!(
            APP_ACCESS_TOKEN_INTERNAL_URL_PATH,
            "/open-apis/auth/v3/app_access_token/internal"
        );
        assert_eq!(
            APP_ACCESS_TOKEN_URL_PATH,
            "/open-apis/auth/v3/app_access_token"
        );
        assert_eq!(
            TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH,
            "/open-apis/auth/v3/tenant_access_token/internal"
        );
        assert_eq!(
            TENANT_ACCESS_TOKEN_URL_PATH,
            "/open-apis/auth/v3/tenant_access_token"
        );
        assert_eq!(
            APPLY_APP_TICKET_PATH,
            "/open-apis/auth/v3/app_ticket/resend"
        );

        // Test project info
        assert_eq!(PROJECT, "open-lark");
        // VERSION is a const from env!() so it's never empty - verify it's a valid version
        assert!(
            VERSION.contains('.'),
            "Version should contain dots for proper semver"
        );

        // Test base URLs
        assert_eq!(FEISHU_BASE_URL, "https://open.feishu.cn");
        assert_eq!(LARK_BASE_URL, "https://open.larksuite.com");

        // Test content types
        assert_eq!(DEFAULT_CONTENT_TYPE, "application/json; charset=utf-8");
        assert_eq!(FILE_CONTENT_TYPE, "multipart/form-data");
        assert_eq!(CONTENT_TYPE_JSON, "application/json");

        // Test headers
        assert_eq!(USER_AGENT_HEADER, "User-Agent");
        assert_eq!(HTTP_HEADER_KEY_REQUEST_ID, "X-Request-Id");
        assert_eq!(HTTP_HEADER_REQUEST_ID, "Request-Id");
        assert_eq!(HTTP_HEADER_KEY_LOG_ID, "X-Tt-Logid");
        assert_eq!(CONTENT_TYPE_HEADER, "Content-Type");
        assert_eq!(CUSTOM_REQUEST_ID, "Open-Lark-Request-Id");

        // Test cache key prefixes
        assert_eq!(APP_TICKET_KEY_PREFIX, "app_ticket");
        assert_eq!(APP_ACCESS_TOKEN_KEY_PREFIX, "app_access_token");
        assert_eq!(TENANT_ACCESS_TOKEN_KEY_PREFIX, "tenant_access_token");

        // Test numeric constants
        assert_eq!(EXPIRY_DELTA, 60 * 3);
        assert_eq!(ERR_CODE_APP_TICKET_INVALID, 10012);
        assert_eq!(ERR_CODE_ACCESS_TOKEN_INVALID, 99991671);
        assert_eq!(ERR_CODE_APP_ACCESS_TOKEN_INVALID, 99991664);
        assert_eq!(ERR_CODE_TENANT_ACCESS_TOKEN_INVALID, 99991663);
    }

    #[test]
    fn test_version_format() {
        // VERSION should follow semver format
        let version_parts: Vec<&str> = VERSION.split('.').collect();
        assert!(
            version_parts.len() >= 2,
            "Version should have at least major.minor format"
        );

        // Each part should be numeric
        for part in &version_parts {
            assert!(!part.is_empty(), "Version parts should not be empty");
            // Basic check for digits (might have pre-release suffixes)
            assert!(
                part.chars().next().unwrap().is_ascii_digit(),
                "Version should start with digit"
            );
        }
    }

    #[test]
    fn test_url_constants_format() {
        // All URLs should start with proper prefixes
        assert!(FEISHU_BASE_URL.starts_with("https://"));
        assert!(LARK_BASE_URL.starts_with("https://"));

        // API paths should start with /open-apis/
        assert!(APP_ACCESS_TOKEN_INTERNAL_URL_PATH.starts_with("/open-apis/"));
        assert!(APP_ACCESS_TOKEN_URL_PATH.starts_with("/open-apis/"));
        assert!(TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH.starts_with("/open-apis/"));
        assert!(TENANT_ACCESS_TOKEN_URL_PATH.starts_with("/open-apis/"));
        assert!(APPLY_APP_TICKET_PATH.starts_with("/open-apis/"));
    }

    #[test]
    fn test_error_code_ranges() {
        // Error codes should be in expected ranges
        // These are const assertions that will be optimized out
        // Removed to avoid clippy warnings about constant assertions

        // Different error codes should be unique
        let error_codes = [
            ERR_CODE_APP_TICKET_INVALID,
            ERR_CODE_ACCESS_TOKEN_INVALID,
            ERR_CODE_APP_ACCESS_TOKEN_INVALID,
            ERR_CODE_TENANT_ACCESS_TOKEN_INVALID,
        ];

        for (i, &code1) in error_codes.iter().enumerate() {
            for &code2 in error_codes.iter().skip(i + 1) {
                assert_ne!(code1, code2, "Error codes should be unique");
            }
        }
    }

    #[test]
    fn test_expiry_delta_reasonable() {
        // EXPIRY_DELTA should be reasonable (3 minutes in seconds)
        assert_eq!(EXPIRY_DELTA, 180);
        // These are const assertions that will be optimized out
        // Removed to avoid clippy warnings about constant assertions
    }

    #[test]
    fn test_content_type_formats() {
        assert!(DEFAULT_CONTENT_TYPE.contains("application/json"));
        assert!(DEFAULT_CONTENT_TYPE.contains("charset=utf-8"));
        assert!(FILE_CONTENT_TYPE.contains("multipart/form-data"));
        assert_eq!(CONTENT_TYPE_JSON, "application/json");
    }
}
