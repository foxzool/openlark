//! CoreError：基于 thiserror 的企业级错误处理
//!
//! 目标：展示单一映射来源（ErrorCode），简化字段、统一严重度/可重试策略，
//! 并保留可序列化的 ErrorRecord 供观测与分析使用。

use std::{borrow::Cow, time::Duration};

use super::{
    codes::ErrorCode,
    context::ErrorContext,
    traits::{ErrorSeverity, ErrorTrait, ErrorType},
};
use serde::Serialize;
use thiserror::Error;

// 重试与恢复策略

/// 重试策略（供 CoreError 使用）
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// 最大重试次数
    pub max_retries: u32,
    /// 寏次重试基础延迟
    pub base_delay: Duration,
    /// 退避因子
    pub backoff_factor: f64,
    /// 最大延迟
    pub max_delay: Option<Duration>,
}

impl RetryPolicy {
    /// 创建不重试策略
    pub fn no_retry() -> Self {
        Self {
            max_retries: 0,
            base_delay: Duration::from_secs(1),
            backoff_factor: 1.0,
            max_delay: None,
        }
    }

    /// 创建固定延迟重试策略
    pub fn fixed(max_retries: u32, delay: Duration) -> Self {
        Self {
            max_retries,
            base_delay: delay,
            backoff_factor: 1.0,
            max_delay: Some(delay),
        }
    }

    /// 创建指数退避重试策略
    pub fn exponential(max_retries: u32, base_delay: Duration) -> Self {
        Self {
            max_retries,
            base_delay,
            backoff_factor: 2.0,
            max_delay: Some(Duration::from_secs(300)), // 5分钟最大延迟
        }
    }

    /// 是否可重试
    pub fn is_retryable(&self) -> bool {
        self.max_retries > 0
    }

    /// 计算重试延迟
    pub fn retry_delay(&self, attempt: u32) -> Option<Duration> {
        if attempt >= self.max_retries {
            return None;
        }

        let delay = if self.backoff_factor == 1.0 {
            self.base_delay
        } else {
            let seconds = self.base_delay.as_secs_f64() * self.backoff_factor.powi(attempt as i32);
            Duration::from_secs_f64(seconds)
        };

        Some(self.max_delay.map_or(delay, |max| delay.min(max)))
    }

    /// 直接获取延迟（超过最大重试返回0）
    pub fn delay(&self, attempt: u32) -> Duration {
        self.retry_delay(attempt).unwrap_or(Duration::ZERO)
    }

    /// 是否使用指数退避
    pub fn use_exponential_backoff(&self) -> bool {
        self.backoff_factor > 1.0
    }

    /// 最大重试次数（保持旧接口）
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::exponential(3, Duration::from_secs(1))
    }
}

/// 恢复策略
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// 重试并使用指数退避
    RetryWithBackoff,
    /// 验证后重试
    ValidateAndRetry,
    /// 重新认证
    Reauthenticate,
    /// 请求权限
    RequestPermission,
    /// 手动干预
    ManualIntervention,
    /// 延迟重试
    RetryWithDelay,
}

type AnyError = Box<dyn std::error::Error + Send + Sync>;

/// 枚举化的构建器目标类型
#[derive(Debug, Clone, Copy)]
pub enum BuilderKind {
    /// 网络错误
    Network,
    /// 认证错误
    Authentication,
    /// API 错误
    Api,
    /// 验证错误
    Validation,
    /// 配置错误
    Configuration,
    /// 序列化错误
    Serialization,
    /// 业务错误
    Business,
    /// 超时错误
    Timeout,
    /// 限流错误
    RateLimit,
    /// 服务不可用错误
    ServiceUnavailable,
    /// 内部错误
    Internal,
}

/// 统一的错误构建器，避免直接依赖枚举内部字段
#[derive(Debug)]
pub struct ErrorBuilder {
    /// 构建器目标类型
    kind: BuilderKind,
    /// 错误消息
    message: Option<String>,
    /// 错误码
    code: Option<ErrorCode>,
    /// HTTP 状态码
    status: Option<u16>,
    /// API 端点
    endpoint: Option<String>,
    /// 验证字段名
    field: Option<String>,
    /// 源错误
    source: Option<AnyError>,
    /// 重试策略
    policy: Option<RetryPolicy>,
    /// 错误上下文
    ctx: ErrorContext,
    /// 持续时间
    duration: Option<Duration>,
    /// 操作名
    operation: Option<String>,
    /// 限流限制
    limit: Option<u32>,
    /// 限流窗口
    window: Option<Duration>,
    /// 重置时间
    reset_after: Option<Duration>,
    /// 服务名
    service: Option<String>,
    /// 重试等待时间
    retry_after: Option<Duration>,
}

impl ErrorBuilder {
    /// 创建指定类型的错误构建器
    pub fn new(kind: BuilderKind) -> Self {
        Self {
            kind,
            message: None,
            code: None,
            status: None,
            endpoint: None,
            field: None,
            source: None,
            policy: None,
            ctx: ErrorContext::new(),
            duration: None,
            operation: None,
            limit: None,
            window: None,
            reset_after: None,
            service: None,
            retry_after: None,
        }
    }

    /// 设置错误消息
    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }

    /// 设置错误码
    pub fn code(mut self, code: ErrorCode) -> Self {
        self.code = Some(code);
        self
    }

    /// 设置 HTTP 状态码
    pub fn status(mut self, status: u16) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置 API 端点
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    /// 设置验证字段名
    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    /// 设置源错误
    pub fn source<E: std::error::Error + Send + Sync + 'static>(mut self, err: E) -> Self {
        self.source = Some(Box::new(err));
        self
    }

    /// 设置重试策略
    pub fn policy(mut self, policy: RetryPolicy) -> Self {
        self.policy = Some(policy);
        self
    }

    /// 设置请求 ID
    pub fn request_id(mut self, req: impl Into<String>) -> Self {
        self.ctx.set_request_id(req);
        self
    }

    /// 设置操作名称
    pub fn operation(mut self, op: impl Into<String>) -> Self {
        let op = op.into();
        self.operation = Some(op.clone());
        self.ctx.set_operation(op);
        self
    }

    /// 设置组件名称
    pub fn component(mut self, comp: impl Into<String>) -> Self {
        self.ctx.set_component(comp);
        self
    }

    /// 设置用户友好的错误消息
    pub fn user_message(mut self, msg: impl Into<String>) -> Self {
        self.ctx.set_user_message(msg);
        self
    }

    /// 添加上下文键值对
    pub fn context(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.ctx.add_context(key, val);
        self
    }

    /// 设置超时时长
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// 设置限流次数
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// 设置限流时间窗口
    pub fn window(mut self, window: Duration) -> Self {
        self.window = Some(window);
        self
    }

    /// 设置限流重置时间
    pub fn reset_after(mut self, reset: Duration) -> Self {
        self.reset_after = Some(reset);
        self
    }

    /// 设置服务名
    pub fn service(mut self, service: impl Into<String>) -> Self {
        self.service = Some(service.into());
        self
    }

    /// 设置重试等待时间
    pub fn retry_after(mut self, retry_after: Duration) -> Self {
        self.retry_after = Some(retry_after);
        self
    }

    /// 构建 CoreError 实例
    pub fn build(self) -> CoreError {
        let msg = self.message.unwrap_or_else(|| "unknown error".to_string());
        match self.kind {
            BuilderKind::Network => CoreError::Network(Box::new(NetworkError {
                message: msg,
                source: self.source,
                policy: self.policy.unwrap_or_default(),
                ctx: Box::new(self.ctx),
            })),
            BuilderKind::Authentication => CoreError::Authentication {
                message: msg,
                code: self.code.unwrap_or(ErrorCode::AuthenticationFailed),
                ctx: Box::new(self.ctx),
            },
            BuilderKind::Api => {
                let status = self.status.unwrap_or(500);
                CoreError::Api(Box::new(ApiError {
                    status,
                    endpoint: self
                        .endpoint
                        .unwrap_or_else(|| "unknown".to_string())
                        .into(),
                    message: msg,
                    source: self.source,
                    code: self
                        .code
                        .unwrap_or_else(|| ErrorCode::from_http_status(status)),
                    ctx: Box::new(self.ctx),
                }))
            }
            BuilderKind::Validation => CoreError::Validation {
                field: self.field.unwrap_or_else(|| "field".to_string()).into(),
                message: msg,
                code: self.code.unwrap_or(ErrorCode::ValidationError),
                ctx: Box::new(self.ctx),
            },
            BuilderKind::Configuration => CoreError::Configuration {
                message: msg,
                code: self.code.unwrap_or(ErrorCode::ConfigurationError),
                ctx: Box::new(self.ctx),
            },
            BuilderKind::Serialization => CoreError::Serialization {
                message: msg,
                source: self.source,
                code: self.code.unwrap_or(ErrorCode::SerializationError),
                ctx: Box::new(self.ctx),
            },
            BuilderKind::Business => CoreError::Business {
                code: self.code.unwrap_or(ErrorCode::BusinessError),
                message: msg,
                ctx: Box::new(self.ctx),
            },
            BuilderKind::Timeout => CoreError::Timeout {
                duration: self.duration.unwrap_or_default(),
                operation: self.operation,
                ctx: Box::new(self.ctx),
            },
            BuilderKind::RateLimit => CoreError::RateLimit {
                limit: self.limit.unwrap_or(0),
                window: self.window.unwrap_or(Duration::from_secs(1)),
                reset_after: self.reset_after,
                code: self.code.unwrap_or(ErrorCode::RateLimitExceeded),
                ctx: Box::new(self.ctx),
            },
            BuilderKind::ServiceUnavailable => CoreError::ServiceUnavailable {
                service: self.service.unwrap_or_else(|| "service".to_string()).into(),
                retry_after: self.retry_after,
                code: self.code.unwrap_or(ErrorCode::ServiceUnavailable),
                ctx: Box::new(self.ctx),
            },
            BuilderKind::Internal => CoreError::Internal {
                code: self.code.unwrap_or(ErrorCode::InternalError),
                message: msg,
                source: self.source,
                ctx: Box::new(self.ctx),
            },
        }
    }
}

/// 轻量版核心错误
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CoreError {
    /// 网络错误
    #[error("网络错误: {0}")]
    Network(Box<NetworkError>),

    /// 认证错误
    #[error("认证失败: {message}")]
    Authentication {
        /// 错误消息
        message: String,
        /// 错误码
        code: ErrorCode,
        /// 错误上下文
        ctx: Box<ErrorContext>,
    },

    /// API 错误
    #[error("API错误 {0}")]
    Api(Box<ApiError>),

    /// 验证错误
    #[error("验证错误 {field}: {message}")]
    Validation {
        /// 字段名
        field: Cow<'static, str>,
        /// 错误消息
        message: String,
        /// 错误码
        code: ErrorCode,
        /// 错误上下文
        ctx: Box<ErrorContext>,
    },

    /// 配置错误
    #[error("配置错误: {message}")]
    Configuration {
        /// 错误消息
        message: String,
        /// 错误码
        code: ErrorCode,
        /// 错误上下文
        ctx: Box<ErrorContext>,
    },

    /// 序列化错误
    #[error("序列化错误: {message}")]
    Serialization {
        /// 错误消息
        message: String,
        /// 源错误
        #[source]
        source: Option<AnyError>,
        /// 错误码
        code: ErrorCode,
        /// 错误上下文
        ctx: Box<ErrorContext>,
    },

    /// 业务错误
    #[error("业务错误 {code:?}: {message}")]
    Business {
        /// 错误码
        code: ErrorCode,
        /// 错误消息
        message: String,
        /// 错误上下文
        ctx: Box<ErrorContext>,
    },

    /// 超时错误
    #[error("超时 {operation:?} after {duration:?}")]
    Timeout {
        /// 超时时长
        duration: Duration,
        /// 操作名
        operation: Option<String>,
        /// 错误上下文
        ctx: Box<ErrorContext>,
    },

    /// 限流错误
    #[error("限流: {limit} 次/{window:?}")]
    RateLimit {
        /// 限制次数
        limit: u32,
        /// 时间窗口
        window: Duration,
        /// 重置时间
        reset_after: Option<Duration>,
        /// 错误码
        code: ErrorCode,
        /// 错误上下文
        ctx: Box<ErrorContext>,
    },

    /// 服务不可用错误
    #[error("服务不可用: {service}")]
    ServiceUnavailable {
        /// 服务名
        service: Cow<'static, str>,
        /// 重试等待时间
        retry_after: Option<Duration>,
        /// 错误码
        code: ErrorCode,
        /// 错误上下文
        ctx: Box<ErrorContext>,
    },

    /// 响应体大小超过限制
    #[error("响应体过大: {actual} 字节超过限制 {limit} 字节")]
    ResponseTooLarge {
        /// 配置的大小限制
        limit: u64,
        /// 实际大小（已知时），否则为 0
        actual: u64,
        /// 错误上下文
        ctx: Box<ErrorContext>,
    },

    /// 内部错误
    #[error("内部错误 {code:?}: {message}")]
    Internal {
        /// 错误码
        code: ErrorCode,
        /// 错误消息
        message: String,
        /// 源错误
        #[source]
        source: Option<AnyError>,
        /// 错误上下文
        ctx: Box<ErrorContext>,
    },
}

/// 网络错误
#[derive(Debug)]
pub struct NetworkError {
    /// 错误消息
    pub message: String,
    /// 源错误
    pub source: Option<AnyError>,
    /// 重试策略
    pub policy: RetryPolicy,
    /// 错误上下文
    pub ctx: Box<ErrorContext>,
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// API 错误
#[derive(Debug)]
pub struct ApiError {
    /// HTTP 状态码
    pub status: u16,
    /// API 端点
    pub endpoint: Cow<'static, str>,
    /// 错误消息
    pub message: String,
    /// 源错误
    pub source: Option<AnyError>,
    /// 错误码
    pub code: ErrorCode,
    /// 错误上下文
    pub ctx: Box<ErrorContext>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}: {}", self.status, self.endpoint, self.message)
    }
}

impl Clone for CoreError {
    fn clone(&self) -> Self {
        match self {
            Self::Network(net) => Self::Network(Box::new(NetworkError {
                message: net.message.clone(),
                source: None, // 源错误不可克隆，丢弃以便重试记录
                policy: net.policy.clone(),
                ctx: net.ctx.clone(),
            })),
            Self::Authentication { message, code, ctx } => Self::Authentication {
                message: message.clone(),
                code: *code,
                ctx: ctx.clone(),
            },
            Self::Api(api) => Self::Api(Box::new(ApiError {
                status: api.status,
                endpoint: api.endpoint.clone(),
                message: api.message.clone(),
                source: None,
                code: api.code,
                ctx: api.ctx.clone(),
            })),
            Self::Validation {
                field,
                message,
                code,
                ctx,
            } => Self::Validation {
                field: field.clone(),
                message: message.clone(),
                code: *code,
                ctx: ctx.clone(),
            },
            Self::Configuration { message, code, ctx } => Self::Configuration {
                message: message.clone(),
                code: *code,
                ctx: ctx.clone(),
            },
            Self::Serialization {
                message, code, ctx, ..
            } => Self::Serialization {
                message: message.clone(),
                source: None,
                code: *code,
                ctx: ctx.clone(),
            },
            Self::Business { code, message, ctx } => Self::Business {
                code: *code,
                message: message.clone(),
                ctx: ctx.clone(),
            },
            Self::Timeout {
                duration,
                operation,
                ctx,
            } => Self::Timeout {
                duration: *duration,
                operation: operation.clone(),
                ctx: ctx.clone(),
            },
            Self::RateLimit {
                limit,
                window,
                reset_after,
                code,
                ctx,
            } => Self::RateLimit {
                limit: *limit,
                window: *window,
                reset_after: *reset_after,
                code: *code,
                ctx: ctx.clone(),
            },
            Self::ServiceUnavailable {
                service,
                retry_after,
                code,
                ctx,
            } => Self::ServiceUnavailable {
                service: service.clone(),
                retry_after: *retry_after,
                code: *code,
                ctx: ctx.clone(),
            },
            Self::ResponseTooLarge { limit, actual, ctx } => Self::ResponseTooLarge {
                limit: *limit,
                actual: *actual,
                ctx: ctx.clone(),
            },
            Self::Internal {
                code, message, ctx, ..
            } => Self::Internal {
                code: *code,
                message: message.clone(),
                source: None,
                ctx: ctx.clone(),
            },
        }
    }
}

impl CoreError {
    /// 统一构建器入口
    pub fn builder(kind: BuilderKind) -> ErrorBuilder {
        ErrorBuilder::new(kind)
    }

    /// 网络错误构建器
    pub fn network_builder() -> ErrorBuilder {
        ErrorBuilder::new(BuilderKind::Network)
    }

    /// API 错误构建器
    pub fn api_builder() -> ErrorBuilder {
        ErrorBuilder::new(BuilderKind::Api)
    }

    /// 验证错误构建器
    pub fn validation_builder() -> ErrorBuilder {
        ErrorBuilder::new(BuilderKind::Validation)
    }

    /// 认证错误构建器
    pub fn authentication_builder() -> ErrorBuilder {
        ErrorBuilder::new(BuilderKind::Authentication)
    }

    /// 业务错误构建器
    pub fn business_builder() -> ErrorBuilder {
        ErrorBuilder::new(BuilderKind::Business)
    }

    /// 简单网络错误（无 source）
    pub fn network_msg(message: impl Into<String>) -> Self {
        network_error(message)
    }

    /// 简单认证错误
    pub fn authentication(message: impl Into<String>) -> Self {
        authentication_error(message)
    }

    /// 简单 API 错误（便于兼容旧 CoreError::api_error）
    pub fn api_error(
        status: i32,
        endpoint: impl Into<String>,
        message: impl Into<String>,
        request_id: Option<impl Into<String>>,
    ) -> Self {
        api_error(
            status as u16,
            endpoint,
            message,
            request_id.map(|id| id.into()),
        )
    }

    /// 仅带 message 的验证错误（默认字段 general）
    pub fn validation_msg(message: impl Into<String>) -> Self {
        validation_error("general", message)
    }

    /// 用户可读 message（兼容旧 API）
    pub fn message(&self) -> String {
        self.to_string()
    }

    /// 错误类型别名（兼容旧 kind）
    pub fn kind(&self) -> ErrorType {
        self.error_type()
    }

    /// 直接判断 API 错误（便捷别名）
    pub fn is_api_error(&self) -> bool {
        matches!(self, Self::Api(_))
    }

    // === 兼容旧 CoreError 的工厂 ===
    /// 验证错误
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        let mut ctx = ErrorContext::new();
        let field = field.into();
        ctx.add_context("field", field.clone());
        Self::Validation {
            field: field.into(),
            message: message.into(),
            code: ErrorCode::ValidationError,
            ctx: Box::new(ctx),
        }
    }

    /// API 数据错误
    pub fn api_data_error(message: impl Into<String>) -> Self {
        Self::Api(Box::new(ApiError {
            status: 500,
            endpoint: "data_error".into(),
            message: format!("no data: {}", message.into()),
            source: None,
            code: ErrorCode::InternalServerError,
            ctx: Box::new(ErrorContext::new()),
        }))
    }

    /// 获取错误码
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::Network(_) => ErrorCode::NetworkConnectionFailed,
            Self::Authentication { code, .. } => *code,
            Self::Api(api) => api.code,
            Self::Validation { code, .. } => *code,
            Self::Configuration { code, .. } => *code,
            Self::Serialization { code, .. } => *code,
            Self::Business { code, .. } => *code,
            Self::Timeout { .. } => ErrorCode::NetworkTimeout,
            Self::RateLimit { code, .. } => *code,
            Self::ServiceUnavailable { code, .. } => *code,
            Self::ResponseTooLarge { .. } => ErrorCode::ResponseTooLarge,
            Self::Internal { code, .. } => *code,
        }
    }

    /// 获取错误严重程度
    pub fn severity(&self) -> ErrorSeverity {
        self.code().severity()
    }

    /// 是否可重试
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Network(net) => net.policy.is_retryable(),
            Self::Api(api) => matches!(api.status, 429 | 500..=599),
            Self::Timeout { .. } => self.code().is_retryable(),
            Self::RateLimit { .. } => self.code().is_retryable(),
            Self::ServiceUnavailable { .. } => self.code().is_retryable(),
            Self::Internal { .. } => self.code().is_retryable(),
            _ => false,
        }
    }

    /// 获取重试延迟
    pub fn retry_delay(&self, attempt: u32) -> Option<Duration> {
        match self {
            Self::Network(net) => net.policy.retry_delay(attempt),
            Self::RateLimit { window, .. } => Some(*window),
            Self::ServiceUnavailable { retry_after, .. } => *retry_after,
            Self::Api(api) if matches!(api.status, 429 | 500..=599) => {
                Some(Duration::from_secs(1 << attempt.min(5)))
            }
            _ => None,
        }
    }

    /// 获取错误上下文
    pub fn ctx(&self) -> &ErrorContext {
        match self {
            Self::Network(net) => &net.ctx,
            Self::Api(api) => &api.ctx,
            Self::Authentication { ctx, .. }
            | Self::Validation { ctx, .. }
            | Self::Configuration { ctx, .. }
            | Self::Serialization { ctx, .. }
            | Self::Business { ctx, .. }
            | Self::Timeout { ctx, .. }
            | Self::RateLimit { ctx, .. }
            | Self::ServiceUnavailable { ctx, .. }
            | Self::ResponseTooLarge { ctx, .. }
            | Self::Internal { ctx, .. } => ctx,
        }
    }

    /// 修改错误上下文
    pub fn map_context(self, f: impl FnOnce(&mut ErrorContext)) -> Self {
        match self {
            Self::Network(mut net) => {
                f(net.ctx.as_mut());
                Self::Network(net)
            }
            Self::Authentication {
                message,
                code,
                mut ctx,
            } => {
                f(ctx.as_mut());
                Self::Authentication { message, code, ctx }
            }
            Self::Api(mut api) => {
                f(api.ctx.as_mut());
                Self::Api(api)
            }
            Self::Validation {
                field,
                message,
                code,
                mut ctx,
            } => {
                f(ctx.as_mut());
                Self::Validation {
                    field,
                    message,
                    code,
                    ctx,
                }
            }
            Self::Configuration {
                message,
                code,
                mut ctx,
            } => {
                f(ctx.as_mut());
                Self::Configuration { message, code, ctx }
            }
            Self::Serialization {
                message,
                source,
                code,
                mut ctx,
            } => {
                f(ctx.as_mut());
                Self::Serialization {
                    message,
                    source,
                    code,
                    ctx,
                }
            }
            Self::Business {
                code,
                message,
                mut ctx,
            } => {
                f(ctx.as_mut());
                Self::Business { code, message, ctx }
            }
            Self::Timeout {
                duration,
                operation,
                mut ctx,
            } => {
                f(ctx.as_mut());
                Self::Timeout {
                    duration,
                    operation,
                    ctx,
                }
            }
            Self::RateLimit {
                limit,
                window,
                reset_after,
                code,
                mut ctx,
            } => {
                f(ctx.as_mut());
                Self::RateLimit {
                    limit,
                    window,
                    reset_after,
                    code,
                    ctx,
                }
            }
            Self::ServiceUnavailable {
                service,
                retry_after,
                code,
                mut ctx,
            } => {
                f(ctx.as_mut());
                Self::ServiceUnavailable {
                    service,
                    retry_after,
                    code,
                    ctx,
                }
            }
            Self::ResponseTooLarge {
                limit,
                actual,
                mut ctx,
            } => {
                f(ctx.as_mut());
                Self::ResponseTooLarge { limit, actual, ctx }
            }
            Self::Internal {
                code,
                message,
                source,
                mut ctx,
            } => {
                f(ctx.as_mut());
                Self::Internal {
                    code,
                    message,
                    source,
                    ctx,
                }
            }
        }
    }

    /// 添加上下文键值对
    pub fn with_context_kv(self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        let value = value.into();
        self.map_context(|ctx| {
            ctx.add_context(key, value);
        })
    }

    /// 附加请求 ID，上下游都使用统一字段传播链路标识。
    pub fn with_request_id(self, request_id: impl Into<String>) -> Self {
        let request_id = request_id.into();
        self.map_context(|ctx| {
            ctx.set_request_id(request_id);
        })
    }

    /// 附加资源上下文，统一记录当前错误所属的业务资源/动作描述。
    pub fn with_resource(self, resource: impl Into<String>) -> Self {
        let resource = resource.into();
        self.map_context(|ctx| {
            ctx.add_context("resource", resource);
        })
    }

    /// 添加操作和组件信息
    pub fn with_operation(
        self,
        operation: impl Into<String>,
        component: impl Into<String>,
    ) -> Self {
        let operation = operation.into();
        let component = component.into();

        let mapped = self.map_context(|ctx| {
            ctx.set_operation(operation.clone())
                .set_component(component.clone())
                .add_context("operation", operation.clone())
                .add_context("component", component.clone());
        });

        match mapped {
            Self::Timeout { duration, ctx, .. } => Self::Timeout {
                duration,
                operation: Some(operation),
                ctx,
            },
            other => other,
        }
    }

    /// 一次性附加标准错误上下文字段：operation / component / resource / request_id。
    pub fn with_standard_context(
        self,
        operation: impl Into<String>,
        component: impl Into<String>,
        resource: impl Into<String>,
        request_id: Option<String>,
    ) -> Self {
        let mut err = self
            .with_operation(operation, component)
            .with_resource(resource);

        if let Some(request_id) = request_id.filter(|value| !value.trim().is_empty()) {
            err = err.with_request_id(request_id);
        }

        err
    }

    /// 观测记录（可序列化）——供日志/指标/告警统一使用
    pub fn record(&self) -> ErrorRecord {
        ErrorRecord::from(self)
    }

    // === 快速构造器（示例） ===
    /// 网络错误
    pub fn network<E>(source: E, ctx: ErrorContext) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Network(Box::new(NetworkError {
            message: "网络连接失败".to_string(),
            source: Some(Box::new(source)),
            policy: RetryPolicy::default(),
            ctx: Box::new(ctx),
        }))
    }

    /// API 错误
    pub fn api(
        status: u16,
        endpoint: impl Into<Cow<'static, str>>,
        message: impl Into<String>,
        ctx: ErrorContext,
    ) -> Self {
        Self::Api(Box::new(ApiError {
            status,
            endpoint: endpoint.into(),
            message: message.into(),
            source: None,
            code: ErrorCode::from_http_status(status),
            ctx: Box::new(ctx),
        }))
    }

    /// 响应体过大错误
    pub fn response_too_large(limit: u64, actual: u64) -> Self {
        Self::ResponseTooLarge {
            limit,
            actual,
            ctx: Box::new(ErrorContext::new()),
        }
    }
}

/// 统一的可观测记录
#[derive(Debug, Serialize)]
#[serde_with::skip_serializing_none]
pub struct ErrorRecord {
    /// 错误码
    pub code: ErrorCode,
    /// 严重程度
    pub severity: ErrorSeverity,
    /// 是否可重试
    pub retryable: bool,
    /// 重试延迟（毫秒）
    pub retry_delay_ms: Option<u64>,
    /// 错误消息
    pub message: String,
    /// 上下文
    pub context: std::collections::HashMap<String, String>,
    /// 请求 ID
    pub request_id: Option<String>,
    /// 操作名称
    pub operation: Option<String>,
    /// 组件名称
    pub component: Option<String>,
}

impl From<&CoreError> for ErrorRecord {
    fn from(err: &CoreError) -> Self {
        let ctx = err.ctx();
        Self {
            code: err.code(),
            severity: err.severity(),
            retryable: err.is_retryable(),
            retry_delay_ms: err.retry_delay(0).map(|d| d.as_millis() as u64),
            message: err.to_string(),
            context: ctx.all_context().clone(),
            request_id: ctx.request_id().map(|s| s.to_string()),
            operation: ctx.operation().map(|s| s.to_string()),
            component: ctx.component().map(|s| s.to_string()),
        }
    }
}

impl From<reqwest::Error> for CoreError {
    fn from(source: reqwest::Error) -> Self {
        Self::Network(Box::new(NetworkError {
            message: source.to_string(),
            source: Some(Box::new(source)),
            policy: RetryPolicy::default(),
            ctx: Box::new(ErrorContext::new()),
        }))
    }
}

impl From<serde_json::Error> for CoreError {
    fn from(source: serde_json::Error) -> Self {
        Self::Serialization {
            message: format!("JSON序列化错误: {source}"),
            source: Some(Box::new(source)),
            code: ErrorCode::SerializationError,
            ctx: Box::new(ErrorContext::new()),
        }
    }
}

impl ErrorTrait for CoreError {
    fn severity(&self) -> ErrorSeverity {
        self.severity()
    }

    fn is_retryable(&self) -> bool {
        self.is_retryable()
    }

    fn retry_delay(&self, attempt: u32) -> Option<Duration> {
        self.retry_delay(attempt)
    }

    fn user_message(&self) -> Option<&str> {
        match self {
            Self::Network(_) => Some("网络连接异常，请稍后重试"),
            Self::Authentication { .. } => Some("认证失败，请重新登录"),
            Self::Api(api) => Some(api.message.as_str()),
            Self::Validation { message, .. } => Some(message.as_str()),
            Self::Configuration { message, .. } => Some(message.as_str()),
            Self::Serialization { message, .. } => Some(message.as_str()),
            Self::Business { message, .. } => Some(message.as_str()),
            Self::Timeout { .. } => Some("请求超时，请稍后重试"),
            Self::RateLimit { .. } => Some("请求过于频繁，请稍候"),
            Self::ServiceUnavailable { .. } => Some("服务暂不可用，请稍后重试"),
            Self::ResponseTooLarge { .. } => Some("响应数据过大，请减小请求范围"),
            Self::Internal { message, .. } => Some(message.as_str()),
        }
    }

    fn context(&self) -> &ErrorContext {
        self.ctx()
    }

    fn error_type(&self) -> ErrorType {
        match self {
            Self::Network(_) => ErrorType::Network,
            Self::Authentication { .. } => ErrorType::Authentication,
            Self::Api(_) => ErrorType::Api,
            Self::Validation { .. } => ErrorType::Validation,
            Self::Configuration { .. } => ErrorType::Configuration,
            Self::Serialization { .. } => ErrorType::Serialization,
            Self::Business { .. } => ErrorType::Business,
            Self::Timeout { .. } => ErrorType::Timeout,
            Self::RateLimit { .. } => ErrorType::RateLimit,
            Self::ServiceUnavailable { .. } => ErrorType::ServiceUnavailable,
            Self::ResponseTooLarge { .. } => ErrorType::ResponseTooLarge,
            Self::Internal { .. } => ErrorType::Internal,
        }
    }

    fn error_code(&self) -> Option<&str> {
        None
    }
}

// 便利函数（保持向后兼容）

/// 创建网络错误
pub fn network_error(message: impl Into<String>) -> CoreError {
    CoreError::Network(Box::new(NetworkError {
        message: message.into(),
        source: None,
        policy: RetryPolicy::default(),
        ctx: Box::new(ErrorContext::new()),
    }))
}

/// 创建认证错误
pub fn authentication_error(message: impl Into<String>) -> CoreError {
    CoreError::Authentication {
        message: message.into(),
        code: ErrorCode::AuthenticationFailed,
        ctx: Box::new(ErrorContext::new()),
    }
}

/// 创建API错误
pub fn api_error(
    status: u16,
    endpoint: impl Into<String>,
    message: impl Into<String>,
    request_id: Option<String>,
) -> CoreError {
    CoreError::Api(Box::new(ApiError {
        status,
        endpoint: endpoint.into().into(),
        message: message.into(),
        source: None,
        code: ErrorCode::from_http_status(status),
        ctx: {
            let mut ctx = ErrorContext::new();
            if let Some(req_id) = request_id {
                ctx.set_request_id(req_id);
            }
            Box::new(ctx)
        },
    }))
}

/// 创建验证错误
pub fn validation_error(field: impl Into<String>, message: impl Into<String>) -> CoreError {
    let field = field.into();
    let mut ctx = ErrorContext::new();
    ctx.add_context("field", field.clone());

    CoreError::Validation {
        field: field.into(),
        message: message.into(),
        code: ErrorCode::ValidationError,
        ctx: Box::new(ctx),
    }
}

/// 创建序列化错误
pub fn serialization_error<T: std::error::Error + Send + Sync + 'static>(
    message: impl Into<String>,
    source: Option<T>,
) -> CoreError {
    CoreError::Serialization {
        message: message.into(),
        source: source.map(|e| Box::new(e) as AnyError),
        code: ErrorCode::SerializationError,
        ctx: Box::new(ErrorContext::new()),
    }
}

/// 创建业务错误
pub fn business_error(message: impl Into<String>) -> CoreError {
    CoreError::Business {
        message: message.into(),
        code: ErrorCode::BusinessError,
        ctx: Box::new(ErrorContext::new()),
    }
}

/// 创建配置错误
pub fn configuration_error(message: impl Into<String>) -> CoreError {
    CoreError::Configuration {
        message: message.into(),
        code: ErrorCode::ConfigurationError,
        ctx: Box::new(ErrorContext::new()),
    }
}

/// 创建超时错误
pub fn timeout_error(timeout: Duration, operation: Option<String>) -> CoreError {
    CoreError::Timeout {
        duration: timeout,
        operation,
        ctx: Box::new(ErrorContext::new()),
    }
}

/// 创建限流错误
pub fn rate_limit_error(limit: u32, window: Duration, retry_after: Option<Duration>) -> CoreError {
    CoreError::RateLimit {
        limit,
        window,
        reset_after: retry_after,
        code: ErrorCode::TooManyRequests,
        ctx: Box::new(ErrorContext::new()),
    }
}

/// 创建服务不可用错误
pub fn service_unavailable_error(
    service: impl Into<String>,
    retry_after: Option<Duration>,
) -> CoreError {
    CoreError::ServiceUnavailable {
        service: service.into().into(),
        retry_after,
        code: ErrorCode::ServiceUnavailable,
        ctx: Box::new(ErrorContext::new()),
    }
}

/// 创建权限缺失错误
pub fn permission_missing_error(scopes: &[impl AsRef<str>]) -> CoreError {
    let mut ctx = ErrorContext::new();
    ctx.add_context(
        "required_scopes",
        scopes
            .iter()
            .map(|s| s.as_ref())
            .collect::<Vec<_>>()
            .join(","),
    );

    CoreError::Authentication {
        message: "权限范围不足".to_string(),
        code: ErrorCode::PermissionMissing,
        ctx: Box::new(ctx),
    }
}

/// 创建SSO令牌无效错误
pub fn sso_token_invalid_error(detail: impl Into<String>) -> CoreError {
    let mut ctx = ErrorContext::new();
    ctx.add_context("detail", detail.into());

    CoreError::Authentication {
        message: "SSO令牌无效".to_string(),
        code: ErrorCode::SsoTokenInvalid,
        ctx: Box::new(ctx),
    }
}

/// 创建用户身份无效错误
pub fn user_identity_invalid_error(desc: impl Into<String>) -> CoreError {
    let mut ctx = ErrorContext::new();
    ctx.add_context("description", desc.into());

    CoreError::Authentication {
        message: "用户身份无效".to_string(),
        code: ErrorCode::UserIdentityInvalid,
        ctx: Box::new(ctx),
    }
}

/// 创建访问令牌无效错误
pub fn token_invalid_error(detail: impl Into<String>) -> CoreError {
    let mut ctx = ErrorContext::new();
    ctx.add_context("detail", detail.into());

    CoreError::Authentication {
        message: "访问令牌无效".to_string(),
        code: ErrorCode::AccessTokenInvalid,
        ctx: Box::new(ctx),
    }
}

/// 创建访问令牌过期错误
pub fn token_expired_error(detail: impl Into<String>) -> CoreError {
    let mut ctx = ErrorContext::new();
    ctx.add_context("detail", detail.into());

    CoreError::Authentication {
        message: "访问令牌过期".to_string(),
        code: ErrorCode::AccessTokenExpiredV2,
        ctx: Box::new(ctx),
    }
}

/// 创建带详细信息的网络错误
pub fn network_error_with_details(
    message: impl Into<String>,
    endpoint: Option<String>,
    request_id: Option<String>,
) -> CoreError {
    let mut ctx = ErrorContext::new();
    if let Some(ep) = endpoint {
        ctx.add_context("endpoint", ep);
    }
    if let Some(req_id) = request_id {
        ctx.set_request_id(req_id);
    }

    CoreError::Network(Box::new(NetworkError {
        message: message.into(),
        source: None,
        policy: RetryPolicy::default(),
        ctx: Box::new(ctx),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn api_error_has_code_and_severity() {
        let err = CoreError::api(503, "/ping", "service down", ErrorContext::new());

        assert_eq!(err.code(), ErrorCode::ServiceUnavailable);
        assert!(err.is_retryable());
        assert_eq!(err.severity(), ErrorSeverity::Critical);
        assert!(err.retry_delay(1).is_some());
    }

    #[test]
    fn record_contains_context() {
        let mut ctx = ErrorContext::new();
        ctx.add_context("endpoint", "/user");
        ctx.set_request_id("req-1");

        let err = CoreError::network(std::io::Error::other("boom"), ctx);

        let rec = err.record();
        assert_eq!(rec.code, ErrorCode::NetworkConnectionFailed);
        assert_eq!(rec.context.get("endpoint"), Some(&"/user".to_string()));
        assert_eq!(rec.request_id.as_deref(), Some("req-1"));
    }

    #[test]
    fn core_error_to_record() {
        let err = CoreError::api(503, "/ping", "svc down", ErrorContext::new());
        let rec: ErrorRecord = (&err).into();
        assert_eq!(rec.code, ErrorCode::ServiceUnavailable);
        assert!(rec.retryable);
        assert!(rec.message.contains("API错误"));
    }

    #[test]
    fn builder_creates_api_error_with_context() {
        let err = CoreError::api_builder()
            .status(404)
            .endpoint("/users/1")
            .message("not found")
            .request_id("req-123")
            .build();

        assert!(err.is_api_error());
        assert_eq!(err.context().request_id(), Some("req-123"));
        assert_eq!(err.code(), ErrorCode::NotFound);
    }

    #[test]
    fn rate_limit_retry_delay() {
        let err = CoreError::RateLimit {
            limit: 10,
            window: Duration::from_secs(60),
            reset_after: Some(Duration::from_secs(30)),
            code: ErrorCode::RateLimitExceeded,
            ctx: Box::new(ErrorContext::new()),
        };

        assert!(err.is_retryable());
        assert_eq!(err.retry_delay(0), Some(Duration::from_secs(60)));
    }

    #[test]
    fn from_reqwest_error() {
        // 无法直接构造 reqwest::Error（构造函数为私有），跳过具体实例化，只验证 From trait 存在
        fn assert_from_reqwest<E: Into<CoreError>>() {}
        assert_from_reqwest::<reqwest::Error>();
    }

    #[test]
    fn map_context_covers_all_variants() {
        let errors = vec![
            network_error("n"),
            authentication_error("a"),
            api_error(500, "/api", "api", None),
            validation_error("field", "invalid"),
            configuration_error("cfg"),
            serialization_error("serde", None::<serde_json::Error>),
            business_error("biz"),
            timeout_error(Duration::from_secs(1), None),
            rate_limit_error(100, Duration::from_secs(60), Some(Duration::from_secs(10))),
            service_unavailable_error("svc", Some(Duration::from_secs(30))),
            CoreError::response_too_large(1024 * 1024, 5 * 1024 * 1024),
            CoreError::Internal {
                code: ErrorCode::InternalError,
                message: "internal".to_string(),
                source: None,
                ctx: Box::new(ErrorContext::new()),
            },
        ];

        for err in errors {
            let updated = err.map_context(|ctx| {
                ctx.add_context("k", "v");
            });
            assert_eq!(updated.context().get_context("k"), Some("v"));
        }
    }

    #[test]
    fn with_context_kv_adds_context() {
        let err = validation_error("field", "invalid").with_context_kv("user_id", "u-1");
        assert_eq!(err.context().get_context("user_id"), Some("u-1"));
    }

    #[test]
    fn with_request_id_updates_context() {
        let err = validation_error("field", "invalid").with_request_id("req-123");
        assert_eq!(err.context().request_id(), Some("req-123"));
    }

    #[test]
    fn with_resource_adds_resource_context() {
        let err = validation_error("field", "invalid").with_resource("查询记录");
        assert_eq!(err.context().get_context("resource"), Some("查询记录"));
    }

    #[test]
    fn with_operation_updates_timeout_field_and_context() {
        let err = timeout_error(Duration::from_secs(30), Some("old_op".to_string()))
            .with_operation("new_op", "client");

        match err {
            CoreError::Timeout {
                operation, ref ctx, ..
            } => {
                assert_eq!(operation.as_deref(), Some("new_op"));
                assert_eq!(ctx.operation(), Some("new_op"));
                assert_eq!(ctx.component(), Some("client"));
                assert_eq!(ctx.get_context("operation"), Some("new_op"));
                assert_eq!(ctx.get_context("component"), Some("client"));
            }
            other => panic!("expected timeout error, got {:?}", other.error_type()),
        }
    }

    #[test]
    fn with_standard_context_sets_all_standard_fields() {
        let err = validation_error("response.data", "响应数据为空").with_standard_context(
            "extract_response_data",
            "openlark-docs",
            "查询记录",
            Some("req-456".to_string()),
        );

        assert_eq!(err.context().operation(), Some("extract_response_data"));
        assert_eq!(err.context().component(), Some("openlark-docs"));
        assert_eq!(err.context().get_context("resource"), Some("查询记录"));
        assert_eq!(err.context().request_id(), Some("req-456"));
    }
}
