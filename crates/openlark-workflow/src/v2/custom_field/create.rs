//! 创建自定义字段
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/custom_field/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::custom_field::models::{CreateCustomFieldBody, CreateCustomFieldResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 创建自定义字段请求
#[derive(Debug, Clone)]
pub struct CreateCustomFieldRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 请求体
    body: CreateCustomFieldBody,
}

impl CreateCustomFieldRequest {
    pub fn new(config: Arc<Config>, tasklist_guid: String) -> Self {
        Self {
            config,
            tasklist_guid,
            body: CreateCustomFieldBody::default(),
        }
    }

    /// 设置字段名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置字段配置
    pub fn config(mut self, config: crate::v2::custom_field::models::CustomFieldConfig) -> Self {
        self.body.config = config;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateCustomFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateCustomFieldResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");
        validate_required!(self.body.name.trim(), "字段名称不能为空");

        let api_endpoint = TaskApiV2::CustomFieldCreate(self.tasklist_guid.clone());
        let mut request = ApiRequest::<CreateCustomFieldResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建自定义字段")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建自定义字段")
    }
}

impl ApiResponseTrait for CreateCustomFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::sync::Arc;

    use super::*;

    use crate::v2::custom_field::models::{CustomFieldConfig, CustomFieldType};

    #[test]
    fn test_create_custom_field_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let config_value = CustomFieldConfig {
            field_type: CustomFieldType::Text,
            options: None,
        };

        let request = CreateCustomFieldRequest::new(config, "tasklist_123".to_string())
            .name("优先级")
            .config(config_value);

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.body.name, "优先级");
    }
}
