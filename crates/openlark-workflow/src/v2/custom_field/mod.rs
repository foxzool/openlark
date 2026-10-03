/// add 模块。
pub mod add;
/// 创建接口。
pub mod create;
/// 删除接口。
pub mod delete;
/// 获取接口。
pub mod get;
/// 列表接口。
pub mod list;
/// 数据模型。
pub mod models;
/// 自定义字段选项模块。
pub mod option;
/// remove 模块。
pub mod remove;
/// 更新接口。
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// CustomField：自定义字段资源
#[derive(Clone)]
pub struct CustomField {
    config: Arc<Config>,
}

impl CustomField {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建新建请求。
    pub fn create(&self, resource_id: impl Into<String>) -> create::CreateCustomFieldRequest {
        create::CreateCustomFieldRequest::new(self.config.clone(), resource_id.into())
    }

    /// 创建获取详情请求。
    pub fn get(&self, field_guid: impl Into<String>) -> get::GetCustomFieldRequest {
        get::GetCustomFieldRequest::new(self.config.clone(), field_guid.into())
    }

    /// 创建更新请求。
    pub fn update(&self, field_guid: impl Into<String>) -> update::UpdateCustomFieldRequest {
        update::UpdateCustomFieldRequest::new(self.config.clone(), field_guid.into())
    }

    /// 创建删除请求。
    pub fn delete(&self, field_guid: impl Into<String>) -> delete::DeleteCustomFieldRequest {
        delete::DeleteCustomFieldRequest::new(self.config.clone(), field_guid.into())
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::ListCustomFieldsRequest {
        list::ListCustomFieldsRequest::new(self.config.clone())
    }

    /// 获取自定义字段选项资源（不需要 tasklist_guid）
    pub fn option(
        &self,
        custom_field_guid: impl Into<String>,
    ) -> option::CustomFieldOptionResource {
        option::CustomFieldOptionResource::new(self.config.clone(), custom_field_guid.into())
    }
}

// 重新导出请求类型
pub use add::AddCustomFieldRequest;
pub use create::CreateCustomFieldRequest;
pub use delete::DeleteCustomFieldRequest;
pub use get::GetCustomFieldRequest;
pub use list::ListCustomFieldsRequest;
pub use remove::RemoveCustomFieldRequest;
pub use update::UpdateCustomFieldRequest;

// 重新导出响应类型
pub use models::{
    CreateCustomFieldBody, CreateCustomFieldResponse, CustomFieldConfig, CustomFieldItem,
    CustomFieldType, DeleteCustomFieldResponse, GetCustomFieldResponse, ListCustomFieldsResponse,
    UpdateCustomFieldBody, UpdateCustomFieldResponse,
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
    fn test_custom_field_new() {
        let config = create_test_config();
        let _field = CustomField::new(config);
    }

    #[test]
    fn test_custom_field_create() {
        let config = create_test_config();
        let field = CustomField::new(config);
        let _request = field.create("tasklist_123");
    }

    #[test]
    fn test_custom_field_get() {
        let config = create_test_config();
        let field = CustomField::new(config);
        let _request = field.get("field_456");
    }

    #[test]
    fn test_custom_field_update() {
        let config = create_test_config();
        let field = CustomField::new(config);
        let _request = field.update("field_456");
    }

    #[test]
    fn test_custom_field_delete() {
        let config = create_test_config();
        let field = CustomField::new(config);
        let _request = field.delete("field_456");
    }

    #[test]
    fn test_custom_field_list() {
        let config = create_test_config();
        let field = CustomField::new(config);
        let _request = field.list();
    }

    #[test]
    fn test_custom_field_option() {
        let config = create_test_config();
        let field = CustomField::new(config);
        let _resource = field.option("field_456");
    }
}
