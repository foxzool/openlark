//! 创建知识库
//!
//! 创建新的知识库。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 创建知识库请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFaqBody {
    /// 标题
    pub title: String,
    /// 内容
    pub content: String,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
}

impl CreateFaqBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            return Err("title is required".to_string());
        }
        if self.content.is_empty() {
            return Err("content is required".to_string());
        }
        Ok(())
    }
}

/// 创建知识库响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFaqResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateFaqResult>,
}

impl openlark_core::api::ApiResponseTrait for CreateFaqResponse {}

/// 创建知识库结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFaqResult {
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

/// 创建知识库请求
#[derive(Debug, Clone)]
pub struct CreateFaqRequest {
    config: Arc<Config>,
}

impl CreateFaqRequest {
    /// 创建新的创建知识库请求
    pub fn new(config: Arc<Config>) -> Self {
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
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<CreateFaqResponse> =
            ApiRequest::post(HelpdeskApiV1::FaqCreate.to_url())
                .body(serialize_params(&body, "创建知识库")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建知识库")
    }
}

/// 创建知识库请求构建器
#[derive(Debug, Clone)]
pub struct CreateFaqRequestBuilder {
    config: Arc<Config>,
    title: Option<String>,
    content: Option<String>,
    category_id: Option<String>,
}

impl CreateFaqRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
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
    pub fn body(&self) -> Result<CreateFaqBody, String> {
        let title = self.title.clone().ok_or("title is required")?;
        let content = self.content.clone().ok_or("content is required")?;

        Ok(CreateFaqBody {
            title,
            content,
            category_id: self.category_id.clone(),
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
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<CreateFaqResponse> = ApiRequest::post(HelpdeskApiV1::FaqCreate.to_url())
        .body(serialize_params(&body, "创建知识库")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "创建知识库")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid() {
        let body = CreateFaqBody {
            title: "如何重置密码？".to_string(),
            content: "请按照以下步骤重置密码...".to_string(),
            category_id: Some("cat_123".to_string()),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_title() {
        let body = CreateFaqBody {
            title: "".to_string(),
            content: "内容".to_string(),
            category_id: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_empty_content() {
        let body = CreateFaqBody {
            title: "标题".to_string(),
            content: "".to_string(),
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
        let builder = CreateFaqRequestBuilder::new(Arc::new(config));

        assert!(builder.title.is_none());
    }
}
