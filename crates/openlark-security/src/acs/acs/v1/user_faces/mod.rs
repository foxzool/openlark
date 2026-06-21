//! 用户人脸管理 API（门面）
//!
//! [`UserFacesService`] 是轻量门面，返回 `super::user::face::*` 下的端点构建器。
//!
//! 注：飞书"上传用户人脸图片"接口在真实场景下走 multipart 上传二进制图片，当前
//! `super::user::face::update::UpdateUserFaceRequest` 接收 JSON body（字段细化/multipart
//! 支持见 spec §9，不在本次 Transport 迁移范围）。

use openlark_core::config::Config;

/// 用户人脸管理服务
///
/// 不直接发请求，仅返回端点构建器。
#[derive(Debug, Clone)]
pub struct UserFacesService {
    config: Config,
}

impl UserFacesService {
    /// 创建新的人脸管理服务实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 下载用户人脸图片。
    pub fn get(&self, user_id: impl Into<String>) -> super::user::face::get::GetUserFaceRequest {
        super::user::face::get::GetUserFaceRequest::new(self.config.clone(), user_id)
    }

    /// 上传用户人脸图片。
    pub fn update(
        &self,
        user_id: impl Into<String>,
    ) -> super::user::face::update::UpdateUserFaceRequest {
        super::user::face::update::UpdateUserFaceRequest::new(self.config.clone(), user_id)
    }
}
