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
/// 更新接口。
pub mod patch;
/// 分组任务列表接口。
pub mod tasks;
/// 更新接口。
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// Section：分组资源
#[derive(Clone)]
pub struct Section {
    config: Arc<Config>,
}

impl Section {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建新建请求。
    pub fn create(&self) -> create::CreateSectionRequest {
        create::CreateSectionRequest::new(self.config.clone())
    }

    /// 创建获取详情请求。
    pub fn get(&self, section_guid: impl Into<String>) -> get::GetSectionRequest {
        get::GetSectionRequest::new(self.config.clone(), section_guid.into())
    }

    /// 创建更新请求。
    pub fn update(&self, section_guid: impl Into<String>) -> update::UpdateSectionRequest {
        update::UpdateSectionRequest::new(self.config.clone(), section_guid.into())
    }

    /// 创建删除请求。
    pub fn delete(&self, section_guid: impl Into<String>) -> delete::DeleteSectionRequest {
        delete::DeleteSectionRequest::new(self.config.clone(), section_guid.into())
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::ListSectionsRequest {
        list::ListSectionsRequest::new(self.config.clone())
    }

    /// 创建分组任务列表请求。
    pub fn tasks(&self, section_guid: impl Into<String>) -> tasks::GetSectionTasksRequest {
        tasks::GetSectionTasksRequest::new(self.config.clone(), section_guid.into())
    }
}

// 重新导出请求类型
pub use create::CreateSectionRequest;
pub use delete::DeleteSectionRequest;
pub use get::GetSectionRequest;
pub use list::ListSectionsRequest;
pub use patch::UpdateSectionRequest;
pub use tasks::GetSectionTasksRequest;

// 重新导出响应类型
pub use models::{
    CreateSectionBody, CreateSectionResponse, DeleteSectionResponse, GetSectionResponse,
    ListSectionsResponse, SectionItem, UpdateSectionBody, UpdateSectionResponse,
};
pub use tasks::ListSectionTasksResponse;

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
    fn test_section_new() {
        let config = create_test_config();
        let _section = Section::new(config);
    }

    #[test]
    fn test_section_create() {
        let config = create_test_config();
        let section = Section::new(config);
        let _request = section.create();
    }

    #[test]
    fn test_section_get() {
        let config = create_test_config();
        let section = Section::new(config);
        let _request = section.get("section_456");
    }

    #[test]
    fn test_section_update() {
        let config = create_test_config();
        let section = Section::new(config);
        let _request = section.update("section_456");
    }

    #[test]
    fn test_section_delete() {
        let config = create_test_config();
        let section = Section::new(config);
        let _request = section.delete("section_456");
    }

    #[test]
    fn test_section_list() {
        let config = create_test_config();
        let section = Section::new(config);
        let _request = section.list();
    }
}
