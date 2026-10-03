//! 更新指定工单自定义字段
//!
//! 更新指定工单自定义字段的信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket_customized_field/patch>

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

/// 更新工单自定义字段请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchTicketCustomizedFieldBody {
    /// 字段展示名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 字段位置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 字段描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否可见。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    /// 是否必填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 下拉选项。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown_options: Option<TicketCustomizedFieldDropdownOptions>,
}

impl PatchTicketCustomizedFieldBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        if let Some(display_name) = &self.display_name {
            validate_required!(display_name, "display_name 不能为空");
        }
        if let Some(position) = &self.position {
            validate_required!(position, "position 不能为空");
        }
        if let Some(description) = &self.description {
            validate_required!(description, "description 不能为空");
        }
        Ok(())
    }
}

/// 更新工单自定义字段响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchTicketCustomizedFieldResponse {}

impl ApiResponseTrait for PatchTicketCustomizedFieldResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
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
    pub async fn execute(
        self,
        body: PatchTicketCustomizedFieldBody,
    ) -> SDKResult<PatchTicketCustomizedFieldResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
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

        Transport::request_typed(req, &self.config, Some(option), "更新工单自定义字段").await
    }
}

/// 更新工单自定义字段请求构建器
#[derive(Debug, Clone)]
pub struct PatchTicketCustomizedFieldRequestBuilder {
    config: Arc<Config>,
    id: String,
    display_name: Option<String>,
    position: Option<String>,
    description: Option<String>,
    visible: Option<bool>,
    required: Option<bool>,
    dropdown_options: Option<TicketCustomizedFieldDropdownOptions>,
}

impl PatchTicketCustomizedFieldRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self {
            config,
            id,
            display_name: None,
            position: None,
            description: None,
            visible: None,
            required: None,
            dropdown_options: None,
        }
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

    /// 构建请求体
    pub fn body(&self) -> PatchTicketCustomizedFieldBody {
        PatchTicketCustomizedFieldBody {
            display_name: self.display_name.clone(),
            position: self.position.clone(),
            description: self.description.clone(),
            visible: self.visible,
            required: self.required,
            dropdown_options: self.dropdown_options.clone(),
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

    Transport::request_typed(req, config, Some(option), "更新工单自定义字段").await
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
            display_name: Some("新名称".to_string()),
            position: Some("4".to_string()),
            visible: Some(true),
            required: Some(true),
            ..Default::default()
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_display_name() {
        let body = PatchTicketCustomizedFieldBody {
            display_name: Some(" ".to_string()),
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
        let builder = PatchTicketCustomizedFieldRequestBuilder::new(
            Arc::new(config),
            "field_123".to_string(),
        );

        assert_eq!(builder.id, "field_123");
        assert!(builder.display_name.is_none());
    }

    #[test]
    fn test_body_uses_official_fields() {
        let body = PatchTicketCustomizedFieldBody {
            display_name: Some("新名称".to_string()),
            position: Some("4".to_string()),
            visible: Some(true),
            dropdown_options: Some(TicketCustomizedFieldDropdownOptions {
                children: Some(vec![]),
            }),
            ..Default::default()
        };
        let value = serde_json::to_value(body).expect("请求体应可序列化");

        assert_eq!(value["display_name"], "新名称");
        assert_eq!(value["position"], "4");
        assert_eq!(value["visible"], true);
        assert!(value.get("name").is_none());
    }

    /// 端到端：PATCH .../ticket_customized_fields/{id} → 无 `data` 的成功响应可正确解析。
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/helpdesk/v1/ticket_customized_fields/tcf_001",
            ))
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

        let body = PatchTicketCustomizedFieldBody {
            display_name: Some("工单编号-改".to_string()),
            position: Some("4".to_string()),
            visible: Some(true),
            required: Some(true),
            ..Default::default()
        };
        PatchTicketCustomizedFieldRequest::new(config, "tcf_001".to_string())
            .execute(body)
            .await
            .expect("更新工单自定义字段应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/ticket_customized_fields/tcf_001"
        );
        let request_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(request_body["display_name"], "工单编号-改");
        assert!(request_body.get("name").is_none());
    }
}
