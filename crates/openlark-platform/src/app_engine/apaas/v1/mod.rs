//! aPaaS V1 API
//!
//! 提供 aPaaS V1 版本的 API 访问

use crate::PlatformConfig;
use std::sync::Arc;

/// 应用列表与基础信息接口。
pub mod app;
/// 应用级对象、函数与环境变量接口。
pub mod application;
/// 审批实例查询与取消接口。
pub mod approval_instance;
/// 审批任务处理接口。
pub mod approval_task;
/// 席位活跃度查询接口。
pub mod seat_activity;
/// 席位分配查询接口。
pub mod seat_assignment;
/// 用户任务处理接口。
pub mod user_task;
/// 工作空间数据与元数据接口。
pub mod workspace;

/// aPaaS V1 API
#[derive(Debug, Clone)]
pub struct ApaasV1 {
    config: Arc<PlatformConfig>,
}

impl ApaasV1 {
    /// 创建新的 aPaaS V1 实例。
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// app 资源
    pub fn app(&self) -> app::AppService {
        app::AppService::new(self.config.as_ref().clone())
    }

    /// approval_task 资源
    pub fn approval_task(&self) -> approval_task::ApprovalTaskService {
        approval_task::ApprovalTaskService::new(self.config.as_ref().clone())
    }

    /// approval_instance 资源
    pub fn approval_instance(&self) -> approval_instance::ApprovalInstanceService {
        approval_instance::ApprovalInstanceService::new(self.config.as_ref().clone())
    }

    /// user_task 资源
    pub fn user_task(&self) -> user_task::UserTaskService {
        user_task::UserTaskService::new(self.config.as_ref().clone())
    }

    /// seat_activity 资源
    pub fn seat_activity(&self) -> seat_activity::SeatActivityService {
        seat_activity::SeatActivityService::new(self.config.as_ref().clone())
    }

    /// seat_assignment 资源
    pub fn seat_assignment(&self) -> seat_assignment::SeatAssignmentService {
        seat_assignment::SeatAssignmentService::new(self.config.as_ref().clone())
    }

    /// application 资源（中间级，持 namespace 路径参数）
    pub fn application(&self, namespace: impl Into<String>) -> application::ApplicationService {
        application::ApplicationService::new(self.config.clone(), namespace)
    }

    /// workspace 资源（中间级，持 workspace_id 路径参数）
    pub fn workspace(&self, workspace_id: impl Into<String>) -> workspace::WorkspaceService {
        workspace::WorkspaceService::new(self.config.clone(), workspace_id)
    }
}

#[cfg(test)]
mod tests {
    use super::ApaasV1;
    use crate::PlatformConfig;

    #[test]
    fn test_apaas_v1_creation() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let api = ApaasV1::new(std::sync::Arc::new(config));
        assert_eq!(api.config.app_id(), "test_app_id");
    }

    #[test]
    fn test_apaas_v1_top_chain_access() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let api = ApaasV1::new(std::sync::Arc::new(config));
        // 顶层 6 个浅 service 叶子可达
        let _ = api.app().list();
        let _ = api.approval_instance().list();
        let _ = api.approval_task().agree();
        let _ = api.approval_task().transfer("task_1", "user_2");
        let _ = api.user_task().query();
        let _ = api.user_task().cc("task_1");
        let _ = api.seat_activity().list();
        let _ = api.seat_assignment().list();
        // application/workspace 中间级可达（深链在 Task 4/5 补）
        let _ = api.application("ns_x");
        let _ = api.workspace("ws_x");
    }

    #[test]
    fn test_apaas_v1_application_deep_chain_access() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let api = ApaasV1::new(std::sync::Arc::new(config));
        let app = api.application("ns_x");
        // object → record 深链到叶子
        let _ = app.object("obj_y").record().create();
        let _ = app.object("obj_y").record().batch_create();
        let _ = app.object("obj_y").record().query("rec_1");
        // object 直接子（search/oql）
        let _ = app.object("obj_y").search("q");
        let _ = app.object("obj_y").oql_query("select *");
        // role → member
        let _ = app.role("role_a").member().get();
        let _ = app.role("role_a").member().batch_create_authorization();
        // record_permission → member
        let _ = app
            .record_permission("rp_b")
            .member()
            .batch_create_authorization();
        // application 直接子
        let _ = app.environment_variable().query();
        let _ = app.environment_variable().get("var_k");
        let _ = app.function("fn_a").invoke();
        let _ = app.flow("flow_1").execute();
        let _ = app.audit_log().list();
        let _ = app.audit_log().get("log_9");
    }
}
