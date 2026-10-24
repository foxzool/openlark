//! 自定义字段选项模块

pub mod create;
/// 更新接口。
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

/// CustomFieldOption：自定义字段选项资源
#[derive(Clone)]
pub struct CustomFieldOptionResource {
    config: Arc<Config>,
    custom_field_guid: String,
}

impl CustomFieldOptionResource {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, custom_field_guid: impl Into<String>) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
        }
    }

    /// 创建自定义字段选项
    pub fn create(&self) -> create::CreateCustomFieldOptionRequest {
        create::CreateCustomFieldOptionRequest::new(
            self.config.clone(),
            self.custom_field_guid.clone(),
        )
    }

    /// 更新自定义字段选项
    pub fn patch(&self, option_guid: impl Into<String>) -> patch::UpdateCustomFieldOptionRequest {
        patch::UpdateCustomFieldOptionRequest::new(
            self.config.clone(),
            self.custom_field_guid.clone(),
            option_guid.into(),
        )
    }
}

// 重新导出请求类型
pub use create::CreateCustomFieldOptionRequest;
pub use patch::UpdateCustomFieldOptionRequest;

// 重新导出响应类型
pub use create::{CreateCustomFieldOptionBody, CreateCustomFieldOptionResponse, CustomFieldOption};
pub use patch::{
    InputCustomFieldOption, UpdateCustomFieldOptionBody, UpdateCustomFieldOptionResponse,
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
    fn test_custom_field_option_resource_new() {
        let config = create_test_config();
        let resource = CustomFieldOptionResource::new(config, "field_guid_123");
        assert_eq!(resource.custom_field_guid, "field_guid_123");
    }

    #[test]
    fn test_custom_field_option_create() {
        let config = create_test_config();
        let resource = CustomFieldOptionResource::new(config, "field_guid_123");
        let _request = resource.create();
    }

    #[test]
    fn test_custom_field_option_patch() {
        let config = create_test_config();
        let resource = CustomFieldOptionResource::new(config, "field_guid_123");
        let _request = resource.patch("option_456");
    }
}
