//! 更新指定知识库
//!
//! 更新指定知识库的信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;

/// 更新知识库请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchFaqBody {
    /// 知识库内容
    pub faq: PatchFaq,
}

/// 待更新的知识库内容
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchFaq {
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 问题
    pub question: String,
    /// 答案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    /// 富文本答案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_richtext: Option<Vec<PatchFaqRichText>>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// 富文本答案节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFaqRichText {
    /// 节点内容
    pub content: String,
    /// 节点类型
    pub r#type: String,
}

impl PatchFaqBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.faq.question, "question 不能为空");
        Ok(())
    }
}

/// 更新知识库响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFaqResponse {
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

impl openlark_core::api::ApiResponseTrait for PatchFaqResponse {}

/// 更新知识库请求
#[derive(Debug, Clone)]
pub struct PatchFaqRequest {
    config: Config,
    id: String,
}

impl PatchFaqRequest {
    /// 创建新的更新知识库请求
    pub fn new(config: Config, id: String) -> Self {
        Self { config, id }
    }

    /// 执行更新知识库请求
    pub async fn execute(self, body: PatchFaqBody) -> SDKResult<PatchFaqResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行更新知识库请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: PatchFaqBody,
        option: RequestOption,
    ) -> SDKResult<PatchFaqResponse> {
        body.validate()?;

        let req: ApiRequest<PatchFaqResponse> =
            ApiRequest::patch(HelpdeskApiV1::FaqPatch(self.id.clone()).to_url())
                .body(serialize_params(&body, "更新知识库")?);

        Transport::request_typed(req, &self.config, Some(option), "更新知识库").await
    }
}

/// 更新知识库请求构建器
#[derive(Debug, Clone)]
pub struct PatchFaqRequestBuilder {
    config: Config,
    id: String,
    category_id: Option<String>,
    question: Option<String>,
    answer: Option<String>,
    answer_richtext: Option<Vec<PatchFaqRichText>>,
    tags: Option<Vec<String>>,
}

impl PatchFaqRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config, id: String) -> Self {
        Self {
            config,
            id,
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

    /// 设置富文本答案
    pub fn answer_richtext(mut self, answer_richtext: Vec<PatchFaqRichText>) -> Self {
        self.answer_richtext = Some(answer_richtext);
        self
    }

    /// 设置标签
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> Result<PatchFaqBody, String> {
        let question = self.question.clone().ok_or("question 不能为空")?;

        Ok(PatchFaqBody {
            faq: PatchFaq {
                category_id: self.category_id.clone(),
                question,
                answer: self.answer.clone(),
                answer_richtext: self.answer_richtext.clone(),
                tags: self.tags.clone(),
            },
        })
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<PatchFaqResponse> {
        let body = self
            .body()
            .map_err(|reason| openlark_core::error::validation_error("body", reason))?;
        let request = PatchFaqRequest::new(self.config.clone(), self.id.clone());
        request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(&self, option: RequestOption) -> SDKResult<PatchFaqResponse> {
        let body = self
            .body()
            .map_err(|reason| openlark_core::error::validation_error("body", reason))?;
        let request = PatchFaqRequest::new(self.config.clone(), self.id.clone());
        request.execute_with_options(body, option).await
    }
}

/// 执行更新知识库
pub async fn patch_faq(
    config: &Config,
    id: String,
    body: PatchFaqBody,
) -> SDKResult<PatchFaqResponse> {
    patch_faq_with_options(config, id, body, RequestOption::default()).await
}

/// 执行更新知识库（支持自定义选项）
pub async fn patch_faq_with_options(
    config: &Config,
    id: String,
    body: PatchFaqBody,
    option: RequestOption,
) -> SDKResult<PatchFaqResponse> {
    body.validate()?;

    let req: ApiRequest<PatchFaqResponse> = ApiRequest::patch(HelpdeskApiV1::FaqPatch(id).to_url())
        .body(serialize_params(&body, "更新知识库")?);

    Transport::request_typed(req, config, Some(option), "更新知识库").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_empty_question() {
        let body = PatchFaqBody::default();
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = PatchFaqBody {
            faq: PatchFaq {
                question: "新问题".to_string(),
                answer: Some("新答案".to_string()),
                ..Default::default()
            },
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_serialization_matches_official_shape() {
        let body = PatchFaqBody {
            faq: PatchFaq {
                question: "新问题".to_string(),
                answer_richtext: Some(vec![PatchFaqRichText {
                    content: "富文本".to_string(),
                    r#type: "text".to_string(),
                }]),
                ..Default::default()
            },
        };
        let value = serde_json::to_value(body).expect("请求体应可序列化");

        assert!(value["faq"]["answer_richtext"].is_array());
        assert_eq!(value["faq"]["answer_richtext"][0]["type"], "text");
        assert!(value.get("question").is_none());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = PatchFaqRequestBuilder::new(config, "faq_123".to_string());

        assert_eq!(builder.id, "faq_123");
        assert!(builder.question.is_none());
    }

    /// 端到端：PATCH .../faqs/{id} → 强类型 PatchFaqResponse 解析。
    #[tokio::test]
    async fn test_patch_faq_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/helpdesk/v1/faqs/faq_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "id": "faq_001", "question": "新问题" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = PatchFaqBody {
            faq: PatchFaq {
                question: "新问题".to_string(),
                ..Default::default()
            },
        };
        let resp = PatchFaqRequest::new(config, "faq_001".to_string())
            .execute(body)
            .await
            .expect("更新知识库应成功");
        assert_eq!(resp.id.as_deref(), Some("faq_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/faqs/faq_001"
        );
    }
}
