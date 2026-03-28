//! 通过服务台机器人发送消息
//!
//! 通过服务台机器人给指定用户的服务台专属群或私聊发送消息，支持文本、富文本、卡片、图片。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket-message/create-2>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 通过服务台机器人发送消息请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBotMessageBody {
    /// 接收者的 open_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_id: Option<String>,
    /// 消息类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    /// 消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl CreateBotMessageBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.receive_id.is_none() {
            return Err("receive_id is required".to_string());
        }
        if self.msg_type.is_none() {
            return Err("msg_type is required".to_string());
        }
        if self.content.is_none() {
            return Err("content is required".to_string());
        }
        Ok(())
    }
}

/// 通过服务台机器人发送消息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBotMessageResponse {
    /// 消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

impl ApiResponseTrait for CreateBotMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 通过服务台机器人发送消息请求
#[derive(Debug, Clone)]
pub struct CreateBotMessageRequest {
    config: Arc<Config>,
}

impl CreateBotMessageRequest {
    /// 创建新的通过服务台机器人发送消息请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行通过服务台机器人发送消息请求
    pub async fn execute(self, body: CreateBotMessageBody) -> SDKResult<CreateBotMessageResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行通过服务台机器人发送消息请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: CreateBotMessageBody,
        option: RequestOption,
    ) -> SDKResult<CreateBotMessageResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<CreateBotMessageResponse> =
            ApiRequest::post(HelpdeskApiV1::BotMessageCreate.to_url())
                .body(serialize_params(&body, "通过服务台机器人发送消息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "通过服务台机器人发送消息")
    }
}

/// 通过服务台机器人发送消息请求构建器
#[derive(Debug, Clone)]
pub struct CreateBotMessageRequestBuilder {
    config: Arc<Config>,
    receive_id: Option<String>,
    msg_type: Option<String>,
    content: Option<String>,
}

impl CreateBotMessageRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            receive_id: None,
            msg_type: None,
            content: None,
        }
    }

    /// 设置接收者的 open_id
    pub fn receive_id(mut self, receive_id: impl Into<String>) -> Self {
        self.receive_id = Some(receive_id.into());
        self
    }

    /// 设置消息类型
    pub fn msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.msg_type = Some(msg_type.into());
        self
    }

    /// 设置消息内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// 构建请求体
    pub fn body(&self) -> Result<CreateBotMessageBody, String> {
        let receive_id = self.receive_id.clone().ok_or("receive_id is required")?;
        let msg_type = self.msg_type.clone().ok_or("msg_type is required")?;
        let content = self.content.clone().ok_or("content is required")?;

        Ok(CreateBotMessageBody {
            receive_id: Some(receive_id),
            msg_type: Some(msg_type),
            content: Some(content),
        })
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CreateBotMessageResponse> {
        let body = self
            .body()
            .map_err(|reason| openlark_core::error::validation_error("body", reason))?;
        let request = CreateBotMessageRequest::new(self.config.clone());
        request.execute(body).await
    }
}

/// 执行通过服务台机器人发送消息
pub async fn create_bot_message(
    config: &Config,
    body: CreateBotMessageBody,
) -> SDKResult<CreateBotMessageResponse> {
    create_bot_message_with_options(config, body, RequestOption::default()).await
}

/// 执行通过服务台机器人发送消息（支持自定义选项）
pub async fn create_bot_message_with_options(
    config: &Config,
    body: CreateBotMessageBody,
    option: RequestOption,
) -> SDKResult<CreateBotMessageResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<CreateBotMessageResponse> =
        ApiRequest::post(HelpdeskApiV1::BotMessageCreate.to_url())
            .body(serialize_params(&body, "通过服务台机器人发送消息")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "通过服务台机器人发送消息")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid() {
        let body = CreateBotMessageBody {
            receive_id: Some("ou_xxx".to_string()),
            msg_type: Some("text".to_string()),
            content: Some(r#"{"text":"hello"}"#.to_string()),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_missing_receive_id() {
        let body = CreateBotMessageBody {
            receive_id: None,
            msg_type: Some("text".to_string()),
            content: Some(r#"{"text":"hello"}"#.to_string()),
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
        let builder = CreateBotMessageRequestBuilder::new(Arc::new(config));

        assert!(builder.receive_id.is_none());
    }
}
