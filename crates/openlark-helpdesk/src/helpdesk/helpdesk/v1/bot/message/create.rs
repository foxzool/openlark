//! 通过服务台机器人发送消息
//!
//! 通过服务台机器人给指定用户的服务台专属群或私聊发送消息，支持文本、富文本、卡片、图片。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket-message/create-2>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;

/// 通过服务台机器人发送消息请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBotMessageBody {
    /// 消息类型
    pub msg_type: String,
    /// 消息内容
    pub content: String,
    /// 接收者 ID
    pub receiver_id: String,
    /// 接收者类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_type: Option<String>,
}

impl CreateBotMessageBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.msg_type, "msg_type 不能为空");
        validate_required!(self.content, "content 不能为空");
        validate_required!(self.receiver_id, "receiver_id 不能为空");
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
    config: Config,
}

impl CreateBotMessageRequest {
    /// 创建新的通过服务台机器人发送消息请求
    pub fn new(config: Config) -> Self {
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
        body.validate()?;

        let req: ApiRequest<CreateBotMessageResponse> =
            ApiRequest::post(HelpdeskApiV1::BotMessageCreate.to_url())
                .body(serialize_params(&body, "通过服务台机器人发送消息")?);

        Transport::request_typed(req, &self.config, Some(option), "通过服务台机器人发送消息").await
    }
}

/// 通过服务台机器人发送消息请求构建器
#[derive(Debug, Clone)]
pub struct CreateBotMessageRequestBuilder {
    config: Config,
    msg_type: Option<String>,
    content: Option<String>,
    receiver_id: Option<String>,
    receive_type: Option<String>,
}

impl CreateBotMessageRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            msg_type: None,
            content: None,
            receiver_id: None,
            receive_type: None,
        }
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

    /// 设置接收者 ID
    pub fn receiver_id(mut self, receiver_id: impl Into<String>) -> Self {
        self.receiver_id = Some(receiver_id.into());
        self
    }

    /// 设置接收者类型
    pub fn receive_type(mut self, receive_type: impl Into<String>) -> Self {
        self.receive_type = Some(receive_type.into());
        self
    }

    /// 构建请求体
    pub fn body(&self) -> Result<CreateBotMessageBody, String> {
        let msg_type = self.msg_type.clone().ok_or("msg_type 不能为空")?;
        let content = self.content.clone().ok_or("content 不能为空")?;
        let receiver_id = self.receiver_id.clone().ok_or("receiver_id 不能为空")?;

        Ok(CreateBotMessageBody {
            msg_type,
            content,
            receiver_id,
            receive_type: self.receive_type.clone(),
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
    body.validate()?;

    let req: ApiRequest<CreateBotMessageResponse> =
        ApiRequest::post(HelpdeskApiV1::BotMessageCreate.to_url())
            .body(serialize_params(&body, "通过服务台机器人发送消息")?);

    Transport::request_typed(req, config, Some(option), "通过服务台机器人发送消息").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid() {
        let body = CreateBotMessageBody {
            msg_type: "post".to_string(),
            content: r#"{"text":"hello"}"#.to_string(),
            receiver_id: "ou_xxx".to_string(),
            receive_type: Some("chat".to_string()),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_missing_receiver_id() {
        let body = CreateBotMessageBody {
            msg_type: "post".to_string(),
            content: r#"{"text":"hello"}"#.to_string(),
            receiver_id: String::new(),
            receive_type: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_serialization_matches_official_shape() {
        let body = CreateBotMessageBody {
            msg_type: "post".to_string(),
            content: "消息内容".to_string(),
            receiver_id: "ou_xxx".to_string(),
            receive_type: Some("chat".to_string()),
        };
        let value = serde_json::to_value(body).expect("请求体应可序列化");

        assert_eq!(value["receiver_id"], "ou_xxx");
        assert_eq!(value["receive_type"], "chat");
        assert!(value.get("receive_id").is_none());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = CreateBotMessageRequestBuilder::new(config);

        assert!(builder.receiver_id.is_none());
    }

    /// 端到端：POST .../message → 强类型 CreateBotMessageResponse 解析（data 内层为 message_id）。
    #[tokio::test]
    async fn test_create_bot_message_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/helpdesk/v1/message"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "message_id": "msg_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = CreateBotMessageBody {
            msg_type: "text".to_string(),
            content: r#"{"text":"hello"}"#.to_string(),
            receiver_id: "ou_test_user".to_string(),
            receive_type: None,
        };
        let resp = CreateBotMessageRequest::new(config)
            .execute(body)
            .await
            .expect("机器人发送消息应成功");
        assert_eq!(resp.message_id.as_deref(), Some("msg_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/helpdesk/v1/message");
    }
}
