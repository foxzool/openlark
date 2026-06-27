//! 审计日志 API
//!
//! 当前仍是 runtime stub。
//!
//! 平台 admin 已接入的真实审计相关接口是 `audit_info/list.rs`，
//! 当前这个更宽泛的 `audit` facade 并没有对应的已接线服务端端点。
//! 为避免继续返回占位 JSON，本模块现在会显式返回未接线错误。

use crate::PlatformConfig;
use openlark_core::{SDKResult, error::business_error, req_option::RequestOption};
use std::sync::Arc;

/// 审计日志 facade。
#[derive(Debug, Clone)]
pub struct AuditApi {
    config: Arc<PlatformConfig>,
}

impl AuditApi {
    /// 创建新的审计日志 facade。
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// 查询审计日志
    pub fn query(&self) -> QueryAuditLogsRequest {
        QueryAuditLogsRequest::new(self.config.clone())
    }

    /// 获取日志详情
    pub fn get(&self) -> GetAuditLogRequest {
        GetAuditLogRequest::new(self.config.clone())
    }
}

/// 查询审计日志请求
pub struct QueryAuditLogsRequest {
    config: Arc<PlatformConfig>,
    start_time: Option<String>,
    end_time: Option<String>,
    page_size: Option<u32>,
}

impl QueryAuditLogsRequest {
    fn new(config: Arc<PlatformConfig>) -> Self {
        Self {
            config,
            start_time: None,
            end_time: None,
            page_size: None,
        }
    }

    /// 设置开始时间
    pub fn start_time(mut self, time: impl Into<String>) -> Self {
        self.start_time = Some(time.into());
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, time: impl Into<String>) -> Self {
        self.end_time = Some(time.into());
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求并传入请求选项。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        Err(business_error(
            "admin.audit.query: openlark-platform 尚未接入该 facade，请改用已实现的 admin.audit_info.list 等真实端点",
        ))
    }
}

/// 获取日志详情请求
pub struct GetAuditLogRequest {
    config: Arc<PlatformConfig>,
    log_id: Option<String>,
}

impl GetAuditLogRequest {
    fn new(config: Arc<PlatformConfig>) -> Self {
        Self {
            config,
            log_id: None,
        }
    }

    /// 设置日志 ID
    pub fn log_id(mut self, log_id: impl Into<String>) -> Self {
        self.log_id = Some(log_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求并传入请求选项。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        Err(business_error(
            "admin.audit.get: openlark-platform 尚未接入该 facade，请改用已实现的 admin.audit_info.list 等真实端点",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }

    #[tokio::test]
    async fn test_audit_stub_returns_explicit_error() {
        let config = Arc::new(PlatformConfig::default());
        let err = AuditApi::new(config)
            .query()
            .start_time("2026-01-01")
            .end_time("2026-01-31")
            .execute()
            .await
            .expect_err("audit stub should now fail explicitly");
        assert!(err.to_string().contains("尚未接入"));
    }
}
