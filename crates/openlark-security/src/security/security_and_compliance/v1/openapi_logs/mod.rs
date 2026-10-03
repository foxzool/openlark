//! OpenAPI 审计日志 API（门面）
//!
//! [`OpenApiLogsService`] 是轻量门面，返回 [`list_data`] 下的端点构建器。

pub mod list_data;

use openlark_core::config::Config;

/// OpenAPI 审计日志服务
///
/// 不直接发请求，仅返回端点构建器。
#[derive(Debug, Clone)]
pub struct OpenApiLogsService {
    config: Config,
}

impl OpenApiLogsService {
    /// 创建新的审计日志服务实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取审计日志数据（POST `/openapi_logs/list_data`）。
    pub fn list_data(&self) -> list_data::ListOpenApiLogsRequest {
        list_data::ListOpenApiLogsRequest::new(self.config.clone())
    }
}
