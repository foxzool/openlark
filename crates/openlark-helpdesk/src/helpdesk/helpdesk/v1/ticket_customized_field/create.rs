//! 创建工单自定义字段
//!
//! 创建工单自定义字段。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket_customized_field/create

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

/// 创建工单自定义字段请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTicketCustomizedFieldBody {
    /// 字段名称
    pub name: String,
    /// 字段类型
    pub field_type: String,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl CreateTicketCustomizedFieldBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.name, "name is required");
        validate_required!(self.field_type, "field_type is required");
        Ok(())
    }
}

/// 创建工单自定义字段响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketCustomizedFieldResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateTicketCustomizedFieldResult>,
}

impl openlark_core::api::ApiResponseTrait for CreateTicketCustomizedFieldResponse {}

/// 创建工单自定义字段结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketCustomizedFieldResult {
    /// 字段ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 创建工单自定义字段请求
#[derive(Debug, Clone)]
pub struct CreateTicketCustomizedFieldRequest {
    config: Arc<Config>,
}

impl CreateTicketCustomizedFieldRequest {
    /// 创建新的创建工单自定义字段请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行创建工单自定义字段请求
    pub async fn execute(self, body: CreateTicketCustomizedFieldBody) -> SDKResult<CreateTicketCustomizedFieldResponse> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行创建工单自定义字段请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: CreateTicketCustomizedFieldBody,
        option: RequestOption,
    ) -> SDKResult<CreateTicketCustomizedFieldResponse> {
        body.validate()?;

        let req: ApiRequest<CreateTicketCustomizedFieldResponse> =
            ApiRequest::post(HelpdeskApiV1::TicketCustomizedFieldCreate.to_url())
                .body(serialize_params(&body, "创建工单自定义字段")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建工单自定义字段")
    }
}

/// 创建工单自定义字段请求构建器
#[derive(Debug, Clone)]
pub struct CreateTicketCustomizedFieldRequestBuilder {
    config: Arc<Config>,
    name: Option<String>,
    field_type: Option<String>,
    required: Option<bool>,
}

impl CreateTicketCustomizedFieldRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            name: None,
            field_type: None,
            required: None,
        }
    }

    /// 设置字段名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置字段类型
    pub fn field_type(mut self, field_type: impl Into<String>) -> Self {
        self.field_type = Some(field_type.into());
        self
    }

    /// 设置是否必填
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> Result<CreateTicketCustomizedFieldBody, String> {
        let name = self.name.clone().ok_or("name is required")?;
        let field_type = self.field_type.clone().ok_or("field_type is required")?;

        Ok(CreateTicketCustomizedFieldBody {
            name,
            field_type,
            required: self.required,
        })
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CreateTicketCustomizedFieldResponse> {
        let body = self.body().map_err(|reason| {
            openlark_core::error::validation_error("body", reason)
        })?;
        let request = CreateTicketCustomizedFieldRequest::new(self.config.clone());
        request.execute(body).await
    }
}

/// 执行创建工单自定义字段
pub async fn create_ticket_customized_field(
    config: &Config,
    body: CreateTicketCustomizedFieldBody,
) -> SDKResult<CreateTicketCustomizedFieldResponse> {
    create_ticket_customized_field_with_options(config, body, RequestOption::default()).await
}

/// 执行创建工单自定义字段（支持自定义选项）
pub async fn create_ticket_customized_field_with_options(
    config: &Config,
    body: CreateTicketCustomizedFieldBody,
    option: RequestOption,
) -> SDKResult<CreateTicketCustomizedFieldResponse> {
    body.validate()?;

    let req: ApiRequest<CreateTicketCustomizedFieldResponse> =
        ApiRequest::post(HelpdeskApiV1::TicketCustomizedFieldCreate.to_url())
            .body(serialize_params(&body, "创建工单自定义字段")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "创建工单自定义字段")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid() {
        let body = CreateTicketCustomizedFieldBody {
            name: "自定义字段".to_string(),
            field_type: "text".to_string(),
            required: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_name() {
        let body = CreateTicketCustomizedFieldBody {
            name: "".to_string(),
            field_type: "text".to_string(),
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
        let builder = CreateTicketCustomizedFieldRequestBuilder::new(Arc::new(config));

        assert!(builder.name.is_none());
    }
}
