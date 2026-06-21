//! 访问控制系统 (ACS) - Project
//!
//! 智能门禁访问控制系统，提供用户、设备、权限和访客管理功能。
//!
//! **迁移说明（见 docs/superpowers/specs/2026-06-20-acs-transport-migration-design.md）**：
//! acs 正在从 SecurityConfig + 原始 reqwest 迁移到 openlark_core::Config + Transport。
//! Task 1 仅做边界：`AcsProject`/`AcsV1Service` 切到 `openlark_core::Config`（owned，与
//! communication/auth 等 crate 一致），6 个资源 Service 暂时下线，在 Task 2-5 中逐个
//! 以门面形式恢复（持 `Config`）。

use openlark_core::config::Config;

/// ACS 项目服务
#[derive(Debug)]
pub struct AcsProject {
    config: Config,
    v1: AcsV1Service,
}

impl AcsProject {
    /// 创建新的 ACS 项目实例
    pub fn new(config: Config) -> Self {
        Self {
            v1: AcsV1Service::new(config.clone()),
            config,
        }
    }

    /// 获取 v1 版本服务
    pub fn v1(&self) -> &AcsV1Service {
        &self.v1
    }

    /// 获取配置信息
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// ACS v1 版本服务
///
/// **迁移进行中**：资源 Service 在 Task 2-5 中逐个恢复。当前已恢复全部 6 个：
/// users / user_faces / devices / rule_external / visitors / access_records
/// （+ `client_device`、`face` 便捷方法）。
#[derive(Debug)]
pub struct AcsV1Service {
    users: crate::acs::acs::v1::users::UsersService,
    user_faces: crate::acs::acs::v1::user_faces::UserFacesService,
    devices: crate::acs::acs::v1::devices::DevicesService,
    rule_external: crate::acs::acs::v1::rule_external::RuleExternalService,
    visitors: crate::acs::acs::v1::visitors::VisitorsService,
    access_records: crate::acs::acs::v1::access_records::AccessRecordsService,
    config: Config,
}

impl AcsV1Service {
    /// 创建新的 v1 服务实例
    pub fn new(config: Config) -> Self {
        Self {
            users: crate::acs::acs::v1::users::UsersService::new(config.clone()),
            user_faces: crate::acs::acs::v1::user_faces::UserFacesService::new(config.clone()),
            devices: crate::acs::acs::v1::devices::DevicesService::new(config.clone()),
            rule_external: crate::acs::acs::v1::rule_external::RuleExternalService::new(
                config.clone(),
            ),
            visitors: crate::acs::acs::v1::visitors::VisitorsService::new(config.clone()),
            access_records: crate::acs::acs::v1::access_records::AccessRecordsService::new(
                config.clone(),
            ),
            config,
        }
    }

    /// 获取用户管理服务
    pub fn users(&self) -> &crate::acs::acs::v1::users::UsersService {
        &self.users
    }

    /// 获取人脸管理服务（用户人脸，`/users/{user_id}/face`）
    pub fn user_faces(&self) -> &crate::acs::acs::v1::user_faces::UserFacesService {
        &self.user_faces
    }

    /// 获取设备管理服务
    pub fn devices(&self) -> &crate::acs::acs::v1::devices::DevicesService {
        &self.devices
    }

    /// 获取权限规则管理服务
    pub fn rule_external(&self) -> &crate::acs::acs::v1::rule_external::RuleExternalService {
        &self.rule_external
    }

    /// 获取访客管理服务
    pub fn visitors(&self) -> &crate::acs::acs::v1::visitors::VisitorsService {
        &self.visitors
    }

    /// 获取访问记录服务
    pub fn access_records(&self) -> &crate::acs::acs::v1::access_records::AccessRecordsService {
        &self.access_records
    }

    /// 人脸资源便捷方法（独立的人脸资源 `/acs/v1/faces/{face_id}`，
    /// 区别于用户人脸 `user_faces()`）。
    pub fn face(&self) -> FaceAccessors {
        FaceAccessors {
            config: self.config.clone(),
        }
    }

    /// 获取客户端设备认证信息（便捷方法，直接返回请求构建器）。
    pub fn client_device(
        &self,
        device_id: impl Into<String>,
    ) -> crate::acs::acs::v1::client_device::get::GetClientDeviceRequest {
        crate::acs::acs::v1::client_device::get::GetClientDeviceRequest::new(
            self.config.clone(),
            device_id,
        )
    }
}

// v1 模块
pub mod v1;

/// 人脸资源访问器（独立的人脸资源，`/acs/v1/faces`）。
///
/// 由 [`AcsV1Service::face`] 返回，提供 `get`/`create`/`delete` 端点构建器。
#[derive(Debug, Clone)]
pub struct FaceAccessors {
    config: Config,
}

impl FaceAccessors {
    /// 获取人脸信息。
    pub fn get(
        &self,
        face_id: impl Into<String>,
    ) -> crate::acs::acs::v1::face::get::FaceGetRequest {
        crate::acs::acs::v1::face::get::FaceGetRequest::new(self.config.clone(), face_id)
    }

    /// 创建人脸。
    pub fn create(&self) -> crate::acs::acs::v1::face::create::FaceCreateRequest {
        crate::acs::acs::v1::face::create::FaceCreateRequest::new(self.config.clone())
    }

    /// 删除人脸。
    pub fn delete(
        &self,
        face_id: impl Into<String>,
    ) -> crate::acs::acs::v1::face::delete::FaceDeleteRequest {
        crate::acs::acs::v1::face::delete::FaceDeleteRequest::new(self.config.clone(), face_id)
    }
}
