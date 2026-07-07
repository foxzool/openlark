//! 发送工单消息
//!
//! 发送工单消息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket-message/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 发送工单消息请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTicketMessageBody {
    /// 消息内容
    pub content: String,
    /// 消息类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
}

impl CreateTicketMessageBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.content, "content is required");
        Ok(())
    }
}

/// 发送工单消息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketMessageResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateTicketMessageResult>,
}

impl openlark_core::api::ApiResponseTrait for CreateTicketMessageResponse {}

/// 发送工单消息结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketMessageResult {
    /// 消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// 发送工单消息请求
#[derive(Debug, Clone)]
pub struct CreateTicketMessageRequest {
    config: Arc<Config>,
    ticket_id: String,
}

impl CreateTicketMessageRequest {
    /// 创建新的发送工单消息请求
    pub fn new(config: Arc<Config>, ticket_id: String) -> Self {
        Self { config, ticket_id }
    }

    /// 执行发送工单消息请求
    pub async fn execute(
        self,
        body: CreateTicketMessageBody,
    ) -> SDKResult<CreateTicketMessageResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行发送工单消息请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: CreateTicketMessageBody,
        option: RequestOption,
    ) -> SDKResult<CreateTicketMessageResponse> {
        body.validate()?;

        let req: ApiRequest<CreateTicketMessageResponse> =
            ApiRequest::post(HelpdeskApiV1::TicketMessageCreate(self.ticket_id.clone()).to_url())
                .body(serialize_params(&body, "发送工单消息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "发送工单消息")
    }
}

/// 发送工单消息请求构建器
#[derive(Debug, Clone)]
pub struct CreateTicketMessageRequestBuilder {
    config: Arc<Config>,
    ticket_id: String,
    content: Option<String>,
    msg_type: Option<String>,
}

impl CreateTicketMessageRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, ticket_id: String) -> Self {
        Self {
            config,
            ticket_id,
            content: None,
            msg_type: None,
        }
    }

    /// 设置消息内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// 设置消息类型
    pub fn msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.msg_type = Some(msg_type.into());
        self
    }

    /// 构建请求体
    pub fn body(&self) -> Result<CreateTicketMessageBody, String> {
        let content = self.content.clone().ok_or("content is required")?;

        Ok(CreateTicketMessageBody {
            content,
            msg_type: self.msg_type.clone(),
        })
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CreateTicketMessageResponse> {
        let body = self
            .body()
            .map_err(|reason| openlark_core::error::validation_error("body", reason))?;
        let request = CreateTicketMessageRequest::new(self.config.clone(), self.ticket_id.clone());
        request.execute(body).await
    }
}

/// 执行发送工单消息
pub async fn create_ticket_message(
    config: &Config,
    ticket_id: String,
    body: CreateTicketMessageBody,
) -> SDKResult<CreateTicketMessageResponse> {
    create_ticket_message_with_options(config, ticket_id, body, RequestOption::default()).await
}

/// 执行发送工单消息（支持自定义选项）
pub async fn create_ticket_message_with_options(
    config: &Config,
    ticket_id: String,
    body: CreateTicketMessageBody,
    option: RequestOption,
) -> SDKResult<CreateTicketMessageResponse> {
    body.validate()?;

    let req: ApiRequest<CreateTicketMessageResponse> =
        ApiRequest::post(HelpdeskApiV1::TicketMessageCreate(ticket_id).to_url())
            .body(serialize_params(&body, "发送工单消息")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "发送工单消息")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid() {
        let body = CreateTicketMessageBody {
            content: "这是一条测试消息".to_string(),
            msg_type: Some("text".to_string()),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_content() {
        let body = CreateTicketMessageBody {
            content: "".to_string(),
            msg_type: None,
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
        let builder =
            CreateTicketMessageRequestBuilder::new(Arc::new(config), "ticket_123".to_string());

        assert_eq!(builder.ticket_id, "ticket_123");
        assert!(builder.content.is_none());
    }

    /// 端到端：POST .../tickets/{id}/messages → 强类型 CreateTicketMessageResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_create_ticket_message_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/helpdesk/v1/tickets/tk_001/messages"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "id": "msg_001" } }
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

        let body = CreateTicketMessageBody {
            content: "您好，请问需要什么帮助？".to_string(),
            msg_type: Some("text".to_string()),
        };
        let resp = CreateTicketMessageRequest::new(config, "tk_001".to_string())
            .execute(body)
            .await
            .expect("发送工单消息应成功");
        assert!(resp.data.is_some());
        assert_eq!(resp.data.unwrap().id.as_deref(), Some("msg_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/tickets/tk_001/messages"
        );
    }
}
