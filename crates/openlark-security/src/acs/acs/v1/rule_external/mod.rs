//! 外部权限规则管理 API（门面）
//!
//! [`RuleExternalService`] 是轻量门面，返回 `create|get|delete|device_bind` 子模块
//! 下的端点构建器。
//!
//! 字段语义已按飞书文档核对（见各端点文件 docPath）。

use openlark_core::config::Config;

// 端点子模块
pub mod create;
pub mod delete;
pub mod device_bind;
pub mod get;

/// 权限规则管理服务
///
/// 不直接发请求，仅返回端点构建器。
#[derive(Debug, Clone)]
pub struct RuleExternalService {
    config: Config,
}

impl RuleExternalService {
    /// 创建新的权限规则管理服务实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建或更新权限组（`rule_id` 为查询参数，body 为 `{"rule": ...}` 包装）。
    pub fn create(&self, rule_id: impl Into<String>) -> create::CreateRuleExternalRequest {
        create::CreateRuleExternalRequest::new(self.config.clone(), rule_id)
    }

    /// 获取权限组信息（查询参数 `device_id`）。
    pub fn get(&self, device_id: impl Into<String>) -> get::GetRuleExternalRequest {
        get::GetRuleExternalRequest::new(self.config.clone(), device_id)
    }

    /// 删除权限组（查询参数 `rule_id`，无 body）。
    pub fn delete(&self, rule_id: impl Into<String>) -> delete::DeleteRuleExternalRequest {
        delete::DeleteRuleExternalRequest::new(self.config.clone(), rule_id)
    }

    /// 设备绑定权限组（flat body `{"device_id", "rule_ids"}`）。
    pub fn device_bind(&self) -> device_bind::BindDeviceToRuleRequest {
        device_bind::BindDeviceToRuleRequest::new(self.config.clone())
    }
}
