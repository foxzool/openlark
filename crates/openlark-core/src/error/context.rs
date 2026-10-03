//! 现代化错误上下文
//!
//! 与 thiserror-based CoreError 完美配合的高性能错误上下文系统

use std::collections::HashMap;

/// 现代化错误上下文
///
/// 提供丰富的错误上下文信息，支持结构化数据和链式错误追踪
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// 用户友好消息
    user_message: Option<String>,
    /// 上下文信息
    context: HashMap<String, String>,
    /// 时间戳（错误发生时间）
    timestamp: Option<std::time::SystemTime>,
    /// 请求ID（用于链路追踪）
    request_id: Option<String>,
    /// 操作名称
    operation: Option<String>,
    /// 组件名称
    component: Option<String>,
}

impl ErrorContext {
    /// 创建新的错误上下文
    pub fn new() -> Self {
        Self {
            user_message: None,
            context: HashMap::new(),
            timestamp: Some(std::time::SystemTime::now()),
            request_id: None,
            operation: None,
            component: None,
        }
    }

    /// 创建带有用户消息的错误上下文
    pub fn with_user_message(message: impl Into<String>) -> Self {
        Self {
            user_message: Some(message.into()),
            context: HashMap::new(),
            timestamp: Some(std::time::SystemTime::now()),
            request_id: None,
            operation: None,
            component: None,
        }
    }

    /// 创建带有操作名的错误上下文
    pub fn with_operation(operation: impl Into<String>) -> Self {
        Self {
            user_message: None,
            context: HashMap::new(),
            timestamp: Some(std::time::SystemTime::now()),
            request_id: None,
            operation: Some(operation.into()),
            component: None,
        }
    }

    /// 设置用户友好消息
    pub fn set_user_message(&mut self, message: impl Into<String>) {
        self.user_message = Some(message.into());
    }

    /// 获取用户友好消息
    pub fn user_message(&self) -> Option<&str> {
        self.user_message.as_deref()
    }

    /// 添加上下文信息
    pub fn add_context(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.context.insert(key.into(), value.into());
        self
    }

    /// 批量添加上下文信息
    pub fn extend_context<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.context
            .extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// 获取上下文值
    pub fn get_context(&self, key: &str) -> Option<&str> {
        self.context.get(key).map(|s| s.as_str())
    }

    /// 获取所有上下文信息
    pub fn all_context(&self) -> &HashMap<String, String> {
        &self.context
    }

    /// 检查是否有指定的上下文键
    pub fn has_context(&self, key: &str) -> bool {
        self.context.contains_key(key)
    }

    /// 设置请求ID
    pub fn set_request_id(&mut self, request_id: impl Into<String>) -> &mut Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// 获取请求ID
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    /// 设置操作名称
    pub fn set_operation(&mut self, operation: impl Into<String>) -> &mut Self {
        self.operation = Some(operation.into());
        self
    }

    /// 获取操作名称
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// 设置组件名称
    pub fn set_component(&mut self, component: impl Into<String>) -> &mut Self {
        self.component = Some(component.into());
        self
    }

    /// 获取组件名称
    pub fn component(&self) -> Option<&str> {
        self.component.as_deref()
    }

    /// 获取时间戳
    pub fn timestamp(&self) -> Option<std::time::SystemTime> {
        self.timestamp
    }

    /// 清空所有上下文信息
    /// 清空所有上下文信息
    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    /// 获取上下文信息数量
    pub fn context_len(&self) -> usize {
        self.context.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.user_message.is_none()
            && self.context.is_empty()
            && self.request_id.is_none()
            && self.operation.is_none()
            && self.component.is_none()
    }

    /// 转换为调试格式
    pub fn debug_format(&self) -> String {
        let mut parts = Vec::new();

        if let Some(timestamp) = self.timestamp {
            // 手动格式化 SystemTime
            let duration = timestamp
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            let secs = duration.as_secs();
            let datetime =
                chrono::DateTime::from_timestamp(secs as i64, 0).unwrap_or_else(chrono::Utc::now);
            parts.push(format!(
                "时间: {}",
                datetime.format("%Y-%m-%d %H:%M:%S UTC")
            ));
        }

        if let Some(component) = &self.component {
            parts.push(format!("组件: {component}"));
        }

        if let Some(operation) = &self.operation {
            parts.push(format!("操作: {operation}"));
        }

        if let Some(request_id) = &self.request_id {
            parts.push(format!("请求ID: {request_id}"));
        }

        if !self.context.is_empty() {
            parts.push("上下文:".to_string());
            for (key, value) in &self.context {
                parts.push(format!("  {key}: {value}"));
            }
        }

        if let Some(message) = &self.user_message {
            parts.push(format!("用户消息: {message}"));
        }

        parts.join("\n")
    }

    /// 从当前上下文创建副本，并可以添加额外信息
    pub fn clone_with(&self) -> Self {
        let mut clone = self.clone();
        clone.timestamp = Some(std::time::SystemTime::now());
        clone
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

/// 错误上下文构建器
///
/// 提供流式API来构建复杂的错误上下文
#[derive(Debug, Clone)]
pub struct ErrorContextBuilder {
    context: ErrorContext,
}

impl ErrorContextBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            context: ErrorContext::new(),
        }
    }

    /// 设置用户消息
    pub fn user_message(mut self, message: impl Into<String>) -> Self {
        self.context.set_user_message(message);
        self
    }

    /// 添加上下文信息
    pub fn context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.add_context(key, value);
        self
    }

    /// 批量添加上下文信息
    pub fn extend_context<I, K, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.context.extend_context(iter);
        self
    }

    /// 设置请求ID
    pub fn request_id(mut self, request_id: impl Into<String>) -> Self {
        self.context.set_request_id(request_id);
        self
    }

    /// 设置操作名称
    pub fn operation(mut self, operation: impl Into<String>) -> Self {
        self.context.set_operation(operation);
        self
    }

    /// 设置组件名称
    pub fn component(mut self, component: impl Into<String>) -> Self {
        self.context.set_component(component);
        self
    }

    /// 构建错误上下文
    pub fn build(self) -> ErrorContext {
        self.context
    }
}

impl Default for ErrorContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug_format())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_context_basic() {
        let mut context = ErrorContext::new();
        context.add_context("endpoint", "https://api.example.com");
        context.set_user_message("连接失败");

        assert!(context.has_context("endpoint"));
        assert_eq!(
            context.get_context("endpoint"),
            Some("https://api.example.com")
        );
        assert_eq!(context.user_message(), Some("连接失败"));
    }

    #[test]
    fn test_error_context_constructors() {
        let context_with_msg = ErrorContext::with_user_message("测试消息");
        assert_eq!(context_with_msg.user_message(), Some("测试消息"));

        let context_with_op = ErrorContext::with_operation("api_call");
        assert_eq!(context_with_op.operation(), Some("api_call"));
    }

    #[test]
    fn test_error_context_builder() {
        let context = ErrorContextBuilder::new()
            .user_message("构建器测试")
            .context("url", "https://example.com")
            .context("method", "GET")
            .operation("http_request")
            .component("http_client")
            .request_id("req-123")
            .build();

        assert_eq!(context.user_message(), Some("构建器测试"));
        assert_eq!(context.get_context("url"), Some("https://example.com"));
        assert_eq!(context.get_context("method"), Some("GET"));
        assert_eq!(context.operation(), Some("http_request"));
        assert_eq!(context.component(), Some("http_client"));
        assert_eq!(context.request_id(), Some("req-123"));
    }

    #[test]
    fn test_error_context_chain_operations() {
        let mut context = ErrorContext::new()
            .add_context("key1", "value1")
            .add_context("key2", "value2")
            .clone_with();

        context
            .add_context("key3", "value3")
            .set_operation("test_operation");

        assert_eq!(context.get_context("key1"), Some("value1"));
        assert_eq!(context.get_context("key2"), Some("value2"));
        assert_eq!(context.get_context("key3"), Some("value3"));
        assert_eq!(context.operation(), Some("test_operation"));
    }

    #[test]
    fn test_error_context_extend() {
        let mut context = ErrorContext::new();
        context.extend_context(vec![
            ("key1", "value1"),
            ("key2", "value2"),
            ("key3", "value3"),
        ]);

        assert_eq!(context.context_len(), 3);
        assert!(context.has_context("key1"));
        assert!(context.has_context("key2"));
        assert!(context.has_context("key3"));
    }

    #[test]
    fn test_error_context_debug_format() {
        let context = ErrorContextBuilder::new()
            .user_message("测试错误")
            .context("api", "send_message")
            .operation("message_send")
            .component("communication")
            .request_id("req-456")
            .build();

        let debug_str = context.debug_format();
        assert!(debug_str.contains("测试错误"));
        assert!(debug_str.contains("api"));
        assert!(debug_str.contains("send_message"));
        assert!(debug_str.contains("message_send"));
        assert!(debug_str.contains("communication"));
        assert!(debug_str.contains("req-456"));
        assert!(debug_str.contains("时间:"));
    }

    #[test]
    fn test_error_context_is_empty() {
        let empty_context = ErrorContext::new();
        assert!(empty_context.is_empty());

        let mut non_empty_context = ErrorContext::new();
        non_empty_context.add_context("test", "value");
        assert!(!non_empty_context.is_empty());
    }

    #[test]
    fn test_error_context_clone_with() {
        let mut original = ErrorContext::new();
        original.add_context("original", "data");
        original.set_user_message("原始消息");

        let cloned = original.clone_with();

        // 验证所有数据都被复制
        assert_eq!(cloned.get_context("original"), Some("data"));
        assert_eq!(cloned.user_message(), Some("原始消息"));

        // 验证时间戳被更新
        assert!(cloned.timestamp() > original.timestamp());
    }
}
