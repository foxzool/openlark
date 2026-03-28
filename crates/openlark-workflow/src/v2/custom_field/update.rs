//! 更新自定义字段
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/custom_field/update>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::custom_field::models::{UpdateCustomFieldBody, UpdateCustomFieldResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 更新自定义字段请求
#[derive(Debug, Clone)]
pub struct UpdateCustomFieldRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 字段 GUID
    field_guid: String,
    /// 请求体
    body: UpdateCustomFieldBody,
}

impl UpdateCustomFieldRequest {
    pub fn new(config: Arc<Config>, tasklist_guid: String, field_guid: String) -> Self {
        Self {
            config,
            tasklist_guid,
            field_guid,
            body: UpdateCustomFieldBody::default(),
        }
    }

    /// 设置字段名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    /// 设置字段配置
    pub fn config(mut self, config: crate::v2::custom_field::models::CustomFieldConfig) -> Self {
        self.body.config = Some(config);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateCustomFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateCustomFieldResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");
        validate_required!(self.field_guid.trim(), "字段GUID不能为空");

        let api_endpoint =
            TaskApiV2::CustomFieldUpdate(self.tasklist_guid.clone(), self.field_guid.clone());
        let mut request = ApiRequest::<UpdateCustomFieldResponse>::put(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新自定义字段")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新自定义字段")
    }
}

impl ApiResponseTrait for UpdateCustomFieldResponse {
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
    fn test_update_custom_field_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = UpdateCustomFieldRequest::new(
            config,
            "tasklist_123".to_string(),
            "field_456".to_string(),
        )
        .name("更新的字段名");

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.field_guid, "field_456");
        assert_eq!(request.body.name, Some("更新的字段名".to_string()));
    }
}
