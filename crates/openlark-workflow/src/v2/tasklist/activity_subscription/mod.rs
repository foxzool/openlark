//! 动态订阅模块

pub mod create;
/// 删除接口。
pub mod delete;
/// 获取接口。
pub mod get;
/// 列表接口。
pub mod list;
/// 数据模型。
pub mod models;
/// 更新接口。
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

/// ActivitySubscription：动态订阅资源
#[derive(Clone)]
pub struct ActivitySubscriptionResource {
    config: Arc<Config>,
    tasklist_guid: String,
}

impl ActivitySubscriptionResource {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, tasklist_guid: impl Into<String>) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
        }
    }

    /// 创建动态订阅
    pub fn create(&self) -> create::CreateActivitySubscriptionRequest {
        create::CreateActivitySubscriptionRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
        )
    }

    /// 获取动态订阅
    pub fn get(&self, subscription_guid: impl Into<String>) -> get::GetActivitySubscriptionRequest {
        get::GetActivitySubscriptionRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
            subscription_guid.into(),
        )
    }

    /// 更新动态订阅
    pub fn patch(
        &self,
        subscription_guid: impl Into<String>,
    ) -> patch::UpdateActivitySubscriptionRequest {
        patch::UpdateActivitySubscriptionRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
            subscription_guid.into(),
        )
    }

    /// 删除动态订阅
    pub fn delete(
        &self,
        subscription_guid: impl Into<String>,
    ) -> delete::DeleteActivitySubscriptionRequest {
        delete::DeleteActivitySubscriptionRequest::new(
            self.config.clone(),
            self.tasklist_guid.clone(),
            subscription_guid.into(),
        )
    }

    /// 列取动态订阅
    pub fn list(&self) -> list::ListActivitySubscriptionsRequest {
        list::ListActivitySubscriptionsRequest::new(self.config.clone(), self.tasklist_guid.clone())
    }
}

// 重新导出请求类型
pub use create::CreateActivitySubscriptionRequest;
pub use delete::DeleteActivitySubscriptionRequest;
pub use get::GetActivitySubscriptionRequest;
pub use list::ListActivitySubscriptionsRequest;
pub use patch::UpdateActivitySubscriptionRequest;

// 重新导出响应类型
pub use models::{
    ActivitySubscription, ActivitySubscriptionInput, CreateActivitySubscriptionBody,
    CreateActivitySubscriptionResponse, DeleteActivitySubscriptionResponse,
    GetActivitySubscriptionResponse, ListActivitySubscriptionsResponse,
    UpdateActivitySubscriptionBody, UpdateActivitySubscriptionResponse,
};

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_config() -> Arc<Config> {
        Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        )
    }

    #[test]
    fn test_activity_subscription_resource_new() {
        let config = create_test_config();
        let resource = ActivitySubscriptionResource::new(config, "tasklist_123");
        assert_eq!(resource.tasklist_guid, "tasklist_123");
    }

    #[test]
    fn test_activity_subscription_create() {
        let config = create_test_config();
        let resource = ActivitySubscriptionResource::new(config, "tasklist_123");
        let _request = resource.create();
    }

    #[test]
    fn test_activity_subscription_get() {
        let config = create_test_config();
        let resource = ActivitySubscriptionResource::new(config, "tasklist_123");
        let _request = resource.get("subscription_456");
    }

    #[test]
    fn test_activity_subscription_patch() {
        let config = create_test_config();
        let resource = ActivitySubscriptionResource::new(config, "tasklist_123");
        let _request = resource.patch("subscription_456");
    }

    #[test]
    fn test_activity_subscription_delete() {
        let config = create_test_config();
        let resource = ActivitySubscriptionResource::new(config, "tasklist_123");
        let _request = resource.delete("subscription_456");
    }

    #[test]
    fn test_activity_subscription_list() {
        let config = create_test_config();
        let resource = ActivitySubscriptionResource::new(config, "tasklist_123");
        let _request = resource.list();
    }
}
