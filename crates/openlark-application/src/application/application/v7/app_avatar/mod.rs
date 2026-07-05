/// upload 模块。
pub mod upload;

use openlark_core::config::Config;
use std::sync::Arc;

/// AppAvatar：v7 app_avatar 资源（上传应用头像）。
#[derive(Clone)]
pub struct AppAvatar {
    config: Arc<Config>,
}

impl AppAvatar {
    /// 创建新的 AppAvatar 资源实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 上传应用头像。
    pub fn upload(&self) -> upload::create::AppAvatarUploadCreateRequest {
        upload::create::AppAvatarUploadCreateRequest::new(self.config.clone())
    }
}
