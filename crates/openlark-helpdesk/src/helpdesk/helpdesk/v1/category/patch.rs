//! 更新指定知识库分类
//!
//! 更新指定知识库分类的信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/category/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;

/// 更新知识库分类请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchCategoryBody {
    /// 分类名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 父分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

impl PatchCategoryBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        if let Some(name) = &self.name {
            validate_required!(name, "name 不能为空");
        }
        if let Some(parent_id) = &self.parent_id {
            validate_required!(parent_id, "parent_id 不能为空");
        }
        Ok(())
    }
}

/// 更新知识库分类响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCategoryResponse {
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 分类名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 父分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for PatchCategoryResponse {}

/// 更新知识库分类结果。
pub type PatchCategoryResult = PatchCategoryResponse;

/// 更新知识库分类请求
#[derive(Debug, Clone)]
pub struct PatchCategoryRequest {
    config: Config,
    id: String,
}

impl PatchCategoryRequest {
    /// 创建新的更新知识库分类请求
    pub fn new(config: Config, id: String) -> Self {
        Self { config, id }
    }

    /// 执行更新知识库分类请求
    pub async fn execute(self, body: PatchCategoryBody) -> SDKResult<PatchCategoryResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行更新知识库分类请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: PatchCategoryBody,
        option: RequestOption,
    ) -> SDKResult<PatchCategoryResponse> {
        body.validate()?;

        let req: ApiRequest<PatchCategoryResponse> =
            ApiRequest::patch(HelpdeskApiV1::CategoryPatch(self.id.clone()).to_url())
                .body(serialize_params(&body, "更新知识库分类")?);

        Transport::request_typed(req, &self.config, Some(option), "更新知识库分类").await
    }
}

/// 更新知识库分类请求构建器
#[derive(Debug, Clone)]
pub struct PatchCategoryRequestBuilder {
    config: Config,
    id: String,
    name: Option<String>,
    parent_id: Option<String>,
}

impl PatchCategoryRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config, id: String) -> Self {
        Self {
            config,
            id,
            name: None,
            parent_id: None,
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

    /// 构建请求体
    pub fn body(&self) -> PatchCategoryBody {
        PatchCategoryBody {
            name: self.name.clone(),
            parent_id: self.parent_id.clone(),
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<PatchCategoryResponse> {
        let body = self.body();
        let request = PatchCategoryRequest::new(self.config.clone(), self.id.clone());
        request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<PatchCategoryResponse> {
        let body = self.body();
        let request = PatchCategoryRequest::new(self.config.clone(), self.id.clone());
        request.execute_with_options(body, option).await
    }
}

/// 执行更新知识库分类
pub async fn patch_category(
    config: &Config,
    id: String,
    body: PatchCategoryBody,
) -> SDKResult<PatchCategoryResponse> {
    patch_category_with_options(config, id, body, RequestOption::default()).await
}

/// 执行更新知识库分类（支持自定义选项）
pub async fn patch_category_with_options(
    config: &Config,
    id: String,
    body: PatchCategoryBody,
    option: RequestOption,
) -> SDKResult<PatchCategoryResponse> {
    body.validate()?;

    let req: ApiRequest<PatchCategoryResponse> =
        ApiRequest::patch(HelpdeskApiV1::CategoryPatch(id).to_url())
            .body(serialize_params(&body, "更新知识库分类")?);

    Transport::request_typed(req, config, Some(option), "更新知识库分类").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_body_validation_empty() {
        let body = PatchCategoryBody::default();
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_serialization_matches_official_schema() {
        let body = PatchCategoryBody {
            name: Some("新分类名称".to_string()),
            parent_id: Some("cat_root".to_string()),
        };
        assert!(body.validate().is_ok());
        assert_eq!(
            serde_json::to_value(body).expect("序列化请求体失败"),
            json!({
                "name": "新分类名称",
                "parent_id": "cat_root"
            })
        );
    }

    #[test]
    fn test_body_validation_empty_name() {
        let body = PatchCategoryBody {
            name: Some(" ".to_string()),
            parent_id: None,
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
        let builder = PatchCategoryRequestBuilder::new(config, "category_123".to_string());

        assert_eq!(builder.id, "category_123");
        assert!(builder.name.is_none());
    }

    /// 端到端：PATCH .../categories/{id} → 强类型响应解析。
    #[tokio::test]
    async fn test_patch_category_returns_data_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/helpdesk/v1/categories/cat_001"))
            .and(body_json(json!({
                "name": "新名称",
                "parent_id": "cat_root"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "id": "cat_001", "name": "新名称", "parent_id": "cat_root" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = PatchCategoryBody {
            name: Some("新名称".to_string()),
            parent_id: Some("cat_root".to_string()),
        };
        let resp = PatchCategoryRequest::new(config, "cat_001".to_string())
            .execute(body)
            .await
            .expect("更新分类应成功");
        assert_eq!(resp.id.as_deref(), Some("cat_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/categories/cat_001"
        );
    }
}
