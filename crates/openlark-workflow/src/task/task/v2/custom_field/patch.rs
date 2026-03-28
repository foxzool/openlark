//! 更新自定义字段（PATCH）
//!
//! docPath: <https://open.feishu.cn/document/task-v2/custom_field/patch>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新自定义字段请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct PatchCustomFieldBody {
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 字段配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    /// 要更新的字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
}

/// 更新自定义字段响应
#[derive(Debug, Clone, Deserialize)]
pub struct PatchCustomFieldResponse {
    /// 字段 GUID
    pub custom_field_guid: String,
    /// 字段名称
    pub name: String,
    /// 字段配置
    pub config: serde_json::Value,
    /// 创建时间
    pub create_time: i64,
    /// 更新时间
    pub update_time: i64,
}

/// 更新自定义字段请求
#[derive(Debug, Clone)]
pub struct PatchCustomFieldRequest {
    config: Arc<Config>,
    custom_field_guid: String,
    body: PatchCustomFieldBody,
}

impl PatchCustomFieldRequest {
    pub fn new(config: Arc<Config>, custom_field_guid: impl Into<String>) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
            body: PatchCustomFieldBody::default(),
        }
    }

    /// 设置字段名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    /// 设置字段配置
    pub fn config(mut self, config: serde_json::Value) -> Self {
        self.body.config = Some(config);
        self
    }

    /// 设置要更新的字段列表
    pub fn update_fields(mut self, update_fields: Vec<String>) -> Self {
        self.body.update_fields = Some(update_fields);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchCustomFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchCustomFieldResponse> {
        validate_required!(
            self.custom_field_guid.trim(),
            "自定义字段 GUID 不能为空"
        );

        let api_endpoint = TaskApiV2::CustomFieldUpdate(
            self.custom_field_guid.clone(),
            self.custom_field_guid,
        );
        let mut request = ApiRequest::<PatchCustomFieldResponse>::patch(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新自定义字段")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新自定义字段")
    }
}

impl ApiResponseTrait for PatchCustomFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_patch_custom_field_url() {
        let endpoint = TaskApiV2::CustomFieldUpdate(
            "field_123".to_string(),
            "field_123".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasklists/field_123/custom_fields/field_123"
        );
    }
}
