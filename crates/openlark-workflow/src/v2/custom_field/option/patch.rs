//! 更新自定义字段选项
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/custom_field-option/patch>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::custom_field::option::create::CustomFieldOption;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新自定义字段选项请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateCustomFieldOptionBody {
    /// 选项名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 选项颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// 更新自定义字段选项响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCustomFieldOptionResponse {
    /// 选项信息
    pub option: CustomFieldOption,
}

/// 更新自定义字段选项请求
#[derive(Debug, Clone)]
pub struct UpdateCustomFieldOptionRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 自定义字段 GUID
    custom_field_guid: String,
    /// 选项 GUID
    option_guid: String,
    /// 请求体
    body: UpdateCustomFieldOptionBody,
}

impl UpdateCustomFieldOptionRequest {
    pub fn new(
        config: Arc<Config>,
        custom_field_guid: impl Into<String>,
        option_guid: impl Into<String>,
    ) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
            option_guid: option_guid.into(),
            body: UpdateCustomFieldOptionBody::default(),
        }
    }

    /// 设置选项名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    /// 设置选项颜色
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.body.color = Some(color.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateCustomFieldOptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateCustomFieldOptionResponse> {
        // 验证必填字段
        validate_required!(self.custom_field_guid.trim(), "自定义字段GUID不能为空");
        validate_required!(self.option_guid.trim(), "选项GUID不能为空");

        let api_endpoint = TaskApiV2::CustomFieldOptionUpdate(
            self.custom_field_guid.clone(),
            self.option_guid.clone(),
        );
        let mut request =
            ApiRequest::<UpdateCustomFieldOptionResponse>::patch(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新自定义字段选项")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新自定义字段选项")
    }
}

impl ApiResponseTrait for UpdateCustomFieldOptionResponse {
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
    fn test_update_custom_field_option_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = UpdateCustomFieldOptionRequest::new(config, "custom_field_123", "option_456")
            .name("中优先级")
            .color("#00FF00");

        assert_eq!(request.custom_field_guid, "custom_field_123");
        assert_eq!(request.option_guid, "option_456");
        assert_eq!(request.body.name, Some("中优先级".to_string()));
        assert_eq!(request.body.color, Some("#00FF00".to_string()));
    }

    #[test]
    fn test_custom_field_option_update_api_v2_url() {
        let endpoint = TaskApiV2::CustomFieldOptionUpdate(
            "custom_field_123".to_string(),
            "option_456".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/custom_fields/custom_field_123/options/option_456"
        );
    }
}
