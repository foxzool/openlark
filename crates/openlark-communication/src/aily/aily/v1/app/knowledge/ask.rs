//! 执行数据知识问答
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/ask>

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_KNOWLEDGE_ASK};
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

/// 执行数据知识问答请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AskKnowledgeBody {
    /// 提问内容。
    pub question: String,
    /// 是否流式返回。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl AskKnowledgeBody {
    /// 创建新的请求构建器。
    pub fn new(question: impl Into<String>) -> Self {
        Self {
            question: question.into(),
            stream: None,
        }
    }

    /// 设置是否流式返回。
    /// 设置是否流式返回。
    /// 设置是否流式返回。
    /// 设置是否流式返回。
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
}

/// 执行数据知识问答请求
///
/// 用于对数据知识执行问答操作。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `app_id`: 应用 ID，必填
///
/// # 请求体字段
///
/// - `question`: 问题内容，必填
/// - `stream`: 是否流式返回（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = AskKnowledgeBody::new("什么是人工智能？")
///     .stream(false);
/// let request = AskKnowledgeRequest::new(config)
///     .app_id("app_xxx")
///     .execute(body).await?;
/// ```
pub struct AskKnowledgeRequest {
    config: Config,
    app_id: String,
}

impl AskKnowledgeRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
        }
    }

    /// 应用 ID（路径参数）
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/ask>
    pub async fn execute(self, body: AskKnowledgeBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: AskKnowledgeBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.app_id, "app_id 不能为空");

        let url = AILY_V1_KNOWLEDGE_ASK.replace("{app_id}", &self.app_id);
        let req: ApiRequest<AskKnowledgeBody> = ApiRequest::post(&url).json_body(&body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "执行数据知识问答")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_ask_knowledge_request_builder() {
        let config = Config::default();
        let request = AskKnowledgeRequest::new(config).app_id("app_xxx");
        assert_eq!(request.app_id, "app_xxx");
    }

    #[test]
    fn test_ask_knowledge_body_builder() {
        let body = AskKnowledgeBody::new("测试问题");
        assert_eq!(body.question, "测试问题");
        assert_eq!(body.stream, None);
    }

    #[test]
    fn test_ask_knowledge_body_with_stream() {
        let body = AskKnowledgeBody::new("测试问题").stream(true);
        assert_eq!(body.stream, Some(true));
    }

    #[test]
    fn test_ask_knowledge_request_default_values() {
        let config = Config::default();
        let request = AskKnowledgeRequest::new(config);
        assert_eq!(request.app_id, "");
    }

    #[test]
    fn test_ask_knowledge_request_chaining() {
        let config = Config::default();
        let request = AskKnowledgeRequest::new(config).app_id("app_123");
        assert_eq!(request.app_id, "app_123");
    }

    #[test]
    fn test_ask_knowledge_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_KNOWLEDGE_ASK;
        let url = AILY_V1_KNOWLEDGE_ASK.replace("{app_id}", "app_1");
        assert_eq!(url, "/open-apis/aily/v1/apps/app_1/knowledges/ask");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }
}
