//! 访问记录 API（门面）
//!
//! [`AccessRecordsService`] 是轻量门面，返回 `super::access_record::*` 下的端点构建器。

use openlark_core::config::Config;

/// 访问记录服务
///
/// 不直接发请求，仅返回端点构建器。
#[derive(Debug, Clone)]
pub struct AccessRecordsService {
    config: Config,
}

impl AccessRecordsService {
    /// 创建新的访问记录服务实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取门禁记录列表。
    pub fn list(&self) -> super::access_record::list::ListAccessRecordsRequest {
        super::access_record::list::ListAccessRecordsRequest::new(self.config.clone())
    }

    /// 下载开门时的人脸识别照片。
    pub fn get_access_photo(
        &self,
        access_record_id: impl Into<String>,
    ) -> super::access_record::access_photo::get::GetAccessPhotoRequest {
        super::access_record::access_photo::get::GetAccessPhotoRequest::new(
            self.config.clone(),
            access_record_id,
        )
    }
}
