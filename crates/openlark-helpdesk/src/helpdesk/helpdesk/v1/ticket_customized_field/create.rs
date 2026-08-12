//! 创建工单自定义字段
//!
//! 创建工单自定义字段。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket_customized_field/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;
use crate::helpdesk::helpdesk::v1::ticket_customized_field::models::TicketCustomizedFieldDropdownOptions;

/// 创建工单自定义字段请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTicketCustomizedFieldBody {
    /// 服务台 ID，可省略但必须与请求头中的服务台 ID 一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    /// 字段键名。
    pub key_name: String,
    /// 字段展示名称。
    pub display_name: String,
    /// 字段位置。
    pub position: String,
    /// 字段类型。
    pub field_type: String,
    /// 字段描述。
    pub description: String,
    /// 是否可见。
    pub visible: bool,
    /// 是否必填。
    pub required: bool,
    /// 下拉选项。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown_options: Option<TicketCustomizedFieldDropdownOptions>,
    /// 下拉字段是否允许多选。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown_allow_multiple: Option<bool>,
}

impl CreateTicketCustomizedFieldBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.key_name, "key_name 不能为空");
        validate_required!(self.display_name, "display_name 不能为空");
        validate_required!(self.position, "position 不能为空");
        validate_required!(self.field_type, "field_type 不能为空");
        validate_required!(self.description, "description 不能为空");
        Ok(())
    }
}

/// 创建工单自定义字段响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTicketCustomizedFieldResponse {}

impl ApiResponseTrait for CreateTicketCustomizedFieldResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
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
    pub async fn execute(
        self,
        body: CreateTicketCustomizedFieldBody,
    ) -> SDKResult<CreateTicketCustomizedFieldResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
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

        Transport::request_typed(req, &self.config, Some(option), "创建工单自定义字段").await
    }
}

/// 创建工单自定义字段请求构建器
#[derive(Debug, Clone)]
pub struct CreateTicketCustomizedFieldRequestBuilder {
    config: Arc<Config>,
    helpdesk_id: Option<String>,
    key_name: Option<String>,
    display_name: Option<String>,
    position: Option<String>,
    field_type: Option<String>,
    description: Option<String>,
    visible: Option<bool>,
    required: Option<bool>,
    dropdown_options: Option<TicketCustomizedFieldDropdownOptions>,
    dropdown_allow_multiple: Option<bool>,
}

impl CreateTicketCustomizedFieldRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            helpdesk_id: None,
            key_name: None,
            display_name: None,
            position: None,
            field_type: None,
            description: None,
            visible: None,
            required: None,
            dropdown_options: None,
            dropdown_allow_multiple: None,
        }
    }

    /// 设置服务台 ID。
    pub fn helpdesk_id(mut self, helpdesk_id: impl Into<String>) -> Self {
        self.helpdesk_id = Some(helpdesk_id.into());
        self
    }

    /// 设置字段键名。
    pub fn key_name(mut self, key_name: impl Into<String>) -> Self {
        self.key_name = Some(key_name.into());
        self
    }

    /// 设置字段展示名称。
    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    /// 设置字段位置。
    pub fn position(mut self, position: impl Into<String>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// 设置字段类型。
    pub fn field_type(mut self, field_type: impl Into<String>) -> Self {
        self.field_type = Some(field_type.into());
        self
    }

    /// 设置字段描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置是否可见。
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    /// 设置是否必填。
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// 设置下拉选项。
    pub fn dropdown_options(
        mut self,
        dropdown_options: TicketCustomizedFieldDropdownOptions,
    ) -> Self {
        self.dropdown_options = Some(dropdown_options);
        self
    }

    /// 设置下拉字段是否允许多选。
    pub fn dropdown_allow_multiple(mut self, dropdown_allow_multiple: bool) -> Self {
        self.dropdown_allow_multiple = Some(dropdown_allow_multiple);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> Result<CreateTicketCustomizedFieldBody, String> {
        let key_name = self.key_name.clone().ok_or("key_name 不能为空")?;
        let display_name = self.display_name.clone().ok_or("display_name 不能为空")?;
        let position = self.position.clone().ok_or("position 不能为空")?;
        let field_type = self.field_type.clone().ok_or("field_type 不能为空")?;
        let description = self.description.clone().ok_or("description 不能为空")?;
        let visible = self.visible.ok_or("visible 不能为空")?;
        let required = self.required.ok_or("required 不能为空")?;

        Ok(CreateTicketCustomizedFieldBody {
            helpdesk_id: self.helpdesk_id.clone(),
            key_name,
            display_name,
            position,
            field_type,
            description,
            visible,
            required,
            dropdown_options: self.dropdown_options.clone(),
            dropdown_allow_multiple: self.dropdown_allow_multiple,
        })
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CreateTicketCustomizedFieldResponse> {
        let body = self
            .body()
            .map_err(|reason| openlark_core::error::validation_error("body", reason))?;
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

    Transport::request_typed(req, config, Some(option), "创建工单自定义字段").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid() {
        let body = CreateTicketCustomizedFieldBody {
            key_name: "priority".to_string(),
            display_name: "优先级".to_string(),
            position: "3".to_string(),
            field_type: "dropdown".to_string(),
            description: "工单优先级".to_string(),
            visible: true,
            required: false,
            ..Default::default()
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_display_name() {
        let body = CreateTicketCustomizedFieldBody {
            key_name: "priority".to_string(),
            display_name: " ".to_string(),
            position: "3".to_string(),
            field_type: "dropdown".to_string(),
            description: "工单优先级".to_string(),
            visible: true,
            required: false,
            ..Default::default()
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

        assert!(builder.display_name.is_none());
    }

    #[test]
    fn test_body_uses_official_fields() {
        let body = CreateTicketCustomizedFieldBody {
            helpdesk_id: Some("1542164574896126".to_string()),
            key_name: "priority".to_string(),
            display_name: "优先级".to_string(),
            position: "3".to_string(),
            field_type: "dropdown".to_string(),
            description: "工单优先级".to_string(),
            visible: true,
            required: false,
            dropdown_options: Some(TicketCustomizedFieldDropdownOptions {
                children: Some(vec![]),
            }),
            dropdown_allow_multiple: Some(true),
        };
        let value = serde_json::to_value(body).expect("请求体应可序列化");

        assert_eq!(value["key_name"], "priority");
        assert_eq!(value["display_name"], "优先级");
        assert_eq!(value["dropdown_allow_multiple"], true);
        assert!(value.get("name").is_none());
    }

    /// 端到端：POST .../ticket_customized_fields → 无 `data` 的成功响应可正确解析。
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/helpdesk/v1/ticket_customized_fields"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success"
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let body = CreateTicketCustomizedFieldBody {
            key_name: "ticket_number".to_string(),
            display_name: "工单编号".to_string(),
            position: "3".to_string(),
            field_type: "text".to_string(),
            description: "工单编号".to_string(),
            visible: true,
            required: true,
            ..Default::default()
        };
        CreateTicketCustomizedFieldRequest::new(config)
            .execute(body)
            .await
            .expect("创建工单自定义字段应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/ticket_customized_fields"
        );
        let request_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(request_body["display_name"], "工单编号");
        assert!(request_body.get("name").is_none());
    }
}
