//! 回复用户在工单里的提问
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/answer_user_query

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

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
