//! 创建知识库分类
//!
//! 创建新的知识库分类。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/category/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;

/// 创建知识库分类请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCategoryBody {
    /// 分类名称
    pub name: String,
    /// 父分类ID
    pub parent_id: String,
    /// 语言。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl CreateCategoryBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.name, "name 不能为空");
        validate_required!(self.parent_id, "parent_id 不能为空");
        Ok(())
    }
}

/// 创建知识库分类响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategoryResponse {
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 分类名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 父分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 语言。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for CreateCategoryResponse {}

/// 创建知识库分类结果。
pub type CreateCategoryResult = CreateCategoryResponse;

/// 创建知识库分类请求
#[derive(Debug, Clone)]
pub struct CreateCategoryRequest {
    config: Config,
}

impl CreateCategoryRequest {
    /// 创建新的创建知识库分类请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行创建知识库分类请求
    pub async fn execute(self, body: CreateCategoryBody) -> SDKResult<CreateCategoryResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行创建知识库分类请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: CreateCategoryBody,
        option: RequestOption,
    ) -> SDKResult<CreateCategoryResponse> {
        body.validate()?;

        let req: ApiRequest<CreateCategoryResponse> =
            ApiRequest::post(HelpdeskApiV1::CategoryCreate.to_url())
                .body(serialize_params(&body, "创建知识库分类")?);

        Transport::request_typed(req, &self.config, Some(option), "创建知识库分类").await
    }
}

/// 创建知识库分类请求构建器
#[derive(Debug, Clone)]
pub struct CreateCategoryRequestBuilder {
    config: Config,
    name: Option<String>,
    parent_id: Option<String>,
    language: Option<String>,
}

impl CreateCategoryRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            name: None,
            parent_id: None,
            language: None,
        }
    }

    /// 设置分类名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置父分类ID
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// 设置语言。
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// 构建请求体
    pub fn body(&self) -> Result<CreateCategoryBody, String> {
        let name = self.name.clone().ok_or("name is required")?;
        let parent_id = self.parent_id.clone().ok_or("parent_id is required")?;

        Ok(CreateCategoryBody {
            name,
            parent_id,
            language: self.language.clone(),
        })
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CreateCategoryResponse> {
        let body = self
            .body()
            .map_err(|reason| openlark_core::error::validation_error("body", reason))?;
        let request = CreateCategoryRequest::new(self.config.clone());
        request.execute(body).await
    }
}

/// 执行创建知识库分类
pub async fn create_category(
    config: &Config,
    body: CreateCategoryBody,
) -> SDKResult<CreateCategoryResponse> {
    create_category_with_options(config, body, RequestOption::default()).await
}

/// 执行创建知识库分类（支持自定义选项）
pub async fn create_category_with_options(
    config: &Config,
    body: CreateCategoryBody,
    option: RequestOption,
) -> SDKResult<CreateCategoryResponse> {
    body.validate()?;

    let req: ApiRequest<CreateCategoryResponse> =
        ApiRequest::post(HelpdeskApiV1::CategoryCreate.to_url())
            .body(serialize_params(&body, "创建知识库分类")?);

    Transport::request_typed(req, config, Some(option), "创建知识库分类").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_body_serialization_matches_official_schema() {
        let body = CreateCategoryBody {
            name: "技术问题".to_string(),
            parent_id: "0".to_string(),
            language: Some("zh_cn".to_string()),
        };
        assert!(body.validate().is_ok());
        assert_eq!(
            serde_json::to_value(body).expect("序列化请求体失败"),
            json!({
                "name": "技术问题",
                "parent_id": "0",
                "language": "zh_cn"
            })
        );
    }

    #[test]
    fn test_body_validation_empty_name() {
        let body = CreateCategoryBody {
            name: "".to_string(),
            parent_id: "0".to_string(),
            language: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_empty_parent_id() {
        let body = CreateCategoryBody {
            name: "技术问题".to_string(),
            parent_id: " ".to_string(),
            language: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = CreateCategoryRequestBuilder::new(config);

        assert!(builder.name.is_none());
    }

    /// 端到端：POST .../categories → 强类型响应解析。
    #[tokio::test]
    async fn test_create_category_returns_data_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/helpdesk/v1/categories"))
            .and(body_json(json!({
                "name": "技术问题",
                "parent_id": "0",
                "language": "zh_cn"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "id": "cat_new", "name": "技术问题", "parent_id": "0", "language": "zh_cn" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = CreateCategoryBody {
            name: "技术问题".to_string(),
            parent_id: "0".to_string(),
            language: Some("zh_cn".to_string()),
        };
        let resp = CreateCategoryRequest::new(config)
            .execute(body)
            .await
            .expect("创建分类应成功");
        assert_eq!(resp.id.as_deref(), Some("cat_new"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/helpdesk/v1/categories");
    }
}
