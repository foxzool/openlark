//! 创建自定义字段选项
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/custom_field-option/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建自定义字段选项请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateCustomFieldOptionBody {
    /// 选项名称
    pub name: String,
    /// 选项颜色（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// 自定义字段选项信息
#[derive(Debug, Clone, Deserialize)]
pub struct CustomFieldOption {
    /// 选项 GUID
    pub option_guid: String,
    /// 选项名称
    pub name: String,
    /// 选项颜色
    #[serde(default)]
    pub color: Option<String>,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 创建自定义字段选项响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCustomFieldOptionResponse {
    /// 选项信息
    pub option: CustomFieldOption,
}

/// 创建自定义字段选项请求
#[derive(Debug, Clone)]
pub struct CreateCustomFieldOptionRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 自定义字段 GUID
    custom_field_guid: String,
    /// 请求体
    body: CreateCustomFieldOptionBody,
}

impl CreateCustomFieldOptionRequest {
    pub fn new(config: Arc<Config>, custom_field_guid: impl Into<String>) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
            body: CreateCustomFieldOptionBody::default(),
        }
    }

    /// 设置选项名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置选项颜色
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.body.color = Some(color.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateCustomFieldOptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateCustomFieldOptionResponse> {
        // 验证必填字段
        validate_required!(self.custom_field_guid.trim(), "自定义字段GUID不能为空");
        validate_required!(self.body.name.trim(), "选项名称不能为空");

        let api_endpoint = TaskApiV2::CustomFieldOptionCreate(self.custom_field_guid.clone());
        let mut request =
            ApiRequest::<CreateCustomFieldOptionResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建自定义字段选项")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建自定义字段选项")
    }
}

impl ApiResponseTrait for CreateCustomFieldOptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_create_custom_field_option_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateCustomFieldOptionRequest::new(config, "custom_field_123")
            .name("高优先级")
            .color("#FF0000");

        assert_eq!(request.custom_field_guid, "custom_field_123");
        assert_eq!(request.body.name, "高优先级");
        assert_eq!(request.body.color, Some("#FF0000".to_string()));
    }

    #[test]
    fn test_custom_field_option_create_api_v2_url() {
        let endpoint = TaskApiV2::CustomFieldOptionCreate("custom_field_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/custom_fields/custom_field_123/options"
        );
    }
}
