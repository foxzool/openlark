//! 审计日志相关 API

pub mod audit_log_list;
pub mod data_change_log_detail;
pub mod data_change_logs_list;
pub mod get;

use openlark_core::config::Config;

/// application.audit_log 服务（叶子级，持 namespace）
#[derive(Debug, Clone)]
pub struct AuditLogService {
    config: Config,
    namespace: String,
}

impl AuditLogService {
    /// 创建新的 audit_log 服务
    pub fn new(config: Config, namespace: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
        }
    }
    /// 查询审计日志列表
    pub fn list(&self) -> audit_log_list::AuditLogListRequestBuilder {
        audit_log_list::AuditLogListRequestBuilder::new(self.config.clone(), self.namespace.clone())
    }
    /// 获取审计日志详情
    pub fn get(&self, log_id: impl Into<String>) -> get::AuditLogGetRequestBuilder {
        get::AuditLogGetRequestBuilder::new(self.config.clone(), self.namespace.clone(), log_id)
    }
    /// 查询数据变更日志列表
    pub fn data_change_logs_list(&self) -> data_change_logs_list::DataChangeLogsListRequestBuilder {
        data_change_logs_list::DataChangeLogsListRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
        )
    }
    /// 获取数据变更日志详情
    pub fn data_change_log_detail(
        &self,
        log_id: impl Into<String>,
    ) -> data_change_log_detail::DataChangeLogDetailRequestBuilder {
        data_change_log_detail::DataChangeLogDetailRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            log_id,
        )
    }
}
