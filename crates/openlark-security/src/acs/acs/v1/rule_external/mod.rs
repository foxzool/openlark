//! 外部权限规则管理 API
//!
//! 提供权限组的创建、查询、删除和设备绑定功能。

use std::sync::Arc;

use openlark_core::error::api_error;
/// 权限规则管理服务
#[derive(Debug)]
pub struct RuleExternalService {
    config: Arc<crate::models::SecurityConfig>,
}

impl RuleExternalService {
    /// 创建新的权限规则管理服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self { config }
    }

    /// 创建或更新权限组
    pub fn create(&self) -> CreateRuleBuilder {
        CreateRuleBuilder {
            config: self.config.clone(),
            name: String::new(),
            description: None,
            device_ids: Vec::new(),
            user_ids: Vec::new(),
            valid_from: None,
            valid_until: None,
        }
    }

    /// 获取权限组信息
    pub fn get(&self) -> GetRuleBuilder {
        GetRuleBuilder {
            config: self.config.clone(),
            rule_id: String::new(),
        }
    }

    /// 删除权限组
    pub fn delete(&self) -> DeleteRuleBuilder {
        DeleteRuleBuilder {
            config: self.config.clone(),
            rule_id: String::new(),
        }
    }

    /// 设备绑定权限组
    pub fn device_bind(&self) -> DeviceBindRuleBuilder {
        DeviceBindRuleBuilder {
            config: self.config.clone(),
            rule_id: String::new(),
            device_ids: Vec::new(),
            overwrite: None,
        }
    }
}

/// 创建或更新权限组构建器
#[derive(Debug)]
pub struct CreateRuleBuilder {
    config: Arc<crate::models::SecurityConfig>,
    name: String,
    description: Option<String>,
    device_ids: Vec<String>,
    user_ids: Vec<String>,
    valid_from: Option<crate::models::Timestamp>,
    valid_until: Option<crate::models::Timestamp>,
}

impl CreateRuleBuilder {
    /// 设置权限组名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置权限组描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置设备ID列表
    pub fn device_ids(mut self, device_ids: Vec<String>) -> Self {
        self.device_ids = device_ids;
        self
    }

    /// 设置用户ID列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 设置生效开始时间
    pub fn valid_from(mut self, valid_from: crate::models::Timestamp) -> Self {
        self.valid_from = Some(valid_from);
        self
    }

    /// 设置生效结束时间
    pub fn valid_until(mut self, valid_until: crate::models::Timestamp) -> Self {
        self.valid_until = Some(valid_until);
        self
    }

    /// 发送请求创建或更新权限组
    pub async fn send(self) -> crate::SecurityResult<crate::models::acs::PermissionRuleInfo> {
        let url = format!("{}/open-apis/acs/v1/rule_external", self.config.base_url);

        let request_body = crate::models::acs::PermissionRuleRequest {
            name: self.name,
            description: self.description,
            device_ids: Some(self.device_ids),
            user_ids: Some(self.user_ids),
            valid_from: self.valid_from,
            valid_until: self.valid_until,
            extension: None,
        };

        let response = reqwest::Client::new()
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::acs::PermissionRuleInfo> =
                response.json().await?;
            match api_response.data {
                Some(rule) => Ok(rule),
                None => Err(api_error(
                    api_response.code as u16,
                    "/acs/v1/rule_external",
                    &api_response.msg,
                    None,
                )),
            }
        } else {
            Err(api_error(
                response.status().as_u16(),
                "/acs/v1/rule_external",
                format!("HTTP: {}", response.status()),
                None,
            ))
        }
    }
}

/// 获取权限组信息构建器
#[derive(Debug)]
pub struct GetRuleBuilder {
    config: Arc<crate::models::SecurityConfig>,
    rule_id: String,
}

impl GetRuleBuilder {
    /// 设置权限组ID
    pub fn rule_id(mut self, rule_id: impl Into<String>) -> Self {
        self.rule_id = rule_id.into();
        self
    }

    /// 发送请求获取权限组信息
    pub async fn send(self) -> crate::SecurityResult<crate::models::acs::PermissionRuleInfo> {
        let url = format!("{}/open-apis/acs/v1/rule_external", self.config.base_url);

        let mut query_params = Vec::new();
        query_params.push(format!("rule_id={}", self.rule_id));

        let url_with_params = format!("{}?{}", url, query_params.join("&"));

        let response = reqwest::Client::new()
            .get(&url_with_params)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::acs::PermissionRuleInfo> =
                response.json().await?;
            match api_response.data {
                Some(rule) => Ok(rule),
                None => Err(api_error(
                    api_response.code as u16,
                    "/acs/v1/rule_external",
                    &api_response.msg,
                    None,
                )),
            }
        } else {
            Err(api_error(
                response.status().as_u16(),
                "/acs/v1/rule_external",
                format!("HTTP: {}", response.status()),
                None,
            ))
        }
    }
}

/// 删除权限组构建器
#[derive(Debug)]
pub struct DeleteRuleBuilder {
    config: Arc<crate::models::SecurityConfig>,
    rule_id: String,
}

impl DeleteRuleBuilder {
    /// 设置权限组ID
    pub fn rule_id(mut self, rule_id: impl Into<String>) -> Self {
        self.rule_id = rule_id.into();
        self
    }

    /// 发送请求删除权限组
    pub async fn send(self) -> crate::SecurityResult<crate::models::OperationResponse> {
        let url = format!("{}/open-apis/acs/v1/rule_external", self.config.base_url);

        let request_body = serde_json::json!({
            "rule_id": self.rule_id
        });

        let response = reqwest::Client::new()
            .delete(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::OperationResponse> =
                response.json().await?;
            match api_response.data {
                Some(result) => Ok(result),
                None => Err(api_error(
                    api_response.code as u16,
                    "/acs/v1/rule_external",
                    &api_response.msg,
                    None,
                )),
            }
        } else {
            Err(api_error(
                response.status().as_u16(),
                "/acs/v1/rule_external",
                format!("HTTP: {}", response.status()),
                None,
            ))
        }
    }
}

/// 设备绑定权限组构建器
#[derive(Debug)]
pub struct DeviceBindRuleBuilder {
    config: Arc<crate::models::SecurityConfig>,
    rule_id: String,
    device_ids: Vec<String>,
    overwrite: Option<bool>,
}

impl DeviceBindRuleBuilder {
    /// 设置权限组ID
    pub fn rule_id(mut self, rule_id: impl Into<String>) -> Self {
        self.rule_id = rule_id.into();
        self
    }

    /// 设置设备ID列表
    pub fn device_ids(mut self, device_ids: Vec<String>) -> Self {
        self.device_ids = device_ids;
        self
    }

    /// 设置是否覆盖现有绑定
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = Some(overwrite);
        self
    }

    /// 发送请求绑定设备到权限组
    pub async fn send(self) -> crate::SecurityResult<crate::models::OperationResponse> {
        let url = format!(
            "{}/open-apis/acs/v1/rule_external/device_bind",
            self.config.base_url
        );

        let request_body = crate::models::acs::DeviceBindRuleRequest {
            rule_id: self.rule_id,
            device_ids: self.device_ids,
            overwrite: self.overwrite,
        };

        let response = reqwest::Client::new()
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::OperationResponse> =
                response.json().await?;
            match api_response.data {
                Some(result) => Ok(result),
                None => Err(api_error(
                    api_response.code as u16,
                    "/acs/v1/rule_external",
                    &api_response.msg,
                    None,
                )),
            }
        } else {
            Err(api_error(
                response.status().as_u16(),
                "/acs/v1/rule_external",
                format!("HTTP: {}", response.status()),
                None,
            ))
        }
    }
}

/// 获取应用访问令牌的辅助函数
async fn get_app_token(config: &crate::models::SecurityConfig) -> crate::SecurityResult<String> {
    // 使用 SecurityConfig 的 get_app_access_token 方法获取真实的 token
    config.get_app_access_token().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_config() -> Arc<crate::models::SecurityConfig> {
        Arc::new(crate::models::SecurityConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
        })
    }

    #[test]
    fn test_rule_external_service_creation() {
        let config = create_test_config();
        let service = RuleExternalService::new(config);
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_create_rule_builder() {
        let config = create_test_config();
        let service = RuleExternalService::new(config);
        let builder = service
            .create()
            .name("测试权限组")
            .description("这是一个测试权限组")
            .device_ids(vec!["device_1".to_string(), "device_2".to_string()])
            .user_ids(vec!["user_1".to_string()])
            .valid_from(1000000)
            .valid_until(2000000);

        assert_eq!(builder.name, "测试权限组");
        assert_eq!(builder.description, Some("这是一个测试权限组".to_string()));
        assert_eq!(builder.device_ids.len(), 2);
        assert_eq!(builder.user_ids.len(), 1);
        assert_eq!(builder.valid_from, Some(1000000));
        assert_eq!(builder.valid_until, Some(2000000));
    }

    #[test]
    fn test_get_rule_builder() {
        let config = create_test_config();
        let service = RuleExternalService::new(config);
        let builder = service.get().rule_id("rule_123");
        assert_eq!(builder.rule_id, "rule_123");
    }

    #[test]
    fn test_delete_rule_builder() {
        let config = create_test_config();
        let service = RuleExternalService::new(config);
        let builder = service.delete().rule_id("rule_456");
        assert_eq!(builder.rule_id, "rule_456");
    }

    #[test]
    fn test_device_bind_rule_builder() {
        let config = create_test_config();
        let service = RuleExternalService::new(config);
        let builder = service
            .device_bind()
            .rule_id("rule_789")
            .device_ids(vec!["device_1".to_string()])
            .overwrite(true);

        assert_eq!(builder.rule_id, "rule_789");
        assert_eq!(builder.device_ids.len(), 1);
        assert_eq!(builder.overwrite, Some(true));
    }

    #[test]
    fn test_create_rule_builder_defaults() {
        let config = create_test_config();
        let service = RuleExternalService::new(config);
        let builder = service.create();

        assert_eq!(builder.name, String::new());
        assert!(builder.description.is_none());
        assert!(builder.device_ids.is_empty());
        assert!(builder.user_ids.is_empty());
        assert!(builder.valid_from.is_none());
        assert!(builder.valid_until.is_none());
    }
}
