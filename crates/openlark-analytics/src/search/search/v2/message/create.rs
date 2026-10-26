//! 搜索消息
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/suite-search/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 搜索消息请求体。
///
/// `query` 必填；其余过滤字段可选（官方文档 Required=No）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessageBody {
    /// 搜索关键词（必填）。
    pub query: String,
    /// 消息来自 user_id 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_ids: Option<Vec<String>>,
    /// 消息所在 chat_id 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_ids: Option<Vec<String>>,
    /// 消息类型（file/image/media）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// at 用户 user_id 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_chatter_ids: Option<Vec<String>>,
    /// 消息来自类型（bot/user）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_type: Option<String>,
    /// 会话类型（group_chat/p2p_chat）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    /// 消息发送起始时间（秒级时间戳字符串）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 消息发送结束时间（秒级时间戳字符串）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

/// 搜索消息请求。
#[derive(Debug, Clone)]
pub struct SearchMessageRequest {
    config: Arc<Config>,
    body: SearchMessageBody,
    /// 用户 ID 类型（查询参数，默认 open_id）。
    user_id_type: Option<String>,
    /// 分页大小（查询参数，默认 20，取值范围 1～100）。
    page_size: Option<i32>,
    /// 分页标记（查询参数，第一次请求不填）。
    page_token: Option<String>,
}

/// 搜索消息响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessageResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SearchMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SearchMessageRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: SearchMessageBody::default(),
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置搜索关键词（必填）。
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = query.into();
        self
    }

    /// 设置消息来自 user_id 列表。
    pub fn from_ids(mut self, from_ids: Vec<String>) -> Self {
        self.body.from_ids = Some(from_ids);
        self
    }

    /// 设置消息所在 chat_id 列表。
    pub fn chat_ids(mut self, chat_ids: Vec<String>) -> Self {
        self.body.chat_ids = Some(chat_ids);
        self
    }

    /// 设置消息类型（file/image/media）。
    pub fn message_type(mut self, message_type: impl Into<String>) -> Self {
        self.body.message_type = Some(message_type.into());
        self
    }

    /// 设置 at 用户 user_id 列表。
    pub fn at_chatter_ids(mut self, at_chatter_ids: Vec<String>) -> Self {
        self.body.at_chatter_ids = Some(at_chatter_ids);
        self
    }

    /// 设置消息来自类型（bot/user）。
    pub fn from_type(mut self, from_type: impl Into<String>) -> Self {
        self.body.from_type = Some(from_type.into());
        self
    }

    /// 设置会话类型（group_chat/p2p_chat）。
    pub fn chat_type(mut self, chat_type: impl Into<String>) -> Self {
        self.body.chat_type = Some(chat_type.into());
        self
    }

    /// 设置消息发送起始时间（秒级时间戳字符串）。
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.body.start_time = Some(start_time.into());
        self
    }

    /// 设置消息发送结束时间（秒级时间戳字符串）。
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.body.end_time = Some(end_time.into());
        self
    }

    /// 设置用户 ID 类型（默认 open_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置分页大小（默认 20，取值范围 1～100）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（第一次请求不填）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行搜索消息请求。
    pub async fn execute(self) -> SDKResult<SearchMessageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行搜索消息请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchMessageResponse> {
        validate_required!(self.body.query, "query 不能为空");
        let path = "/open-apis/search/v2/message".to_string();
        let req: ApiRequest<SearchMessageResponse> = ApiRequest::post(&path)
            .body(serde_json::to_value(&self.body)?)
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()))
            .query_opt("page_token", self.page_token);

        Transport::request_typed(req, &self.config, Some(option), "搜索消息").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/search/v2/message + 官方形状 body + 分页查询参数 → 响应解析。
    #[tokio::test]
    async fn test_create_search_message_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/search/v2/message"))
            .and(wiremock::matchers::body_partial_json(json!({
                "query": "测试消息",
                "from_ids": ["ou_7dab8a3d3cdcc9da365777c7ad535d62"],
                "chat_ids": ["oc_a0553eda9014c201e6969b478895c230"],
                "message_type": "image",
                "at_chatter_ids": ["ou_7dab8a3d3cdcc9da365777c7ad535d62"],
                "from_type": "user",
                "chat_type": "group_chat",
                "start_time": "1609296809",
                "end_time": "1609296809"
            })))
            .and(wiremock::matchers::query_param("user_id_type", "open_id"))
            .and(wiremock::matchers::query_param("page_size", "20"))
            .and(wiremock::matchers::query_param("page_token", "tok_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = SearchMessageRequest::new(config)
            .query("测试消息")
            .from_ids(vec!["ou_7dab8a3d3cdcc9da365777c7ad535d62".to_string()])
            .chat_ids(vec!["oc_a0553eda9014c201e6969b478895c230".to_string()])
            .message_type("image")
            .at_chatter_ids(vec!["ou_7dab8a3d3cdcc9da365777c7ad535d62".to_string()])
            .from_type("user")
            .chat_type("group_chat")
            .start_time("1609296809")
            .end_time("1609296809")
            .user_id_type("open_id")
            .page_size(20)
            .page_token("tok_001")
            .execute()
            .await
            .expect("创建搜索消息应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/search/v2/message");
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["query"], "测试消息");
        assert_eq!(
            sent["from_ids"],
            json!(["ou_7dab8a3d3cdcc9da365777c7ad535d62"])
        );
        assert_eq!(
            sent["chat_ids"],
            json!(["oc_a0553eda9014c201e6969b478895c230"])
        );
        assert_eq!(sent["message_type"], "image");
        assert_eq!(sent["from_type"], "user");
        assert_eq!(sent["chat_type"], "group_chat");
        assert_eq!(sent["start_time"], "1609296809");
        assert_eq!(sent["end_time"], "1609296809");
    }

    /// 只设置必填 query 时 body 仅含 query。
    #[test]
    fn test_search_message_body_requires_query_only() {
        let body = SearchMessageBody {
            query: "测试消息".to_string(),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({ "query": "测试消息" })
        );
    }

    /// query 为空时应被校验拦截，不发请求。
    #[tokio::test]
    async fn test_create_search_message_rejects_blank_query() {
        let server = MockServer::start().await;
        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let result = SearchMessageRequest::new(config).execute().await;
        assert!(result.is_err());
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }
}
