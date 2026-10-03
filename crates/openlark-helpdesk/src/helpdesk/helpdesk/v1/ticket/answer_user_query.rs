//! 回复用户在工单里的提问
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/answer_user_query>

use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 回复用户提问请求。
#[derive(Debug, Clone)]
pub struct AnswerUserQueryRequest {
    config: Config,
    ticket_id: String,
    body: AnswerUserQueryBody,
}

/// 回复用户提问请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnswerUserQueryBody {
    /// 事件 ID。
    pub event_id: String,
    /// 推荐的知识库答案。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faqs: Option<Vec<AnswerUserQueryFaq>>,
}

/// 推荐的知识库答案。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerUserQueryFaq {
    /// 知识库 ID。
    pub id: String,
    /// 匹配分数。
    pub score: f64,
}

impl AnswerUserQueryBody {
    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.event_id, "event_id 不能为空");
        Ok(())
    }
}

/// 回复用户提问响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerUserQueryResponse {
    /// 消息 ID。
    pub message_id: String,
}

impl ApiResponseTrait for AnswerUserQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AnswerUserQueryRequest {
    /// 创建新的实例。
    pub fn new(config: Config, ticket_id: impl Into<String>) -> Self {
        Self {
            config,
            ticket_id: ticket_id.into(),
            body: AnswerUserQueryBody::default(),
        }
    }

    /// 设置事件 ID。
    pub fn event_id(mut self, event_id: impl Into<String>) -> Self {
        self.body.event_id = event_id.into();
        self
    }

    /// 设置推荐的知识库答案。
    pub fn faqs(mut self, faqs: Vec<AnswerUserQueryFaq>) -> Self {
        self.body.faqs = Some(faqs);
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

        Transport::request_typed(req, &self.config, Some(option), "回复用户在工单里的提问").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_requires_event_id() {
        let body = AnswerUserQueryBody::default();
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_serialization_matches_official_shape() {
        let body = AnswerUserQueryBody {
            event_id: "abcd".to_string(),
            faqs: Some(vec![AnswerUserQueryFaq {
                id: "12345".to_string(),
                score: 0.9,
            }]),
        };
        let value = serde_json::to_value(body).expect("请求体应可序列化");

        assert_eq!(value["event_id"], "abcd");
        assert_eq!(value["faqs"][0]["id"], "12345");
        assert!(value.get("content").is_none());
        assert!(value.get("content_type").is_none());
    }

    /// 端到端：POST .../tickets/{id}/answer_user_query → 强类型响应解析。
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

        let resp = AnswerUserQueryRequest::new(config, "tk_001")
            .event_id("abcd")
            .faqs(vec![AnswerUserQueryFaq {
                id: "12345".to_string(),
                score: 0.9,
            }])
            .execute()
            .await
            .expect("回复用户提问应成功");
        assert_eq!(resp.message_id, "msg_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/tickets/tk_001/answer_user_query"
        );
    }
}
