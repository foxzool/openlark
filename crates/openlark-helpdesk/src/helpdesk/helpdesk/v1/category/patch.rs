//! 更新指定知识库分类
//!
//! 更新指定知识库分类的信息。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/category/patch

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 更新知识库分类请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchCategoryBody {
    /// 分类名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 分类描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 父分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

impl PatchCategoryBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        if let Some(name) = &self.name
            && name.is_empty()
        {
            return Err(openlark_core::CoreError::validation_msg(
                "name cannot be empty",
            ));
        }
        Ok(())
    }
}

/// 更新知识库分类响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCategoryResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PatchCategoryResult>,
}

impl openlark_core::api::ApiResponseTrait for PatchCategoryResponse {}

/// 更新知识库分类结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCategoryResult {
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 分类名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 分类描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 父分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// 更新知识库分类请求
#[derive(Debug, Clone)]
pub struct PatchCategoryRequest {
    config: Arc<Config>,
    id: String,
}

impl PatchCategoryRequest {
    /// 创建新的更新知识库分类请求
    pub fn new(config: Arc<Config>, id: String) -> Self {
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

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新知识库分类")
    }
}

/// 更新知识库分类请求构建器
#[derive(Debug, Clone)]
pub struct PatchCategoryRequestBuilder {
    config: Arc<Config>,
    id: String,
    name: Option<String>,
    description: Option<String>,
    parent_id: Option<String>,
    order: Option<i32>,
}

impl PatchCategoryRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self {
            config,
            id,
            name: None,
            description: None,
            parent_id: None,
            order: None,
        }
    }

    /// 设置分类名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置分类描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置父分类ID
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// 设置排序
    pub fn order(mut self, order: i32) -> Self {
        self.order = Some(order);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> PatchCategoryBody {
        PatchCategoryBody {
            name: self.name.clone(),
            description: self.description.clone(),
            parent_id: self.parent_id.clone(),
            order: self.order,
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

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "更新知识库分类")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_empty() {
        let body = PatchCategoryBody::default();
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = PatchCategoryBody {
            name: Some("新分类名称".to_string()),
            description: Some("更新描述".to_string()),
            parent_id: None,
            order: Some(1),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_name() {
        let body = PatchCategoryBody {
            name: Some("".to_string()),
            description: None,
            parent_id: None,
            order: None,
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
        let builder =
            PatchCategoryRequestBuilder::new(Arc::new(config), "category_123".to_string());

        assert_eq!(builder.id, "category_123");
        assert!(builder.name.is_none());
    }
}
