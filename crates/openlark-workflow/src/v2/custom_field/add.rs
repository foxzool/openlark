//! 将自定义字段加入资源
//!
//! docPath: <https://open.feishu.cn/document/task-v2/custom_field/add>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 将自定义字段加入资源请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddCustomFieldBody {
    /// 任务清单 GUID
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tasklist_guid: String,
}

/// 将自定义字段加入资源响应
#[derive(Debug, Clone, Deserialize)]
pub struct AddCustomFieldResponse {
    /// 是否成功
    pub success: bool,
}

/// 将自定义字段加入资源请求
#[derive(Debug, Clone)]
pub struct AddCustomFieldRequest {
    config: Arc<Config>,
    custom_field_guid: String,
    body: AddCustomFieldBody,
}

impl AddCustomFieldRequest {
    pub fn new(config: Arc<Config>, custom_field_guid: impl Into<String>) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
            body: AddCustomFieldBody::default(),
        }
    }

    /// 设置任务清单 GUID
    pub fn tasklist_guid(mut self, tasklist_guid: impl Into<String>) -> Self {
        self.body.tasklist_guid = tasklist_guid.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AddCustomFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<AddCustomFieldResponse> {
        validate_required!(self.custom_field_guid.trim(), "自定义字段 GUID 不能为空");

        let api_endpoint = TaskApiV2::CustomFieldAdd(self.custom_field_guid);
        let mut request = ApiRequest::<AddCustomFieldResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "将自定义字段加入资源")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "将自定义字段加入资源")
    }
}

impl ApiResponseTrait for AddCustomFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_add_custom_field_url() {
        let endpoint = TaskApiV2::CustomFieldAdd("test_guid".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/custom_fields/test_guid/add"
        );
    }
}
