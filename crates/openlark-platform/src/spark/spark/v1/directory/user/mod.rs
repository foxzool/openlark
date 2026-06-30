//! Spark directory.user 相关 API

use openlark_core::config::Config;

pub mod id_convert;

/// Spark directory.user 资源服务
#[derive(Debug, Clone)]
pub struct DirectoryUserService {
    config: Config,
}

impl DirectoryUserService {
    /// 创建新的 directory.user 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 转换妙搭和开放平台用户 ID
    pub fn id_convert(&self) -> id_convert::DirectoryUserIdConvertRequestBuilder {
        id_convert::DirectoryUserIdConvertRequestBuilder::new(
            self.config.clone(),
            id_convert::UserIdConvertType::ForceUserIdToFeishuOpenId,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::DirectoryUserService;

    #[test]
    fn test_directory_user_id_convert_access() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let service = DirectoryUserService::new(config);
        let _ = service.id_convert();
    }
}
