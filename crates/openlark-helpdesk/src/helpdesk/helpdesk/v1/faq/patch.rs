//! 更新指定知识库
//!
//! 更新指定知识库的信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 更新知识库请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchFaqBody {
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
}

impl PatchFaqBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        if let Some(title) = &self.title
            && title.is_empty()
        {
            return Err(openlark_core::CoreError::validation_msg(
                "title cannot be empty",
            ));
        }
        if let Some(content) = &self.content
            && content.is_empty()
        {
            return Err(openlark_core::CoreError::validation_msg(
                "content cannot be empty",
            ));
        }
        Ok(())
    }
}

/// 更新知识库响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFaqResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PatchFaqResult>,
}

impl openlark_core::api::ApiResponseTrait for PatchFaqResponse {}

/// 更新知识库结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFaqResult {
    /// 知识库ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 更新知识库请求
#[derive(Debug, Clone)]
pub struct PatchFaqRequest {
    config: Arc<Config>,
    id: String,
}

impl PatchFaqRequest {
    /// 创建新的更新知识库请求
    pub fn new(config: Arc<Config>, id: String) -> Self {
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

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新知识库")
    }
}

/// 更新知识库请求构建器
#[derive(Debug, Clone)]
pub struct PatchFaqRequestBuilder {
    config: Arc<Config>,
    id: String,
    title: Option<String>,
    content: Option<String>,
    category_id: Option<String>,
}

impl PatchFaqRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self {
            config,
            id,
            title: None,
            content: None,
            category_id: None,
        }
    }

    /// 设置标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// 设置分类ID
    pub fn category_id(mut self, category_id: impl Into<String>) -> Self {
        self.category_id = Some(category_id.into());
        self
    }

    /// 构建请求体
    pub fn body(&self) -> PatchFaqBody {
        PatchFaqBody {
            title: self.title.clone(),
            content: self.content.clone(),
            category_id: self.category_id.clone(),
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<PatchFaqResponse> {
        let body = self.body();
        let request = PatchFaqRequest::new(self.config.clone(), self.id.clone());
        request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(&self, option: RequestOption) -> SDKResult<PatchFaqResponse> {
        let body = self.body();
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

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "更新知识库")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_empty() {
        let body = PatchFaqBody::default();
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = PatchFaqBody {
            title: Some("新标题".to_string()),
            content: Some("新内容".to_string()),
            category_id: None,
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_title() {
        let body = PatchFaqBody {
            title: Some("".to_string()),
            content: None,
            category_id: None,
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
        let builder = PatchFaqRequestBuilder::new(Arc::new(config), "faq_123".to_string());

        assert_eq!(builder.id, "faq_123");
        assert!(builder.title.is_none());
    }

    /// 端到端：PATCH .../faqs/{id} → 强类型 PatchFaqResponse 解析（双层 data 信封）。
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
                "data": { "data": { "id": "faq_001", "title": "新标题" } }
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

        let body = PatchFaqBody {
            title: Some("新标题".to_string()),
            content: None,
            category_id: None,
        };
        let resp = PatchFaqRequest::new(config, "faq_001".to_string())
            .execute(body)
            .await
            .expect("更新知识库应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/faqs/faq_001"
        );
    }
}
