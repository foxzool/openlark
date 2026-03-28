//! 创建知识库分类
//!
//! 创建新的知识库分类。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/category/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 创建知识库分类请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCategoryBody {
    /// 分类名称
    pub name: String,
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

impl CreateCategoryBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("name is required".to_string());
        }
        Ok(())
    }
}

/// 创建知识库分类响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateCategoryResult>,
}

impl openlark_core::api::ApiResponseTrait for CreateCategoryResponse {}

/// 创建知识库分类结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategoryResult {
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

/// 创建知识库分类请求
#[derive(Debug, Clone)]
pub struct CreateCategoryRequest {
    config: Arc<Config>,
}

impl CreateCategoryRequest {
    /// 创建新的创建知识库分类请求
    pub fn new(config: Arc<Config>) -> Self {
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
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<CreateCategoryResponse> =
            ApiRequest::post(HelpdeskApiV1::CategoryCreate.to_url())
                .body(serialize_params(&body, "创建知识库分类")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建知识库分类")
    }
}

/// 创建知识库分类请求构建器
#[derive(Debug, Clone)]
pub struct CreateCategoryRequestBuilder {
    config: Arc<Config>,
    name: Option<String>,
    description: Option<String>,
    parent_id: Option<String>,
    order: Option<i32>,
}

impl CreateCategoryRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
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
    pub fn body(&self) -> Result<CreateCategoryBody, String> {
        let name = self.name.clone().ok_or("name is required")?;

        Ok(CreateCategoryBody {
            name,
            description: self.description.clone(),
            parent_id: self.parent_id.clone(),
            order: self.order,
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
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<CreateCategoryResponse> =
        ApiRequest::post(HelpdeskApiV1::CategoryCreate.to_url())
            .body(serialize_params(&body, "创建知识库分类")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "创建知识库分类")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid() {
        let body = CreateCategoryBody {
            name: "技术问题".to_string(),
            description: Some("技术相关问题分类".to_string()),
            parent_id: None,
            order: Some(1),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_name() {
        let body = CreateCategoryBody {
            name: "".to_string(),
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
        let builder = CreateCategoryRequestBuilder::new(Arc::new(config));

        assert!(builder.name.is_none());
    }
}
