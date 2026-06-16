//! 设备管理 API
//!
//! 提供门禁设备列表查询功能。

use openlark_core::error::api_error;
use std::sync::Arc;

/// 设备管理服务
#[derive(Debug)]
pub struct DevicesService {
    config: Arc<crate::models::SecurityConfig>,
}

impl DevicesService {
    /// 创建新的设备管理服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self { config }
    }

    /// 获取设备列表
    pub fn list(&self) -> ListDevicesBuilder {
        ListDevicesBuilder {
            config: self.config.clone(),
            page_size: Some(20),
            page_token: None,
            device_type: None,
            status: None,
            online: None,
        }
    }
}

/// 获取设备列表构建器
#[derive(Debug)]
pub struct ListDevicesBuilder {
    config: Arc<crate::models::SecurityConfig>,
    page_size: Option<i32>,
    page_token: Option<String>,
    device_type: Option<crate::models::acs::DeviceType>,
    status: Option<crate::models::acs::DeviceStatus>,
    online: Option<bool>,
}

impl ListDevicesBuilder {
    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置设备类型过滤
    pub fn device_type(mut self, device_type: crate::models::acs::DeviceType) -> Self {
        self.device_type = Some(device_type);
        self
    }

    /// 设置设备状态过滤
    pub fn status(mut self, status: crate::models::acs::DeviceStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置在线状态过滤
    pub fn online(mut self, online: bool) -> Self {
        self.online = Some(online);
        self
    }

    /// 发送请求获取设备列表
    pub async fn send(
        self,
    ) -> crate::SecurityResult<crate::models::PageResponse<crate::models::acs::DeviceInfo>> {
        let mut url = format!("{}/open-apis/acs/v1/devices", self.config.base_url);
        let mut query_params = Vec::new();

        if let Some(page_size) = self.page_size {
            query_params.push(format!("page_size={page_size}"));
        }
        if let Some(page_token) = &self.page_token {
            query_params.push(format!("page_token={page_token}"));
        }
        if let Some(device_type) = &self.device_type {
            let type_str = match device_type {
                crate::models::acs::DeviceType::CardReader => "card_reader",
                crate::models::acs::DeviceType::FaceRecognition => "face_recognition",
                crate::models::acs::DeviceType::FingerprintReader => "fingerprint_reader",
                crate::models::acs::DeviceType::DoorLock => "door_lock",
                crate::models::acs::DeviceType::Turnstile => "turnstile",
                crate::models::acs::DeviceType::Other(s) => s,
            };
            query_params.push(format!("device_type={type_str}"));
        }
        if let Some(status) = &self.status {
            let status_str = match status {
                crate::models::acs::DeviceStatus::Normal => "normal",
                crate::models::acs::DeviceStatus::Offline => "offline",
                crate::models::acs::DeviceStatus::Fault => "fault",
                crate::models::acs::DeviceStatus::Maintenance => "maintenance",
                crate::models::acs::DeviceStatus::Disabled => "disabled",
            };
            query_params.push(format!("status={status_str}"));
        }
        if let Some(online) = self.online {
            query_params.push(format!("online={}", if online { "true" } else { "false" }));
        }

        if !query_params.is_empty() {
            url.push_str(&format!("?{}", query_params.join("&")));
        }

        let response = reqwest::Client::new()
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<
                crate::models::PageResponse<crate::models::acs::DeviceInfo>,
            > = response.json().await?;
            match api_response.data {
                Some(devices) => Ok(devices),
                None => Err(api_error(
                    api_response.code as u16,
                    "/acs/v1/devices",
                    &api_response.msg,
                    None,
                )),
            }
        } else {
            Err(api_error(
                response.status().as_u16(),
                "/acs/v1/devices",
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
    fn test_devices_service_creation() {
        let config = create_test_config();
        let service = DevicesService::new(config.clone());
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_list_devices_builder_defaults() {
        let config = create_test_config();
        let service = DevicesService::new(config);
        let builder = service.list();
        assert_eq!(builder.page_size, Some(20));
        assert_eq!(builder.page_token, None);
        assert_eq!(builder.device_type, None);
        assert_eq!(builder.status, None);
        assert_eq!(builder.online, None);
    }

    #[test]
    fn test_list_devices_builder_with_pagination() {
        let config = create_test_config();
        let service = DevicesService::new(config);
        let builder = service.list().page_size(50).page_token("token_123");

        assert_eq!(builder.page_size, Some(50));
        assert_eq!(builder.page_token, Some("token_123".to_string()));
    }

    #[test]
    fn test_list_devices_builder_with_device_type() {
        let config = create_test_config();
        let service = DevicesService::new(config);

        // 测试不同的设备类型
        let builder_face = service
            .list()
            .device_type(crate::models::acs::DeviceType::FaceRecognition);
        assert!(builder_face.device_type.is_some());

        let builder_card = service
            .list()
            .device_type(crate::models::acs::DeviceType::CardReader);
        assert!(builder_card.device_type.is_some());

        let builder_lock = service
            .list()
            .device_type(crate::models::acs::DeviceType::DoorLock);
        assert!(builder_lock.device_type.is_some());
    }

    #[test]
    fn test_list_devices_builder_with_status() {
        let config = create_test_config();
        let service = DevicesService::new(config);

        // 测试不同的设备状态
        let builder_normal = service
            .list()
            .status(crate::models::acs::DeviceStatus::Normal);
        assert!(builder_normal.status.is_some());

        let builder_offline = service
            .list()
            .status(crate::models::acs::DeviceStatus::Offline);
        assert!(builder_offline.status.is_some());

        let builder_fault = service
            .list()
            .status(crate::models::acs::DeviceStatus::Fault);
        assert!(builder_fault.status.is_some());
    }

    #[test]
    fn test_list_devices_builder_with_online_status() {
        let config = create_test_config();
        let service = DevicesService::new(config);

        let builder_online = service.list().online(true);
        assert_eq!(builder_online.online, Some(true));

        let builder_offline = service.list().online(false);
        assert_eq!(builder_offline.online, Some(false));
    }

    #[test]
    fn test_list_devices_builder_full_chaining() {
        let config = create_test_config();
        let service = DevicesService::new(config);
        let builder = service
            .list()
            .page_size(100)
            .page_token("token_456")
            .device_type(crate::models::acs::DeviceType::FingerprintReader)
            .status(crate::models::acs::DeviceStatus::Normal)
            .online(true);

        assert_eq!(builder.page_size, Some(100));
        assert_eq!(builder.page_token, Some("token_456".to_string()));
        assert!(builder.device_type.is_some());
        assert!(builder.status.is_some());
        assert_eq!(builder.online, Some(true));
    }

    #[test]
    fn test_device_type_other_variant() {
        let other_type = crate::models::acs::DeviceType::Other("custom_device".to_string());
        if let crate::models::acs::DeviceType::Other(name) = other_type {
            assert_eq!(name, "custom_device");
        } else {
            panic!("Expected Other variant");
        }
    }
}
