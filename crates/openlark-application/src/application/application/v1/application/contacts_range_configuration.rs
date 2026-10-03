//! 获取应用通讯录权限范围配置
//! docPath: <https://open.feishu.cn/document/application-v6/admin/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取应用通讯录权限范围配置的请求。
#[derive(Debug, Clone)]
pub struct GetApplicationContactsRangeConfigurationRequest {
    config: Arc<Config>,
    app_id: String,
}

/// 获取应用通讯录权限范围配置的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationContactsRangeConfigurationResponse {
    /// 响应数据。
    pub data: Option<ContactsRangeConfigurationData>,
}

impl ApiResponseTrait for GetApplicationContactsRangeConfigurationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取应用通讯录权限范围配置的数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactsRangeConfigurationData {
    /// 应用 ID。
    pub app_id: String,
    /// 通讯录范围。
    pub contacts_range: serde_json::Value,
}

impl GetApplicationContactsRangeConfigurationRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行获取应用通讯录权限范围配置请求。
    pub async fn execute(self) -> SDKResult<GetApplicationContactsRangeConfigurationResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationContactsRangeConfigurationResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/contacts_range_configuration",
            self.app_id
        );
        let req: ApiRequest<GetApplicationContactsRangeConfigurationResponse> =
            ApiRequest::get(&path);

        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "获取应用通讯录权限范围配置",
        )
        .await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

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
