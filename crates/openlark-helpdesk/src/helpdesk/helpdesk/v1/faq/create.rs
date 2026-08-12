//! 创建知识库
//!
//! 创建新的知识库。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;

/// 创建知识库请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFaqBody {
    /// 知识库内容
    pub faq: CreateFaq,
}

/// 待创建的知识库内容
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFaq {
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 问题
    pub question: String,
    /// 答案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    /// 富文本答案，创建接口使用 JSON 字符串
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_richtext: Option<String>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl CreateFaqBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.faq.question, "question 不能为空");
        Ok(())
    }
}

/// 创建知识库响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFaqResponse {
    /// 知识库ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 问题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    /// 答案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for CreateFaqResponse {}

/// 创建知识库请求
#[derive(Debug, Clone)]
pub struct CreateFaqRequest {
    config: Config,
}

impl CreateFaqRequest {
    /// 创建新的创建知识库请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行创建知识库请求
    pub async fn execute(self, body: CreateFaqBody) -> SDKResult<CreateFaqResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行创建知识库请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: CreateFaqBody,
        option: RequestOption,
    ) -> SDKResult<CreateFaqResponse> {
        body.validate()?;

        let req: ApiRequest<CreateFaqResponse> =
            ApiRequest::post(HelpdeskApiV1::FaqCreate.to_url())
                .body(serialize_params(&body, "创建知识库")?);

        Transport::request_typed(req, &self.config, Some(option), "创建知识库").await
    }
}

/// 创建知识库请求构建器
#[derive(Debug, Clone)]
pub struct CreateFaqRequestBuilder {
    config: Config,
    category_id: Option<String>,
    question: Option<String>,
    answer: Option<String>,
    answer_richtext: Option<String>,
    tags: Option<Vec<String>>,
}

impl CreateFaqRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            category_id: None,
            question: None,
            answer: None,
            answer_richtext: None,
            tags: None,
        }
    }

    /// 设置分类ID
    pub fn category_id(mut self, category_id: impl Into<String>) -> Self {
        self.category_id = Some(category_id.into());
        self
    }

    /// 设置问题
    pub fn question(mut self, question: impl Into<String>) -> Self {
        self.question = Some(question.into());
        self
    }

    /// 设置答案
    pub fn answer(mut self, answer: impl Into<String>) -> Self {
        self.answer = Some(answer.into());
        self
    }

    /// 设置富文本答案 JSON 字符串
    pub fn answer_richtext(mut self, answer_richtext: impl Into<String>) -> Self {
        self.answer_richtext = Some(answer_richtext.into());
        self
    }

    /// 设置标签
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> Result<CreateFaqBody, String> {
        let question = self.question.clone().ok_or("question 不能为空")?;

        Ok(CreateFaqBody {
            faq: CreateFaq {
                category_id: self.category_id.clone(),
                question,
                answer: self.answer.clone(),
                answer_richtext: self.answer_richtext.clone(),
                tags: self.tags.clone(),
            },
        })
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CreateFaqResponse> {
        let body = self
            .body()
            .map_err(|reason| openlark_core::error::validation_error("body", reason))?;
        let request = CreateFaqRequest::new(self.config.clone());
        request.execute(body).await
    }
}

/// 执行创建知识库
pub async fn create_faq(config: &Config, body: CreateFaqBody) -> SDKResult<CreateFaqResponse> {
    create_faq_with_options(config, body, RequestOption::default()).await
}

/// 执行创建知识库（支持自定义选项）
pub async fn create_faq_with_options(
    config: &Config,
    body: CreateFaqBody,
    option: RequestOption,
) -> SDKResult<CreateFaqResponse> {
    body.validate()?;

    let req: ApiRequest<CreateFaqResponse> = ApiRequest::post(HelpdeskApiV1::FaqCreate.to_url())
        .body(serialize_params(&body, "创建知识库")?);

    Transport::request_typed(req, config, Some(option), "创建知识库").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid() {
        let body = CreateFaqBody {
            faq: CreateFaq {
                category_id: Some("cat_123".to_string()),
                question: "如何重置密码？".to_string(),
                answer: Some("请按照以下步骤重置密码...".to_string()),
                answer_richtext: Some(r#"[{\"type\":\"text\",\"content\":\"步骤\"}]"#.to_string()),
                tags: Some(vec!["账号".to_string()]),
            },
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_question() {
        let body = CreateFaqBody {
            faq: CreateFaq::default(),
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_serialization_matches_official_shape() {
        let body = CreateFaqBody {
            faq: CreateFaq {
                question: "问题".to_string(),
                answer_richtext: Some("[{\"type\":\"text\"}]".to_string()),
                ..Default::default()
            },
        };
        let value = serde_json::to_value(body).expect("请求体应可序列化");

        assert_eq!(value["faq"]["question"], "问题");
        assert!(value["faq"]["answer_richtext"].is_string());
        assert!(value.get("question").is_none());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = CreateFaqRequestBuilder::new(config);

        assert!(builder.question.is_none());
    }

    /// 端到端：POST .../faqs → 强类型 CreateFaqResponse 解析。
    #[tokio::test]
    async fn test_create_faq_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/helpdesk/v1/faqs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "id": "faq_001", "question": "如何重置密码？" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = CreateFaqBody {
            faq: CreateFaq {
                question: "如何重置密码？".to_string(),
                answer: Some("请按照以下步骤重置密码...".to_string()),
                ..Default::default()
            },
        };
        let resp = CreateFaqRequest::new(config)
            .execute(body)
            .await
            .expect("创建知识库应成功");
        assert_eq!(resp.id.as_deref(), Some("faq_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/helpdesk/v1/faqs");
    }
}
