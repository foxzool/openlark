//! 访问控制系统 (ACS) - Project
//!
//! 智能门禁访问控制系统，提供用户、设备、权限和访客管理功能。

use std::sync::Arc;

/// ACS 项目服务
#[derive(Debug)]
pub struct AcsProject {
    config: Arc<crate::models::SecurityConfig>,
    v1: AcsV1Service,
}

impl AcsProject {
    /// 创建新的 ACS 项目实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
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
    pub fn config(&self) -> &crate::models::SecurityConfig {
        &self.config
    }
}

/// ACS v1 版本服务
#[derive(Debug)]
pub struct AcsV1Service {
    #[allow(dead_code)]
    config: Arc<crate::models::SecurityConfig>,
    users: crate::acs::acs::v1::users::UsersService,
    user_faces: crate::acs::acs::v1::user_faces::UserFacesService,
    rule_external: crate::acs::acs::v1::rule_external::RuleExternalService,
    visitors: crate::acs::acs::v1::visitors::VisitorsService,
    devices: crate::acs::acs::v1::devices::DevicesService,
    access_records: crate::acs::acs::v1::access_records::AccessRecordsService,
}

impl AcsV1Service {
    /// 创建新的 v1 服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self {
            users: crate::acs::acs::v1::users::UsersService::new(config.clone()),
            user_faces: crate::acs::acs::v1::user_faces::UserFacesService::new(config.clone()),
            rule_external: crate::acs::acs::v1::rule_external::RuleExternalService::new(
                config.clone(),
            ),
            visitors: crate::acs::acs::v1::visitors::VisitorsService::new(config.clone()),
            devices: crate::acs::acs::v1::devices::DevicesService::new(config.clone()),
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

    /// 获取人脸识别管理服务
    pub fn user_faces(&self) -> &crate::acs::acs::v1::user_faces::UserFacesService {
        &self.user_faces
    }

    /// 获取权限规则管理服务
    pub fn rule_external(&self) -> &crate::acs::acs::v1::rule_external::RuleExternalService {
        &self.rule_external
    }

    /// 获取访客管理服务
    pub fn visitors(&self) -> &crate::acs::acs::v1::visitors::VisitorsService {
        &self.visitors
    }

    /// 获取设备管理服务
    pub fn devices(&self) -> &crate::acs::acs::v1::devices::DevicesService {
        &self.devices
    }

    /// 获取访问记录服务
    pub fn access_records(&self) -> &crate::acs::acs::v1::access_records::AccessRecordsService {
        &self.access_records
    }
}

// v1 模块
pub mod v1;

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
