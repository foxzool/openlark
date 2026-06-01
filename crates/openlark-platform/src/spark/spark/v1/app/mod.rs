/// create 模块。
pub mod create;
/// enum 模块。
pub mod r#enum;
/// get_app_visibility 模块。
pub mod get_app_visibility;
/// icon 模块。
pub mod icon;
/// list 模块。
pub mod list;
/// patch 模块。
pub mod patch;
/// sql_commands 模块。
pub mod sql_commands;
/// storage 模块。
pub mod storage;
/// table 模块。
pub mod table;
/// update_app_visibility 模块。
pub mod update_app_visibility;
/// upload_html_code_and_release 模块。
pub mod upload_html_code_and_release;
/// view 模块。
pub mod view;

use crate::PlatformConfig;
use std::sync::Arc;

pub use create::CreateSparkAppRequest;
pub use get_app_visibility::GetSparkAppVisibilityRequest;
pub use icon::UploadSparkAppIconRequest;
pub use list::ListSparkAppsRequest;
pub use patch::PatchSparkAppRequest;
pub use update_app_visibility::UpdateSparkAppVisibilityRequest;
pub use upload_html_code_and_release::UploadHtmlCodeAndReleaseRequest;

/// 妙搭应用资源服务。
#[derive(Debug, Clone)]
pub struct SparkAppService {
    config: Arc<PlatformConfig>,
}

impl SparkAppService {
    /// 创建新的妙搭应用资源服务。
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// 创建妙搭应用。
    pub fn create(&self) -> CreateSparkAppRequest {
        CreateSparkAppRequest::new(self.config.clone())
    }

    /// 批量获取妙搭应用。
    pub fn list(&self) -> ListSparkAppsRequest {
        ListSparkAppsRequest::new(self.config.clone())
    }

    /// 更新妙搭应用信息。
    pub fn patch(&self, app_id: impl Into<String>) -> PatchSparkAppRequest {
        PatchSparkAppRequest::new(self.config.clone(), app_id)
    }

    /// 上传妙搭应用图标。
    pub fn icon(&self) -> UploadSparkAppIconRequest {
        UploadSparkAppIconRequest::new(self.config.clone())
    }

    /// 获取妙搭应用可用范围。
    pub fn get_app_visibility(&self, app_id: impl Into<String>) -> GetSparkAppVisibilityRequest {
        GetSparkAppVisibilityRequest::new(self.config.clone(), app_id)
    }

    /// 更新妙搭应用可用范围。
    pub fn update_app_visibility(
        &self,
        app_id: impl Into<String>,
    ) -> UpdateSparkAppVisibilityRequest {
        UpdateSparkAppVisibilityRequest::new(self.config.clone(), app_id)
    }

    /// 上传 HTML 代码并发布。
    pub fn upload_html_code_and_release(
        &self,
        app_id: impl Into<String>,
    ) -> UploadHtmlCodeAndReleaseRequest {
        UploadHtmlCodeAndReleaseRequest::new(self.config.clone(), app_id)
    }
}
