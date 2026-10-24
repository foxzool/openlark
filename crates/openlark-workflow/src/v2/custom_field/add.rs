//! 将自定义字段加入资源
//!
//! docPath: <https://open.feishu.cn/document/task-v2/custom_field/add>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 将自定义字段加入资源请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddCustomFieldBody {
    /// 资源类型（目前仅支持 "tasklist"）
    pub resource_type: String,
    /// 资源 ID（resource_type 为 tasklist 时传任务清单 GUID）
    pub resource_id: String,
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
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, custom_field_guid: impl Into<String>) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
            body: AddCustomFieldBody::default(),
        }
    }

    /// 设置资源类型（目前仅支持 "tasklist"）
    pub fn resource_type(mut self, resource_type: impl Into<String>) -> Self {
        self.body.resource_type = resource_type.into();
        self
    }

    /// 设置资源 ID（resource_type 为 tasklist 时传任务清单 GUID）
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.body.resource_id = resource_id.into();
        self
    }

    /// 设置任务清单 GUID（resource_type 固定为 "tasklist" 的便捷写法）
    pub fn tasklist_guid(mut self, tasklist_guid: impl Into<String>) -> Self {
        self.body.resource_type = "tasklist".to_string();
        self.body.resource_id = tasklist_guid.into();
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
        validate_required!(self.body.resource_type.trim(), "资源类型不能为空");
        validate_required!(self.body.resource_id.trim(), "资源 ID 不能为空");

        let api_endpoint = TaskApiV2::CustomFieldAdd(self.custom_field_guid);
        let mut request = ApiRequest::<AddCustomFieldResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "将自定义字段加入资源")?);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "将自定义字段加入资源",
        )
        .await
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

    #[test]
    fn test_add_custom_field_body_official_shape() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = AddCustomFieldRequest::new(config, "field_guid")
            .tasklist_guid("0110a4bd-f24b-4a93-8c1a-1732b94f9593");

        assert_eq!(request.body.resource_type, "tasklist");
        assert_eq!(
            request.body.resource_id,
            "0110a4bd-f24b-4a93-8c1a-1732b94f9593"
        );

        let json = serde_json::to_value(&request.body).expect("序列化请求体失败");
        assert_eq!(
            json,
            serde_json::json!({
                "resource_type": "tasklist",
                "resource_id": "0110a4bd-f24b-4a93-8c1a-1732b94f9593"
            })
        );
    }
}
