//! Error Codes Definition
//!
//! 统一的错误码系统，提供标准化的错误标识和映射。

use super::traits::ErrorSeverity;
use serde::{Deserialize, Serialize};
use std::fmt;

/// 标准错误码枚举
///
/// 涵盖所有可能的标准错误情况，包括HTTP状态码、飞书API错误码、
/// 网络错误码、系统错误码等。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    // 成功状态
    /// 成功
    Success = 0,

    // HTTP 标准错误码
    /// 请求参数错误
    BadRequest = 400,
    /// 未认证
    Unauthorized = 401,
    /// 禁止访问
    Forbidden = 403,
    /// 资源不存在
    NotFound = 404,
    /// 方法不允许
    MethodNotAllowed = 405,
    /// 请求冲突
    Conflict = 409,
    /// 请求频率过高
    TooManyRequests = 429,

    // 服务器错误码
    /// 内部服务器错误
    InternalServerError = 500,
    /// 网关错误
    BadGateway = 502,
    /// 服务不可用
    ServiceUnavailable = 503,
    /// 网关超时
    GatewayTimeout = 504,

    // 飞书API错误码 - 认证相关
    /// 应用票据无效
    AppTicketInvalid = 10012,
    /// 应用状态异常
    AppStatusException = 10013,
    /// 应用权限不足
    AppPermissionDenied = 19001,
    /// 访问令牌无效
    AccessTokenInvalid = 99991671,
    /// 访问令牌格式错误/无效
    AccessTokenFormatInvalid = 99991661,
    /// 应用访问令牌无效
    AppAccessTokenInvalid = 99991664,
    /// 租户访问令牌无效
    TenantAccessTokenInvalid = 99991663,
    /// SSO 访问令牌无效
    SsoTokenInvalid = 99991670,
    /// 缺少所需权限
    PermissionMissing = 99991672,
    /// 访问令牌缺少权限
    AccessTokenNoPermission = 99991676,
    /// 访问令牌已过期（飞书通用码）
    AccessTokenExpiredV2 = 99991677,

    // 会话 / 身份类
    /// 用户会话已失效
    UserSessionInvalid = 99991641,
    /// 用户会话不存在
    UserSessionNotFound = 99991642,
    /// 用户会话超时
    UserSessionTimeout = 99991645,
    /// 用户身份解析失败
    UserIdentityInvalid = 99991669,
    /// 用户类型不支持
    UserTypeNotSupportedV2 = 99991674,
    /// 用户身份不匹配
    UserIdentityMismatch = 99991675,
    /// 用户ID非法
    UserIdInvalid = 99992351,
    /// OpenID 非法
    OpenIdInvalid = 99992352,
    /// UnionID 非法
    UnionIdInvalid = 99992353,

    // 飞书API错误码 - 资源相关
    /// 应用未安装
    AppNotInstalled = 10003,
    /// 用户不存在
    UserNotFound = 60001,
    /// 用户状态异常
    UserStatusException = 60002,
    /// 部门不存在
    DepartmentNotFound = 60003,
    /// 群组不存在
    ChatNotFound = 70001,
    /// 群组类型不支持
    ChatTypeNotSupported = 70002,
    /// 消息不存在
    MessageNotFound = 80001,
    /// 消息类型不支持
    MessageTypeNotSupported = 80002,
    /// 文件不存在
    FileNotFound = 90001,
    /// 文件大小超限
    FileSizeExceeded = 90002,
    /// 文件类型不支持
    FileTypeNotSupported = 90003,

    // 飞书API错误码 - 业务相关
    /// 日历不存在
    CalendarNotFound = 110001,
    /// 日程不存在
    EventNotFound = 110002,
    /// 日程冲突
    EventConflict = 110003,
    /// 文档不存在
    DocumentNotFound = 120001,
    /// 文档权限不足
    DocumentPermissionDenied = 120002,
    /// 文档已锁定
    DocumentLocked = 120003,
    /// 工作表不存在
    SheetNotFound = 120011,
    /// 表格不存在
    TableNotFound = 120021,

    // 网络和传输错误码
    /// 网络连接超时
    NetworkTimeout = 999001,
    /// 网络连接失败
    NetworkConnectionFailed = 999002,
    /// DNS解析失败
    DnsResolutionFailed = 999003,
    /// SSL证书错误
    SslCertificateError = 999004,
    /// 连接被拒绝
    ConnectionRefused = 999005,

    // 序列化错误码
    /// JSON解析错误
    SerializationError = 999010,
    /// 数据格式错误
    DataFormatError = 999011,
    /// 编码错误
    EncodingError = 999012,

    // 认证和授权错误码
    /// 身份验证失败
    AuthenticationFailed = 999020,
    /// 权限被拒绝
    PermissionDenied = 999021,
    /// 令牌已过期
    TokenExpired = 999022,
    /// 无效签名
    InvalidSignature = 999023,

    // 验证错误码
    /// 参数验证失败
    ValidationError = 999030,
    /// 缺少必需参数
    MissingRequiredParameter = 999031,
    /// 参数格式错误
    InvalidParameterFormat = 999032,
    /// 参数值超出范围
    ParameterOutOfRange = 999033,

    // 业务逻辑错误码
    /// 业务逻辑错误
    BusinessError = 999040,
    /// 操作不被支持
    OperationNotSupported = 999041,
    /// 资源状态冲突
    ResourceConflict = 999042,

    // 系统错误码
    /// 系统内部错误
    InternalError = 999050,
    /// 配置错误
    ConfigurationError = 999051,
    /// 资源耗尽
    ResourceExhausted = 999052,

    // 限流错误码
    /// 请求频率限制
    RateLimitExceeded = 999060,

    // 响应大小错误码
    /// 响应体大小超过限制
    ResponseTooLarge = 41300,

    // 缓存错误码
    /// 缓存未命中
    CacheMiss = 999070,
    /// 缓存服务不可用
    CacheServiceUnavailable = 999071,

    // 安全错误码
    /// 安全策略违规
    SecurityPolicyViolation = 999080,
    /// 访问被拒绝
    AccessDenied = 999081,

    // 未知错误
    /// 未知错误
    Unknown = 999999,
}

impl ErrorCode {
    /// 从数值创建错误码
    pub fn from_code(code: i32) -> Self {
        match code {
            c if (200..=299).contains(&c) => Self::Success,
            0 => Self::Success,

            // HTTP 标准错误码
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            405 => Self::MethodNotAllowed,
            409 => Self::Conflict,
            429 => Self::TooManyRequests,

            // 服务器错误码
            500 => Self::InternalServerError,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,

            // 飞书API错误码
            10003 => Self::AppNotInstalled,
            10012 => Self::AppTicketInvalid,
            10013 => Self::AppStatusException,
            19001 => Self::AppPermissionDenied,
            60001 => Self::UserNotFound,
            60002 => Self::UserStatusException,
            60003 => Self::DepartmentNotFound,
            70001 => Self::ChatNotFound,
            70002 => Self::ChatTypeNotSupported,
            80001 => Self::MessageNotFound,
            80002 => Self::MessageTypeNotSupported,
            90001 => Self::FileNotFound,
            90002 => Self::FileSizeExceeded,
            90003 => Self::FileTypeNotSupported,
            110001 => Self::CalendarNotFound,
            110002 => Self::EventNotFound,
            110003 => Self::EventConflict,
            120001 => Self::DocumentNotFound,
            120002 => Self::DocumentPermissionDenied,
            120003 => Self::DocumentLocked,
            120011 => Self::SheetNotFound,
            120021 => Self::TableNotFound,
            99991661 => Self::AccessTokenFormatInvalid,
            99991671 => Self::AccessTokenInvalid,
            99991664 => Self::AppAccessTokenInvalid,
            99991663 => Self::TenantAccessTokenInvalid,
            99991670 => Self::SsoTokenInvalid,
            99991672 => Self::PermissionMissing,
            99991676 => Self::AccessTokenNoPermission,
            99991677 => Self::AccessTokenExpiredV2,
            99991641 => Self::UserSessionInvalid,
            99991642 => Self::UserSessionNotFound,
            99991645 => Self::UserSessionTimeout,
            99991669 => Self::UserIdentityInvalid,
            99991674 => Self::UserTypeNotSupportedV2,
            99991675 => Self::UserIdentityMismatch,
            99992351 => Self::UserIdInvalid,
            99992352 => Self::OpenIdInvalid,
            99992353 => Self::UnionIdInvalid,

            // 网络错误码
            999001 => Self::NetworkTimeout,
            999002 => Self::NetworkConnectionFailed,
            999003 => Self::DnsResolutionFailed,
            999004 => Self::SslCertificateError,
            999005 => Self::ConnectionRefused,

            // 序列化错误码
            999010 => Self::SerializationError,
            999011 => Self::DataFormatError,
            999012 => Self::EncodingError,

            // 认证和授权错误码
            999020 => Self::AuthenticationFailed,
            999021 => Self::PermissionDenied,
            999022 => Self::TokenExpired,
            999023 => Self::InvalidSignature,

            // 验证错误码
            999030 => Self::ValidationError,
            999031 => Self::MissingRequiredParameter,
            999032 => Self::InvalidParameterFormat,
            999033 => Self::ParameterOutOfRange,

            // 业务逻辑错误码
            999040 => Self::BusinessError,
            999041 => Self::OperationNotSupported,
            999042 => Self::ResourceConflict,

            // 系统错误码
            999050 => Self::InternalError,
            999051 => Self::ConfigurationError,
            999052 => Self::ResourceExhausted,

            // 限流错误码
            999060 => Self::RateLimitExceeded,

            // 响应大小错误码
            41300 => Self::ResponseTooLarge,

            // 缓存错误码
            999070 => Self::CacheMiss,
            999071 => Self::CacheServiceUnavailable,

            // 安全错误码
            999080 => Self::SecurityPolicyViolation,
            999081 => Self::AccessDenied,

            _ => Self::Unknown,
        }
    }

    /// 获取数值形式
    pub fn as_code(&self) -> i32 {
        *self as i32
    }

    /// 获取错误码的描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::Success => "操作成功",

            // HTTP 标准错误码描述
            Self::BadRequest => "请求参数错误",
            Self::Unauthorized => "未认证",
            Self::Forbidden => "禁止访问",
            Self::NotFound => "资源不存在",
            Self::MethodNotAllowed => "方法不允许",
            Self::Conflict => "请求冲突",
            Self::TooManyRequests => "请求频率过高",

            // 服务器错误码描述
            Self::InternalServerError => "内部服务器错误",
            Self::BadGateway => "网关错误",
            Self::ServiceUnavailable => "服务不可用",
            Self::GatewayTimeout => "网关超时",

            // 飞书API错误码描述
            Self::AppNotInstalled => "应用未安装",
            Self::AppTicketInvalid => "应用票据无效",
            Self::AppStatusException => "应用状态异常",
            Self::AppPermissionDenied => "应用权限不足",
            Self::UserNotFound => "用户不存在",
            Self::UserStatusException => "用户状态异常",
            Self::DepartmentNotFound => "部门不存在",
            Self::ChatNotFound => "群组不存在",
            Self::ChatTypeNotSupported => "群组类型不支持",
            Self::MessageNotFound => "消息不存在",
            Self::MessageTypeNotSupported => "消息类型不支持",
            Self::FileNotFound => "文件不存在",
            Self::FileSizeExceeded => "文件大小超限",
            Self::FileTypeNotSupported => "文件类型不支持",
            Self::CalendarNotFound => "日历不存在",
            Self::EventNotFound => "日程不存在",
            Self::EventConflict => "日程冲突",
            Self::DocumentNotFound => "文档不存在",
            Self::DocumentPermissionDenied => "文档权限不足",
            Self::DocumentLocked => "文档已锁定",
            Self::SheetNotFound => "工作表不存在",
            Self::TableNotFound => "表格不存在",
            Self::AccessTokenFormatInvalid => "访问令牌格式错误",
            Self::AccessTokenInvalid => "访问令牌无效",
            Self::AppAccessTokenInvalid => "应用访问令牌无效",
            Self::TenantAccessTokenInvalid => "租户访问令牌无效",
            Self::SsoTokenInvalid => "SSO 访问令牌无效",
            Self::PermissionMissing => "缺少所需权限",
            Self::AccessTokenNoPermission => "访问令牌权限不足",
            Self::AccessTokenExpiredV2 => "访问令牌已过期",
            Self::UserSessionInvalid => "用户会话已失效",
            Self::UserSessionNotFound => "用户会话不存在",
            Self::UserSessionTimeout => "用户会话超时",
            Self::UserIdentityInvalid => "用户身份解析失败",
            Self::UserTypeNotSupportedV2 => "用户类型不支持",
            Self::UserIdentityMismatch => "用户身份不匹配",
            Self::UserIdInvalid => "用户ID非法",
            Self::OpenIdInvalid => "OpenID 非法",
            Self::UnionIdInvalid => "UnionID 非法",

            // 网络错误码描述
            Self::NetworkTimeout => "网络连接超时",
            Self::NetworkConnectionFailed => "网络连接失败",
            Self::DnsResolutionFailed => "DNS解析失败",
            Self::SslCertificateError => "SSL证书错误",
            Self::ConnectionRefused => "连接被拒绝",

            // 序列化错误码描述
            Self::SerializationError => "JSON解析错误",
            Self::DataFormatError => "数据格式错误",
            Self::EncodingError => "编码错误",

            // 认证和授权错误码描述
            Self::AuthenticationFailed => "身份验证失败",
            Self::PermissionDenied => "权限被拒绝",
            Self::TokenExpired => "令牌已过期",
            Self::InvalidSignature => "无效签名",

            // 验证错误码描述
            Self::ValidationError => "参数验证失败",
            Self::MissingRequiredParameter => "缺少必需参数",
            Self::InvalidParameterFormat => "参数格式错误",
            Self::ParameterOutOfRange => "参数值超出范围",

            // 业务逻辑错误码描述
            Self::BusinessError => "业务逻辑错误",
            Self::OperationNotSupported => "操作不被支持",
            Self::ResourceConflict => "资源状态冲突",

            // 系统错误码描述
            Self::InternalError => "系统内部错误",
            Self::ConfigurationError => "配置错误",
            Self::ResourceExhausted => "资源耗尽",

            // 限流错误码描述
            Self::RateLimitExceeded => "请求频率限制",

            // 响应大小错误码描述
            Self::ResponseTooLarge => "响应体大小超过限制",

            // 缓存错误码描述
            Self::CacheMiss => "缓存未命中",
            Self::CacheServiceUnavailable => "缓存服务不可用",

            // 安全错误码描述
            Self::SecurityPolicyViolation => "安全策略违规",
            Self::AccessDenied => "访问被拒绝",

            Self::Unknown => "未知错误",
        }
    }

    /// 获取详细的错误说明
    pub fn detailed_description(&self) -> &'static str {
        match self {
            Self::Success => "请求已成功处理",

            Self::BadRequest => "请求参数格式错误或缺少必需参数，请检查请求内容",
            Self::Unauthorized => "身份验证失败，请检查访问令牌是否有效",
            Self::Forbidden => "权限不足，无法访问请求的资源",
            Self::NotFound => "请求的资源不存在或已被删除",
            Self::MethodNotAllowed => "当前API不支持此HTTP方法",
            Self::Conflict => "请求与当前资源状态冲突，请检查资源状态",
            Self::TooManyRequests => "请求频率超过限制，请降低请求频率后重试",

            Self::InternalServerError => "服务器内部错误，请稍后重试或联系技术支持",
            Self::BadGateway => "网关服务器错误，请检查网络连接或稍后重试",
            Self::ServiceUnavailable => "服务暂时不可用，请稍后重试",
            Self::GatewayTimeout => "网关超时，请稍后重试",

            Self::AppNotInstalled => "应用未安装到当前企业，请在飞书管理后台安装应用",
            Self::AppTicketInvalid => "应用票据已失效，SDK会自动重新申请",
            Self::AppStatusException => "应用状态异常，请检查应用是否正常启用",
            Self::AppPermissionDenied => "应用缺少执行此操作的权限，请在开发者后台配置相应权限",
            Self::UserNotFound => "指定的用户不存在，请检查用户ID是否正确",
            Self::UserStatusException => "用户状态异常，可能已被禁用或删除",
            Self::DepartmentNotFound => "指定的部门不存在，请检查部门ID是否正确",
            Self::ChatNotFound => "指定的群组不存在或机器人未加入该群组",
            Self::ChatTypeNotSupported => "当前群组类型不支持此操作",
            Self::MessageNotFound => "指定的消息不存在或已被删除",
            Self::MessageTypeNotSupported => "不支持的消息类型",
            Self::FileNotFound => "指定的文件不存在或已被删除",
            Self::FileSizeExceeded => "文件大小超出限制，请压缩后重试",
            Self::FileTypeNotSupported => "不支持的文件类型",
            Self::CalendarNotFound => "指定的日历不存在",
            Self::EventNotFound => "指定的日程不存在或已被删除",
            Self::EventConflict => "日程时间冲突，请选择其他时间",
            Self::DocumentNotFound => "指定的文档不存在或已被删除",
            Self::DocumentPermissionDenied => "文档权限不足，请联系文档所有者授权",
            Self::DocumentLocked => "文档已被其他用户锁定，请稍后再试",
            Self::SheetNotFound => "指定的工作表不存在",
            Self::TableNotFound => "指定的表格不存在",
            Self::AccessTokenFormatInvalid => "访问令牌格式或内容无效，请重新获取或检查传参",
            Self::AccessTokenInvalid => "用户访问令牌无效或失效，需要重新获取用户授权",
            Self::AppAccessTokenInvalid => "应用访问令牌无效，请检查应用ID和密钥配置",
            Self::TenantAccessTokenInvalid => "租户访问令牌无效，请检查应用是否正确安装到企业",
            Self::SsoTokenInvalid => "SSO 访问令牌无效，请重新登录或检查 SSO 配置",
            Self::PermissionMissing => "缺少所需权限，请在开发者后台开通对应权限范围",
            Self::AccessTokenNoPermission => "访问令牌权限不足，请重新授权所需 scope",
            Self::AccessTokenExpiredV2 => "访问令牌已过期，需要重新获取",
            Self::UserSessionInvalid => "用户会话失效，请重新登录或刷新会话",
            Self::UserSessionNotFound => "未找到有效用户会话，请重新登录",
            Self::UserSessionTimeout => "用户会话已超时，请重新登录",
            Self::UserIdentityInvalid => "用户身份解析失败，请检查传入的身份标识",
            Self::UserTypeNotSupportedV2 => "当前用户类型不支持该操作",
            Self::UserIdentityMismatch => "用户身份不匹配，请检查 user_id/open_id 组合",
            Self::UserIdInvalid => "用户ID非法，请检查参数格式",
            Self::OpenIdInvalid => "OpenID 非法，请检查参数格式",
            Self::UnionIdInvalid => "UnionID 非法，请检查参数格式",

            Self::NetworkTimeout => "网络请求超时，请检查网络连接或增加超时时间",
            Self::NetworkConnectionFailed => "网络连接失败，请检查网络设置和服务器状态",
            Self::DnsResolutionFailed => "DNS解析失败，请检查域名设置或网络配置",
            Self::SslCertificateError => "SSL证书验证失败，请检查证书配置或系统时间",
            Self::ConnectionRefused => "连接被拒绝，请检查服务器状态和防火墙设置",

            Self::SerializationError => "JSON序列化或反序列化失败，请检查数据格式",
            Self::DataFormatError => "数据格式不正确，请检查输入数据结构",
            Self::EncodingError => "数据编码错误，请检查字符编码设置",

            Self::AuthenticationFailed => "身份验证失败，请检查凭据是否正确",
            Self::PermissionDenied => "权限不足，无法执行此操作",
            Self::TokenExpired => "访问令牌已过期，需要重新获取",
            Self::InvalidSignature => "签名验证失败，请检查签名算法和密钥",

            Self::ValidationError => "输入数据验证失败，请检查数据格式和内容",
            Self::MissingRequiredParameter => "缺少必需的参数，请检查请求参数完整性",
            Self::InvalidParameterFormat => "参数格式不正确，请按照要求格式提供参数",
            Self::ParameterOutOfRange => "参数值超出允许范围，请检查参数值是否有效",

            Self::BusinessError => "业务逻辑处理失败，请检查操作条件和业务规则",
            Self::OperationNotSupported => "当前操作不被支持，请检查API文档或使用替代方法",
            Self::ResourceConflict => "资源状态冲突，请检查资源当前状态或等待后重试",

            Self::InternalError => "系统内部错误，请联系技术支持",
            Self::ConfigurationError => "系统配置错误，请检查配置文件或环境变量",
            Self::ResourceExhausted => "系统资源耗尽，请稍后重试或增加资源配额",

            Self::RateLimitExceeded => "请求频率超过限制，请降低请求频率后重试",

            Self::ResponseTooLarge => "响应体大小超过限制，请减小请求范围或联系管理员调整限制",

            Self::CacheMiss => "缓存中未找到数据，将从数据源获取",
            Self::CacheServiceUnavailable => "缓存服务不可用，将直接访问数据源",

            Self::SecurityPolicyViolation => "操作违反安全策略，请检查操作权限和安全设置",
            Self::AccessDenied => "访问被拒绝，请检查访问权限和身份验证",

            Self::Unknown => "未知错误，请联系技术支持获取帮助",
        }
    }

    /// 判断是否为成功状态
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }

    /// 判断是否为客户端错误 (4xx)
    pub fn is_client_error(&self) -> bool {
        let code = *self as i32;
        (400..=499).contains(&code) && code != 429
    }

    /// 判断是否为服务器错误 (5xx)
    pub fn is_server_error(&self) -> bool {
        let code = *self as i32;
        (500..=599).contains(&code)
    }

    /// 判断是否为网络相关错误
    pub fn is_network_error(&self) -> bool {
        matches!(
            self,
            Self::NetworkTimeout
                | Self::NetworkConnectionFailed
                | Self::DnsResolutionFailed
                | Self::SslCertificateError
                | Self::ConnectionRefused
        )
    }

    /// 判断是否为认证相关错误
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            Self::Unauthorized
                | Self::AccessTokenFormatInvalid
                | Self::AccessTokenInvalid
                | Self::AppAccessTokenInvalid
                | Self::TenantAccessTokenInvalid
                | Self::SsoTokenInvalid
                | Self::AuthenticationFailed
                | Self::TokenExpired
                | Self::AccessTokenExpiredV2
                | Self::InvalidSignature
                | Self::UserSessionInvalid
                | Self::UserSessionNotFound
                | Self::UserSessionTimeout
                | Self::UserIdentityInvalid
                | Self::UserTypeNotSupportedV2
                | Self::UserIdentityMismatch
                | Self::UserIdInvalid
                | Self::OpenIdInvalid
                | Self::UnionIdInvalid
        )
    }

    /// 判断是否为权限相关错误
    pub fn is_permission_error(&self) -> bool {
        matches!(
            self,
            Self::Forbidden
                | Self::AppPermissionDenied
                | Self::DocumentPermissionDenied
                | Self::PermissionDenied
                | Self::PermissionMissing
                | Self::AccessTokenNoPermission
                | Self::AccessDenied
        )
    }

    /// 判断是否为可重试错误
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Success => false,
            Self::TooManyRequests => true,
            Self::InternalServerError => true,
            Self::BadGateway => true,
            Self::ServiceUnavailable => true,
            Self::GatewayTimeout => true,
            Self::NetworkTimeout => true,
            Self::NetworkConnectionFailed => true,
            Self::ConnectionRefused => true,
            Self::CacheServiceUnavailable => true,
            Self::RateLimitExceeded => true,
            _ => false,
        }
    }

    /// 获取建议的重试延迟时间（秒）
    pub fn suggested_retry_delay(&self) -> Option<u64> {
        match self {
            Self::TooManyRequests => Some(60),
            Self::InternalServerError => Some(5),
            Self::BadGateway => Some(3),
            Self::ServiceUnavailable => Some(10),
            Self::GatewayTimeout => Some(5),
            Self::NetworkTimeout => Some(3),
            Self::NetworkConnectionFailed => Some(5),
            Self::ConnectionRefused => Some(10),
            Self::RateLimitExceeded => Some(30),
            _ => None,
        }
    }

    /// 获取HTTP状态码
    pub fn http_status(&self) -> Option<u16> {
        match self {
            Self::ResponseTooLarge => Some(413),
            _ => {
                let code = *self as i32;
                if (100..=599).contains(&code) {
                    Some(code as u16)
                } else {
                    None
                }
            }
        }
    }

    /// 从HTTP状态码创建错误码
    pub fn from_http_status(status: u16) -> Self {
        Self::from_code(status as i32)
    }

    /// 按飞书通用错误码映射（仅飞书返回体的 code 字段，未知返回 None）
    pub fn from_feishu_code(code: i32) -> Option<Self> {
        match code {
            99991661 => Some(Self::AccessTokenFormatInvalid),
            99991663 => Some(Self::TenantAccessTokenInvalid),
            99991664 => Some(Self::AppAccessTokenInvalid),
            99991670 => Some(Self::SsoTokenInvalid),
            99991671 => Some(Self::AccessTokenInvalid),
            99991672 => Some(Self::PermissionMissing),
            99991676 => Some(Self::AccessTokenNoPermission),
            99991677 => Some(Self::AccessTokenExpiredV2),
            99991641 => Some(Self::UserSessionInvalid),
            99991642 => Some(Self::UserSessionNotFound),
            99991645 => Some(Self::UserSessionTimeout),
            99991669 => Some(Self::UserIdentityInvalid),
            99991674 => Some(Self::UserTypeNotSupportedV2),
            99991675 => Some(Self::UserIdentityMismatch),
            99992351 => Some(Self::UserIdInvalid),
            99992352 => Some(Self::OpenIdInvalid),
            99992353 => Some(Self::UnionIdInvalid),
            _ => None,
        }
    }

    // === 与thiserror CoreError配合的新方法 ===

    /// 根据错误类型自动选择合适的错误码
    ///
    /// 为CoreError枚举变体提供最佳的错误码匹配
    pub fn for_error_type(error_type: &str) -> Self {
        match error_type {
            // 网络相关错误
            "Network" | "network" => Self::NetworkConnectionFailed,

            // 认证相关错误
            "Authentication" | "authentication" => Self::AuthenticationFailed,
            "TokenExpired" | "token_expired" => Self::TokenExpired,

            // 权限相关错误
            "Permission" | "permission" => Self::PermissionDenied,
            "Forbidden" | "forbidden" => Self::Forbidden,

            // 验证相关错误
            "Validation" | "validation" => Self::ValidationError,
            "ApiError" | "api_error" => Self::BusinessError,

            // 配置相关错误
            "Configuration" | "configuration" => Self::ConfigurationError,

            // 序列化相关错误
            "Serialization" | "serialization" => Self::SerializationError,

            // 系统相关错误
            "Internal" | "internal" => Self::InternalError,
            "Unknown" | "unknown" => Self::Unknown,

            // HTTP状态码映射
            "BadRequest" | "bad_request" => Self::BadRequest,
            "Unauthorized" | "unauthorized" => Self::Unauthorized,
            "NotFound" | "not_found" => Self::NotFound,
            "Conflict" | "conflict" => Self::Conflict,
            "TooManyRequests" | "too_many_requests" => Self::TooManyRequests,
            "InternalServerError" | "internal_server_error" => Self::InternalServerError,
            "ServiceUnavailable" | "service_unavailable" => Self::ServiceUnavailable,

            // 默认为未知错误
            _ => Self::Unknown,
        }
    }

    /// 创建网络相关错误码
    pub fn network() -> Self {
        Self::NetworkConnectionFailed
    }

    /// 创建认证相关错误码
    pub fn authentication() -> Self {
        Self::AuthenticationFailed
    }

    /// 创建权限相关错误码
    pub fn permission() -> Self {
        Self::PermissionDenied
    }

    /// 创建验证相关错误码
    pub fn validation() -> Self {
        Self::ValidationError
    }

    /// 创建配置相关错误码
    pub fn configuration() -> Self {
        Self::ConfigurationError
    }

    /// 创建系统内部错误码
    pub fn internal() -> Self {
        Self::InternalError
    }

    /// 创建序列化错误码
    pub fn serialization() -> Self {
        Self::SerializationError
    }

    /// 创建未知错误码
    pub fn unknown() -> Self {
        Self::Unknown
    }

    /// 从字符串描述智能推断错误码
    ///
    /// 根据错误消息的关键词自动推断最合适的错误码
    pub fn from_message(message: &str) -> Self {
        let msg_lower = message.to_lowercase();

        // 网络相关关键词
        if msg_lower.contains("network")
            || msg_lower.contains("connection")
            || msg_lower.contains("timeout")
            || msg_lower.contains("dns")
        {
            return Self::NetworkConnectionFailed;
        }

        // 认证相关关键词
        if msg_lower.contains("auth")
            || msg_lower.contains("token")
            || msg_lower.contains("unauthorized")
        {
            return Self::AuthenticationFailed;
        }

        // 权限相关关键词
        if msg_lower.contains("permission")
            || msg_lower.contains("forbidden")
            || msg_lower.contains("access denied")
        {
            return Self::PermissionDenied;
        }

        // 验证相关关键词
        if msg_lower.contains("invalid")
            || msg_lower.contains("validation")
            || msg_lower.contains("parameter")
        {
            return Self::ValidationError;
        }

        // 配置相关关键词
        if msg_lower.contains("config") || msg_lower.contains("setting") {
            return Self::ConfigurationError;
        }

        // 序列化相关关键词
        if msg_lower.contains("json")
            || msg_lower.contains("serialize")
            || msg_lower.contains("parse")
        {
            return Self::SerializationError;
        }

        // 资源相关关键词
        if msg_lower.contains("not found") || msg_lower.contains("missing") {
            return Self::NotFound;
        }

        // 服务相关关键词
        if msg_lower.contains("unavailable") || msg_lower.contains("service") {
            return Self::ServiceUnavailable;
        }

        Self::Unknown
    }

    /// 判断是否应该触发重试策略
    ///
    /// 提供更细粒度的重试判断逻辑
    pub fn should_retry(&self, attempt: u32) -> bool {
        if !self.is_retryable() {
            return false;
        }

        match self {
            // 限制重试次数的错误类型
            Self::TooManyRequests | Self::RateLimitExceeded => attempt < 5,
            Self::NetworkTimeout | Self::NetworkConnectionFailed => attempt < 3,
            Self::InternalServerError | Self::BadGateway | Self::GatewayTimeout => attempt < 2,
            _ => attempt < 3,
        }
    }

    /// 获取重试的指数退避延迟时间（毫秒）
    ///
    /// 提供智能的退避策略
    pub fn exponential_backoff_delay(&self, attempt: u32) -> Option<u64> {
        if !self.should_retry(attempt) {
            return None;
        }

        let base_delay = self.suggested_retry_delay().unwrap_or(1);
        let max_delay = match self {
            Self::TooManyRequests => 300,   // 5分钟
            Self::RateLimitExceeded => 180, // 3分钟
            _ => 60,                        // 1分钟
        };

        // 指数退避公式: base_delay * 2^(attempt-1)
        let delay = base_delay * 2u64.pow(attempt.saturating_sub(1));
        Some(delay.min(max_delay))
    }

    /// 获取错误的严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::Success => ErrorSeverity::Info,

            // 客户端错误通常是警告级别
            Self::BadRequest
            | Self::Forbidden
            | Self::NotFound
            | Self::MethodNotAllowed
            | Self::Conflict
            | Self::ValidationError
            | Self::MissingRequiredParameter
            | Self::InvalidParameterFormat
            | Self::ParameterOutOfRange
            | Self::ResponseTooLarge => ErrorSeverity::Warning,

            // 认证和权限错误是错误级别
            Self::Unauthorized
            | Self::AccessTokenInvalid
            | Self::AppAccessTokenInvalid
            | Self::TenantAccessTokenInvalid
            | Self::AuthenticationFailed
            | Self::TokenExpired
            | Self::PermissionDenied
            | Self::AccessDenied => ErrorSeverity::Error,

            // 服务器和网络错误是严重错误
            Self::InternalServerError
            | Self::BadGateway
            | Self::ServiceUnavailable
            | Self::GatewayTimeout
            | Self::NetworkTimeout
            | Self::NetworkConnectionFailed
            | Self::DnsResolutionFailed
            | Self::SslCertificateError
            | Self::ConnectionRefused
            | Self::InternalError
            | Self::ConfigurationError
            | Self::ResourceExhausted => ErrorSeverity::Critical,

            // 其他错误根据具体情况
            _ => ErrorSeverity::Error,
        }
    }

    /// 获取用户友好的恢复建议
    pub fn recovery_suggestion(&self) -> &'static str {
        match self {
            Self::BadRequest => "请检查请求参数是否正确",
            Self::Unauthorized => "请重新进行身份验证",
            Self::Forbidden => "请联系管理员获取相应权限",
            Self::NotFound => "请确认资源是否存在",
            Self::TooManyRequests => "请降低请求频率后重试",
            Self::NetworkConnectionFailed => "请检查网络连接",
            Self::TokenExpired => "请重新获取访问令牌",
            Self::ServiceUnavailable => "服务暂时不可用，请稍后重试",
            Self::InternalServerError => "系统内部错误，请联系技术支持",
            Self::ValidationError => "请检查输入参数格式",
            Self::ConfigurationError => "请检查系统配置",
            Self::ResponseTooLarge => "请减小请求范围或联系管理员调整响应大小限制",
            Self::Unknown => "发生未知错误，请联系技术支持",
            _ => "请稍后重试，如问题持续请联系技术支持",
        }
    }

    /// 判断是否为用户操作导致的错误
    pub fn is_user_error(&self) -> bool {
        matches!(
            self,
            Self::BadRequest
                | Self::Unauthorized
                | Self::Forbidden
                | Self::ValidationError
                | Self::MissingRequiredParameter
                | Self::InvalidParameterFormat
                | Self::ParameterOutOfRange
                | Self::AccessTokenInvalid
                | Self::TokenExpired
        )
    }

    /// 判断是否为系统问题导致的错误
    pub fn is_system_error(&self) -> bool {
        matches!(
            self,
            Self::InternalServerError
                | Self::BadGateway
                | Self::ServiceUnavailable
                | Self::GatewayTimeout
                | Self::NetworkTimeout
                | Self::NetworkConnectionFailed
                | Self::DnsResolutionFailed
                | Self::SslCertificateError
                | Self::ConnectionRefused
                | Self::InternalError
                | Self::ConfigurationError
                | Self::ResourceExhausted
        )
    }

    /// 获取日志级别建议
    pub fn log_level(&self) -> &'static str {
        match self.severity() {
            ErrorSeverity::Info => "info",
            ErrorSeverity::Warning => "warn",
            ErrorSeverity::Error => "error",
            ErrorSeverity::Critical => "error",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.description(), self.as_code())
    }
}

/// 错误码分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// 成功
    Success,
    /// 认证错误
    Authentication,
    /// 权限错误
    Permission,
    /// 参数错误
    Parameter,
    /// 资源错误
    Resource,
    /// 服务器错误
    Server,
    /// 网络错误
    Network,
    /// 业务错误
    Business,
    /// 系统错误
    System,
    /// 限流错误
    RateLimit,
    /// 其他错误
    Other,
}

impl ErrorCode {
    /// 获取错误码分类
    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::Success => ErrorCategory::Success,

            // 认证相关
            Self::Unauthorized
            | Self::AppTicketInvalid
            | Self::AccessTokenFormatInvalid
            | Self::AccessTokenInvalid
            | Self::AppAccessTokenInvalid
            | Self::TenantAccessTokenInvalid
            | Self::SsoTokenInvalid
            | Self::AuthenticationFailed
            | Self::TokenExpired
            | Self::AccessTokenExpiredV2
            | Self::InvalidSignature
            | Self::UserSessionInvalid
            | Self::UserSessionNotFound
            | Self::UserSessionTimeout
            | Self::UserIdentityInvalid
            | Self::UserTypeNotSupportedV2
            | Self::UserIdentityMismatch
            | Self::UserIdInvalid
            | Self::OpenIdInvalid
            | Self::UnionIdInvalid => ErrorCategory::Authentication,

            // 权限相关
            Self::Forbidden
            | Self::AppPermissionDenied
            | Self::DocumentPermissionDenied
            | Self::PermissionDenied
            | Self::PermissionMissing
            | Self::AccessTokenNoPermission
            | Self::AccessDenied
            | Self::SecurityPolicyViolation => ErrorCategory::Permission,

            // 参数相关
            Self::BadRequest
            | Self::ValidationError
            | Self::MissingRequiredParameter
            | Self::InvalidParameterFormat
            | Self::ParameterOutOfRange
            | Self::ChatTypeNotSupported
            | Self::MessageTypeNotSupported
            | Self::FileTypeNotSupported => ErrorCategory::Parameter,

            // 资源相关
            Self::NotFound
            | Self::AppNotInstalled
            | Self::UserNotFound
            | Self::UserStatusException
            | Self::DepartmentNotFound
            | Self::ChatNotFound
            | Self::MessageNotFound
            | Self::FileNotFound
            | Self::FileSizeExceeded
            | Self::CalendarNotFound
            | Self::EventNotFound
            | Self::DocumentNotFound
            | Self::DocumentLocked
            | Self::SheetNotFound
            | Self::TableNotFound => ErrorCategory::Resource,

            // 服务器相关
            Self::InternalServerError
            | Self::BadGateway
            | Self::ServiceUnavailable
            | Self::GatewayTimeout
            | Self::AppStatusException
            | Self::InternalError
            | Self::CacheServiceUnavailable => ErrorCategory::Server,

            // 网络相关
            Self::NetworkTimeout
            | Self::NetworkConnectionFailed
            | Self::DnsResolutionFailed
            | Self::SslCertificateError
            | Self::ConnectionRefused => ErrorCategory::Network,

            // 业务相关
            Self::Conflict
            | Self::EventConflict
            | Self::BusinessError
            | Self::OperationNotSupported
            | Self::ResourceConflict => ErrorCategory::Business,

            // 系统相关
            Self::MethodNotAllowed
            | Self::SerializationError
            | Self::DataFormatError
            | Self::EncodingError
            | Self::ConfigurationError
            | Self::ResourceExhausted
            | Self::ResponseTooLarge => ErrorCategory::System,

            // 限流相关
            Self::TooManyRequests | Self::RateLimitExceeded => ErrorCategory::RateLimit,

            // 其他
            _ => ErrorCategory::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_conversion() {
        assert_eq!(ErrorCode::from_code(0), ErrorCode::Success);
        assert_eq!(ErrorCode::from_code(404), ErrorCode::NotFound);
        assert_eq!(ErrorCode::from_code(999999), ErrorCode::Unknown);
    }

    #[test]
    fn test_error_code_properties() {
        let not_found = ErrorCode::NotFound;
        assert_eq!(not_found.as_code(), 404);
        assert_eq!(not_found.description(), "资源不存在");
        assert!(not_found.is_client_error());
        assert!(!not_found.is_server_error());
        assert!(!not_found.is_retryable());

        let timeout = ErrorCode::NetworkTimeout;
        assert!(timeout.is_network_error());
        assert!(timeout.is_retryable());
        assert_eq!(timeout.suggested_retry_delay(), Some(3));
    }

    #[test]
    fn test_http_status_conversion() {
        assert_eq!(ErrorCode::from_http_status(200), ErrorCode::Success);
        assert_eq!(ErrorCode::from_http_status(404), ErrorCode::NotFound);
        assert_eq!(
            ErrorCode::from_http_status(500),
            ErrorCode::InternalServerError
        );
    }

    #[test]
    fn test_error_categories() {
        let auth_error = ErrorCode::Unauthorized;
        assert_eq!(auth_error.category(), ErrorCategory::Authentication);

        let perm_error = ErrorCode::Forbidden;
        assert_eq!(perm_error.category(), ErrorCategory::Permission);

        let net_error = ErrorCode::NetworkTimeout;
        assert_eq!(net_error.category(), ErrorCategory::Network);
    }

    #[test]
    fn test_error_code_display() {
        let error = ErrorCode::AccessTokenInvalid;
        let display = format!("{error}");
        assert!(display.contains("访问令牌无效"));
        assert!(display.contains("99991671"));
    }

    #[test]
    fn test_new_error_code_methods() {
        // 测试便利创建方法
        assert_eq!(ErrorCode::network(), ErrorCode::NetworkConnectionFailed);
        assert_eq!(ErrorCode::authentication(), ErrorCode::AuthenticationFailed);
        assert_eq!(ErrorCode::permission(), ErrorCode::PermissionDenied);
        assert_eq!(ErrorCode::validation(), ErrorCode::ValidationError);
        assert_eq!(ErrorCode::configuration(), ErrorCode::ConfigurationError);
        assert_eq!(ErrorCode::internal(), ErrorCode::InternalError);
        assert_eq!(ErrorCode::serialization(), ErrorCode::SerializationError);
        assert_eq!(ErrorCode::unknown(), ErrorCode::Unknown);
    }

    #[test]
    fn test_for_error_type() {
        assert_eq!(
            ErrorCode::for_error_type("Network"),
            ErrorCode::NetworkConnectionFailed
        );
        assert_eq!(
            ErrorCode::for_error_type("authentication"),
            ErrorCode::AuthenticationFailed
        );
        assert_eq!(
            ErrorCode::for_error_type("Validation"),
            ErrorCode::ValidationError
        );
        assert_eq!(
            ErrorCode::for_error_type("unknown_type"),
            ErrorCode::Unknown
        );
    }

    #[test]
    fn test_from_message() {
        assert_eq!(
            ErrorCode::from_message("Network connection failed"),
            ErrorCode::NetworkConnectionFailed
        );
        assert_eq!(
            ErrorCode::from_message("Authentication token invalid"),
            ErrorCode::AuthenticationFailed
        );
        assert_eq!(
            ErrorCode::from_message("Permission denied"),
            ErrorCode::PermissionDenied
        );
        assert_eq!(
            ErrorCode::from_message("Invalid parameter"),
            ErrorCode::ValidationError
        );
        assert_eq!(
            ErrorCode::from_message("JSON parse error"),
            ErrorCode::SerializationError
        );
        assert_eq!(
            ErrorCode::from_message("Unknown error occurred"),
            ErrorCode::Unknown
        );
    }

    #[test]
    fn test_retry_logic() {
        let retryable_error = ErrorCode::NetworkTimeout;
        assert!(retryable_error.should_retry(0));
        assert!(retryable_error.should_retry(1));
        assert!(!retryable_error.should_retry(3)); // 超过最大重试次数

        let non_retryable_error = ErrorCode::BadRequest;
        assert!(!non_retryable_error.should_retry(0));

        // 测试指数退避
        let delay = retryable_error.exponential_backoff_delay(2);
        assert!(delay.is_some());
        assert!(delay.unwrap() > 1); // 第二次重试应该有更长的延迟
    }

    #[test]
    fn test_error_severity() {
        assert_eq!(ErrorCode::Success.severity(), ErrorSeverity::Info);
        assert_eq!(ErrorCode::BadRequest.severity(), ErrorSeverity::Warning);
        assert_eq!(ErrorCode::Unauthorized.severity(), ErrorSeverity::Error);
        assert_eq!(
            ErrorCode::InternalServerError.severity(),
            ErrorSeverity::Critical
        );
    }

    #[test]
    fn test_recovery_suggestions() {
        let suggestion = ErrorCode::NetworkConnectionFailed.recovery_suggestion();
        assert!(suggestion.contains("网络"));

        let suggestion = ErrorCode::TokenExpired.recovery_suggestion();
        assert!(suggestion.contains("令牌"));

        let suggestion = ErrorCode::ValidationError.recovery_suggestion();
        assert!(suggestion.contains("参数"));
    }

    #[test]
    fn test_user_vs_system_errors() {
        // 用户错误
        assert!(ErrorCode::BadRequest.is_user_error());
        assert!(ErrorCode::Unauthorized.is_user_error());
        assert!(ErrorCode::ValidationError.is_user_error());

        // 系统错误
        assert!(ErrorCode::InternalServerError.is_system_error());
        assert!(ErrorCode::NetworkTimeout.is_system_error());
        assert!(ErrorCode::ServiceUnavailable.is_system_error());

        // 交叉验证
        assert!(!ErrorCode::BadRequest.is_system_error());
        assert!(!ErrorCode::InternalServerError.is_user_error());
    }

    #[test]
    fn test_log_levels() {
        assert_eq!(ErrorCode::Success.log_level(), "info");
        assert_eq!(ErrorCode::BadRequest.log_level(), "warn");
        assert_eq!(ErrorCode::Unauthorized.log_level(), "error");
        assert_eq!(ErrorCode::InternalServerError.log_level(), "error");
    }
}

#[cfg(test)]
mod severity_tests {
    use super::*;

    #[test]
    fn test_error_severity_basic() {
        assert_eq!(ErrorSeverity::Info.as_level(), 0);
        assert_eq!(ErrorSeverity::Warning.as_level(), 1);
        assert_eq!(ErrorSeverity::Error.as_level(), 2);
        assert_eq!(ErrorSeverity::Critical.as_level(), 3);
    }

    #[test]
    fn test_error_severity_ordering() {
        assert!(ErrorSeverity::Info < ErrorSeverity::Warning);
        assert!(ErrorSeverity::Warning < ErrorSeverity::Error);
        assert!(ErrorSeverity::Error < ErrorSeverity::Critical);
    }

    #[test]
    fn test_error_severity_from_level() {
        assert_eq!(ErrorSeverity::from_level(0), ErrorSeverity::Info);
        assert_eq!(ErrorSeverity::from_level(1), ErrorSeverity::Warning);
        assert_eq!(ErrorSeverity::from_level(2), ErrorSeverity::Error);
        assert_eq!(ErrorSeverity::from_level(3), ErrorSeverity::Critical);
        assert_eq!(ErrorSeverity::from_level(99), ErrorSeverity::Error); // 无效值默认为Error
    }

    #[test]
    fn test_error_severity_properties() {
        assert!(!ErrorSeverity::Info.requires_immediate_action());
        assert!(!ErrorSeverity::Warning.requires_immediate_action());
        assert!(ErrorSeverity::Error.requires_immediate_action());
        assert!(ErrorSeverity::Critical.requires_immediate_action());

        assert!(!ErrorSeverity::Info.requires_user_intervention());
        assert!(!ErrorSeverity::Warning.requires_user_intervention());
        assert!(ErrorSeverity::Error.requires_user_intervention());
        assert!(ErrorSeverity::Critical.requires_user_intervention());

        assert!(ErrorSeverity::Info.is_auto_recoverable());
        assert!(ErrorSeverity::Warning.is_auto_recoverable());
        assert!(!ErrorSeverity::Error.is_auto_recoverable());
        assert!(!ErrorSeverity::Critical.is_auto_recoverable());
    }

    #[test]
    fn test_error_severity_display() {
        assert_eq!(format!("{}", ErrorSeverity::Info), "信息");
        assert_eq!(format!("{}", ErrorSeverity::Warning), "警告");
        assert_eq!(format!("{}", ErrorSeverity::Error), "错误");
        assert_eq!(format!("{}", ErrorSeverity::Critical), "严重错误");
    }
}
