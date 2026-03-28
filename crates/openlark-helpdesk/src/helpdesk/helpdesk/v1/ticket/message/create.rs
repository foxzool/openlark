//! 发送工单消息
//!
//! 发送工单消息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket-message/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
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
    pub fn validate(&self) -> Result<(), String> {
        if self.content.is_empty() {
            return Err("content is required".to_string());
        }
        Ok(())
    }
}

/// 发送工单消息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketMessageResponse {
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
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

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
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

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
}
