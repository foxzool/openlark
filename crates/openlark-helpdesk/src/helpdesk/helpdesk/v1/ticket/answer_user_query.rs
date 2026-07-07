//! 回复用户在工单里的提问
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/answer_user_query>

use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 回复用户提问请求。
#[derive(Debug, Clone)]
pub struct AnswerUserQueryRequest {
    config: Arc<Config>,
    ticket_id: String,
    body: AnswerUserQueryBody,
}

/// 回复用户提问请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnswerUserQueryBody {
    /// 回复内容。
    pub content: String,
    /// 回复内容类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl AnswerUserQueryBody {
    fn validate(&self) -> SDKResult<()> {
        if self.content.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "回复内容不能为空",
                "",
            ));
        }
        Ok(())
    }
}

/// 回复用户提问响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerUserQueryResponse {
    /// 响应数据。
    pub data: Option<AnswerUserQueryData>,
}

impl ApiResponseTrait for AnswerUserQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 回复用户提问响应数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerUserQueryData {
    /// 消息 ID。
    pub message_id: String,
}

impl AnswerUserQueryRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, ticket_id: impl Into<String>) -> Self {
        Self {
            config,
            ticket_id: ticket_id.into(),
            body: AnswerUserQueryBody::default(),
        }
    }

    /// 设置回复内容。
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = content.into();
        self
    }

    /// 设置回复内容类型。
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.body.content_type = Some(content_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<AnswerUserQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AnswerUserQueryResponse> {
        self.body.validate()?;

        let path = HelpdeskApiV1::TicketAnswerUserQuery(self.ticket_id.clone()).to_url();
        let req: ApiRequest<AnswerUserQueryResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "回复用户在工单里的提问")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("回复用户在工单里的提问", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../tickets/{id}/answer_user_query → 强类型 AnswerUserQueryResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_answer_user_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/helpdesk/v1/tickets/tk_001/answer_user_query",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "message_id": "msg_001" } }
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

        let resp = AnswerUserQueryRequest::new(config, "tk_001")
            .content("您好，请问需要什么帮助？")
            .execute()
            .await
            .expect("回复用户提问应成功");
        assert!(resp.data.is_some());
        assert_eq!(resp.data.unwrap().message_id, "msg_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/tickets/tk_001/answer_user_query"
        );
    }
}
