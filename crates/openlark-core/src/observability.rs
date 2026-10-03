//! 可观测性模块
//!
//! 提供响应处理跟踪（`ResponseTracker`），记录响应解析、验证、总耗时与成功/失败，
//! 供 `request_execution::decode` 在处理响应时埋点。
//!
//! 历史上的 `OperationTracker`/`HttpTracker`/`AuthTracker`/`trace_health_check`/
//! `trace_*` 宏及 `tracing-init`/`otel` 初始化函数均为 0 引用死代码，已在 #277
//! （inner-attribute 收尾）删除；本模块现仅保留被 `request_execution::decode` 实际使用的
//! `ResponseTracker`。

use std::time::Instant;
use tracing::{Level, Span, span};

/// 响应处理跟踪器
///
/// 记录响应解析、验证、总耗时与成功/失败。
pub struct ResponseTracker {
    span: Span,
    start_time: Instant,
}

impl ResponseTracker {
    /// 开始跟踪响应处理
    pub fn start(response_format: &str, response_size: Option<u64>) -> Self {
        let span = span!(
            Level::DEBUG,
            "response_processing",
            format = response_format,
            input_size = response_size.unwrap_or(0),
            parsing_duration_ms = tracing::field::Empty,
            validation_duration_ms = tracing::field::Empty,
            total_duration_ms = tracing::field::Empty,
            success = tracing::field::Empty,
        );

        {
            let _enter = span.enter();
            tracing::debug!("Starting response processing");
        }

        Self {
            span,
            start_time: Instant::now(),
        }
    }

    /// 记录解析阶段完成
    pub fn parsing_complete(&self) {
        let parsing_duration = self.start_time.elapsed();
        let parsing_duration_ms = parsing_duration.as_millis() as u64;
        self.span.record("parsing_duration_ms", parsing_duration_ms);
    }

    /// 记录验证阶段完成
    pub fn validation_complete(&self) {
        let total_duration = self.start_time.elapsed();
        let validation_duration_ms = total_duration.as_millis() as u64;
        self.span
            .record("validation_duration_ms", validation_duration_ms);
    }

    /// 记录处理成功
    pub fn success(self) {
        let duration = self.start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;

        self.span.record("total_duration_ms", duration_ms);
        self.span.record("success", true);

        let _enter = self.span.enter();
        tracing::debug!("Response processing completed successfully");
    }

    /// 记录处理失败
    pub fn error(self, error: &str) {
        let duration = self.start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;

        self.span.record("total_duration_ms", duration_ms);
        self.span.record("success", false);

        let _enter = self.span.enter();
        tracing::error!(error = error, "Response processing failed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_response_tracker() {
        let tracker = ResponseTracker::start("json", Some(1024));
        std::thread::sleep(Duration::from_millis(5));

        // 模拟解析完成
        tracker.parsing_complete();
        std::thread::sleep(Duration::from_millis(3));

        // 模拟验证完成
        tracker.validation_complete();

        // 完成处理
        tracker.success();

        // 验证响应处理日志
        assert!(logs_contain("Starting response processing"));
        assert!(logs_contain("Response processing completed successfully"));
    }

    #[traced_test]
    #[test]
    fn test_response_tracker_error() {
        let tracker = ResponseTracker::start("xml", Some(512));
        tracker.error("Parse error: invalid XML structure");

        // 验证响应处理错误日志
        assert!(logs_contain("Starting response processing"));
        assert!(logs_contain("Response processing failed"));
        assert!(logs_contain("Parse error: invalid XML structure"));
    }

    #[traced_test]
    #[test]
    fn test_response_tracker_with_none_size() {
        let tracker = ResponseTracker::start("xml", None);

        // Test parsing without validation
        tracker.parsing_complete();
        tracker.success();

        assert!(logs_contain("Starting response processing"));
        assert!(logs_contain("Response processing completed successfully"));
    }

    #[traced_test]
    #[test]
    fn test_response_tracker_validation_timing() {
        let tracker = ResponseTracker::start("binary", Some(2048));

        // Test timing sequence
        std::thread::sleep(Duration::from_millis(2));
        tracker.parsing_complete();

        std::thread::sleep(Duration::from_millis(3));
        tracker.validation_complete();

        std::thread::sleep(Duration::from_millis(1));
        tracker.success();

        assert!(logs_contain("Starting response processing"));
        assert!(logs_contain("Response processing completed successfully"));
    }
}
