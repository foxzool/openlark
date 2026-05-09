//! 更新指定工单自定义字段
//!
//! 更新指定工单自定义字段的信息。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket_customized_field/patch

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 更新工单自定义字段请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchTicketCustomizedFieldBody {
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl PatchTicketCustomizedFieldBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        if let Some(name) = &self.name {
            validate_required!(name, "name cannot be empty");
        }
        Ok(())
    }
}

/// 更新工单自定义字段响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchTicketCustomizedFieldResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PatchTicketCustomizedFieldResult>,
}

impl openlark_core::api::ApiResponseTrait for PatchTicketCustomizedFieldResponse {}

/// 更新工单自定义字段结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchTicketCustomizedFieldResult {
    /// 字段ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 更新工单自定义字段请求
#[derive(Debug, Clone)]
pub struct PatchTicketCustomizedFieldRequest {
    config: Arc<Config>,
    id: String,
}

impl PatchTicketCustomizedFieldRequest {
    /// 创建新的更新工单自定义字段请求
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行更新工单自定义字段请求
    pub async fn execute(self, body: PatchTicketCustomizedFieldBody) -> SDKResult<PatchTicketCustomizedFieldResponse> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行更新工单自定义字段请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: PatchTicketCustomizedFieldBody,
        option: RequestOption,
    ) -> SDKResult<PatchTicketCustomizedFieldResponse> {
        body.validate()?;

        let req: ApiRequest<PatchTicketCustomizedFieldResponse> =
            ApiRequest::patch(HelpdeskApiV1::TicketCustomizedFieldPatch(self.id.clone()).to_url())
                .body(serialize_params(&body, "更新工单自定义字段")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新工单自定义字段")
    }
}

/// 更新工单自定义字段请求构建器
#[derive(Debug, Clone)]
pub struct PatchTicketCustomizedFieldRequestBuilder {
    config: Arc<Config>,
    id: String,
    name: Option<String>,
    required: Option<bool>,
}

impl PatchTicketCustomizedFieldRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self {
            config,
            id,
            name: None,
            required: None,
        }
    }

    /// 设置字段名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置是否必填
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> PatchTicketCustomizedFieldBody {
        PatchTicketCustomizedFieldBody {
            name: self.name.clone(),
            required: self.required,
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<PatchTicketCustomizedFieldResponse> {
        let body = self.body();
        let request = PatchTicketCustomizedFieldRequest::new(self.config.clone(), self.id.clone());
        request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<PatchTicketCustomizedFieldResponse> {
        let body = self.body();
        let request = PatchTicketCustomizedFieldRequest::new(self.config.clone(), self.id.clone());
        request.execute_with_options(body, option).await
    }
}

/// 执行更新工单自定义字段
pub async fn patch_ticket_customized_field(
    config: &Config,
    id: String,
    body: PatchTicketCustomizedFieldBody,
) -> SDKResult<PatchTicketCustomizedFieldResponse> {
    patch_ticket_customized_field_with_options(config, id, body, RequestOption::default()).await
}

/// 执行更新工单自定义字段（支持自定义选项）
pub async fn patch_ticket_customized_field_with_options(
    config: &Config,
    id: String,
    body: PatchTicketCustomizedFieldBody,
    option: RequestOption,
) -> SDKResult<PatchTicketCustomizedFieldResponse> {
    body.validate()?;

    let req: ApiRequest<PatchTicketCustomizedFieldResponse> =
        ApiRequest::patch(HelpdeskApiV1::TicketCustomizedFieldPatch(id).to_url())
            .body(serialize_params(&body, "更新工单自定义字段")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "更新工单自定义字段")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_empty() {
        let body = PatchTicketCustomizedFieldBody::default();
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = PatchTicketCustomizedFieldBody {
            name: Some("新名称".to_string()),
            required: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_name() {
        let body = PatchTicketCustomizedFieldBody {
            name: Some("".to_string()),
            required: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = PatchTicketCustomizedFieldRequestBuilder::new(Arc::new(config), "field_123".to_string());

        assert_eq!(builder.id, "field_123");
        assert!(builder.name.is_none());
    }
}
