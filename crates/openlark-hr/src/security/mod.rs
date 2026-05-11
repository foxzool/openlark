//! 安全合规（Security and Compliance）服务
//!
//! 提供飞书安全合规的完整功能集，支持审计日志、OpenAPI日志、
//! 行为监控、合规检查等企业级安全管理能力。是企业安全体系的重要组成部分。
//!
//! # 核心功能
//!
//! ## OpenAPI审计日志
//! - 📋 OpenAPI调用日志记录
//! - 🔍 API访问行为分析
//! - 📊 API使用统计报告
//! - 🚨 异常API调用检测
//! - 📈 API性能监控分析
//!
//! ## 行为审计日志
//! - 👤 用户行为全程记录
//! - 🔒 敏感操作审计跟踪
//! - 📊 行为模式分析统计
//! - 🚨 异常行为告警机制
//! - 📋 合规性审查支持
//!
//! ## 安全监控
//! - 🔍 实时安全事件监控
//! - 🚨 安全威胁检测告警
//! - 📊 安全态势分析展示
//! - 🛡️ 安全策略执行监督
//! - 📈 安全指标统计分析
//!
//! ## 合规管理
//! - 📋 合规规则配置管理
//! - ✅ 合规状态检查验证
//! - 📊 合规报告自动生成
//! - 🔍 合规问题追溯分析
//! - 📈 合规趋势监控评估
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取安全合规服务
//! let security = &client.security_and_compliance;
//!
//! // 查询OpenAPI日志
//! // let api_log_request = QueryOpenapiLogRequest::builder()
//! //     .start_time("2024-01-01T00:00:00Z")
//! //     .end_time("2024-01-31T23:59:59Z")
//! //     .api_path("/open-apis/im/v1/messages")
//! //     .page_size(100)
//! //     .build();
//! // let api_logs = security.openapi_log.query(api_log_request, None).await?;
//!
//! // 查询审计日志
//! // let audit_request = QueryAuditLogRequest::builder()
//! //     .start_time("2024-01-01T00:00:00Z")
//! //     .end_time("2024-01-31T23:59:59Z")
//! //     .event_type("user_login")
//! //     .user_id("user_123")
//! //     .build();
//! // let audit_logs = security.audit_log.query(audit_request, None).await?;
//!
//! // 获取安全统计
//! // let stats_request = GetSecurityStatsRequest::builder()
//! //     .date_range("last_30_days")
//! //     .metric_types(vec!["api_calls", "login_attempts", "data_access"])
//! //     .build();
//! // let stats = security.audit_log.get_stats(stats_request, None).await?;
//!
//! // 检查合规状态
//! // let compliance_request = CheckComplianceRequest::builder()
//! //     .check_type("data_retention")
//! //     .scope("tenant")
//! //     .build();
//! // let compliance_status = security.audit_log.check_compliance(compliance_request, None).await?;
//! ```
//!
//! # 安全管理特性
//!
//! - 🔐 全方位的安全监控
//! - 📋 完整的审计追踪
//! - 🚨 智能的异常检测
//! - 📊 详细的分析报告
//! - ✅ 自动化的合规检查
//!
//! # 企业应用
//!
//! - 🏢 企业安全管理体系
//! - 📋 合规性审查支持
//! - 🔍 安全事件调查分析
//! - 📊 安全风险评估
//! - 🛡️ 数据保护和隐私

use openlark_core::config::Config;

pub mod audit_log;
pub mod models;
pub use self::models::*;
pub mod openapi_log;

pub use audit_log::AuditLogService;
pub use openapi_log::OpenapiLogService;

/// 安全合规服务
///
/// 提供飞书安全合规相关功能，包括：
/// - OpenAPI 审计日志
/// - 行为审计日志
pub struct SecurityAndComplianceService {
    pub openapi_log: OpenapiLogService,
    pub audit_log: AuditLogService,
}

impl SecurityAndComplianceService {
    pub fn new(config: Config) -> Self {
        Self {
            openapi_log: OpenapiLogService::new(config.clone()),
            audit_log: AuditLogService::new(config),
        }
    }

    /// 使用共享配置创建服务（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            openapi_log: OpenapiLogService::new(shared.as_ref().clone()),
            audit_log: AuditLogService::new(shared.as_ref().clone()),
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_security_and_compliance_service_creation() {
        let config = Config::default();
        let service = SecurityAndComplianceService::new(config.clone());

        assert_eq!(service.openapi_log.config.app_id, config.app_id);
        assert_eq!(service.openapi_log.config.app_secret, config.app_secret);
        assert_eq!(service.audit_log.config.app_id, config.app_id);
        assert_eq!(service.audit_log.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_security_and_compliance_service_with_custom_config() {
        let config = Config::builder()
            .app_id("security_test_app")
            .app_secret("security_test_secret")
            .req_timeout(Duration::from_secs(180))
            .build();

        let service = SecurityAndComplianceService::new(config.clone());

        assert_eq!(service.openapi_log.config.app_id, "security_test_app");
        assert_eq!(
            service.openapi_log.config.app_secret,
            "security_test_secret"
        );
        assert_eq!(
            service.openapi_log.config.req_timeout,
            Some(Duration::from_secs(180))
        );
        assert_eq!(service.audit_log.config.app_id, "security_test_app");
        assert_eq!(
            service.audit_log.config.req_timeout,
            Some(Duration::from_secs(180))
        );
    }

    #[test]
    fn test_security_and_compliance_service_config_independence() {
        let config1 = Config::builder().app_id("security_app_1").build();

        let config2 = Config::builder().app_id("security_app_2").build();

        let service1 = SecurityAndComplianceService::new(config1);
        let service2 = SecurityAndComplianceService::new(config2);

        assert_eq!(service1.openapi_log.config.app_id, "security_app_1");
        assert_eq!(service2.openapi_log.config.app_id, "security_app_2");
        assert_ne!(
            service1.openapi_log.config.app_id,
            service2.openapi_log.config.app_id
        );
        assert_ne!(
            service1.audit_log.config.app_id,
            service2.audit_log.config.app_id
        );
    }

    #[test]
    fn test_security_and_compliance_service_sub_services_accessible() {
        let config = Config::default();
        let service = SecurityAndComplianceService::new(config.clone());

        assert_eq!(service.openapi_log.config.app_id, config.app_id);
        assert_eq!(service.audit_log.config.app_id, config.app_id);
    }

    #[test]
    fn test_security_and_compliance_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = SecurityAndComplianceService::new(config.clone());

        assert_eq!(service.openapi_log.config.app_id, "clone_test_app");
        assert_eq!(service.openapi_log.config.app_secret, "clone_test_secret");
        assert_eq!(service.audit_log.config.app_secret, "clone_test_secret");
        assert_eq!(service.audit_log.config.app_id, "clone_test_app");
    }

    #[test]
    fn test_security_and_compliance_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(200))
            .build();

        let service = SecurityAndComplianceService::new(config);

        assert_eq!(
            service.openapi_log.config.req_timeout,
            Some(Duration::from_secs(200))
        );
        assert_eq!(
            service.audit_log.config.req_timeout,
            Some(Duration::from_secs(200))
        );
    }

    #[test]
    fn test_security_and_compliance_service_multiple_instances() {
        let config = Config::default();

        let service1 = SecurityAndComplianceService::new(config.clone());
        let service2 = SecurityAndComplianceService::new(config.clone());

        assert_eq!(
            service1.openapi_log.config.app_id,
            service2.openapi_log.config.app_id
        );
        assert_eq!(
            service1.openapi_log.config.app_secret,
            service2.openapi_log.config.app_secret
        );
        assert_eq!(
            service1.audit_log.config.app_id,
            service2.audit_log.config.app_id
        );
        assert_eq!(
            service1.audit_log.config.app_secret,
            service2.audit_log.config.app_secret
        );
    }

    #[test]
    fn test_security_and_compliance_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(150))
            .build();

        let service = SecurityAndComplianceService::new(config);

        assert_eq!(service.openapi_log.config.app_id, "consistency_test");
        assert_eq!(service.openapi_log.config.app_secret, "consistency_secret");
        assert_eq!(
            service.openapi_log.config.req_timeout,
            Some(Duration::from_secs(150))
        );
        assert_eq!(service.audit_log.config.app_id, "consistency_test");
        assert_eq!(service.audit_log.config.app_secret, "consistency_secret");
        assert_eq!(
            service.audit_log.config.req_timeout,
            Some(Duration::from_secs(150))
        );
    }
}
