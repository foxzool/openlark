//! 创建推送通知
//!
//! 创建推送通知。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/notification/create

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 创建推送通知请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateNotificationBody {
    /// 标题
    pub title: String,
    /// 内容
    pub content: String,
}

impl CreateNotificationBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.title, "title is required");
        validate_required!(self.content, "content is required");
        Ok(())
    }
}

/// 创建推送通知响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNotificationResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateNotificationResult>,
}

impl openlark_core::api::ApiResponseTrait for CreateNotificationResponse {}

/// 创建推送通知结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNotificationResult {
    /// 推送ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 创建推送通知请求
#[derive(Debug, Clone)]
pub struct CreateNotificationRequest {
    config: Arc<Config>,
}

impl CreateNotificationRequest {
    /// 创建新的创建推送通知请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行创建推送通知请求
    pub async fn execute(
        self,
        body: CreateNotificationBody,
    ) -> SDKResult<CreateNotificationResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行创建推送通知请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: CreateNotificationBody,
        option: RequestOption,
    ) -> SDKResult<CreateNotificationResponse> {
        body.validate()?;

        let req: ApiRequest<CreateNotificationResponse> =
            ApiRequest::post(HelpdeskApiV1::NotificationCreate.to_url())
                .body(serialize_params(&body, "创建推送通知")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建推送通知")
    }
}

/// 创建推送通知请求构建器
#[derive(Debug, Clone)]
pub struct CreateNotificationRequestBuilder {
    config: Arc<Config>,
    title: Option<String>,
    content: Option<String>,
}

impl CreateNotificationRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            title: None,
            content: None,
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

    /// 构建请求体
    pub fn body(&self) -> Result<CreateNotificationBody, String> {
        let title = self.title.clone().ok_or("title is required")?;
        let content = self.content.clone().ok_or("content is required")?;

        Ok(CreateNotificationBody { title, content })
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CreateNotificationResponse> {
        let body = self
            .body()
            .map_err(|reason| openlark_core::error::validation_error("body", reason))?;
        let request = CreateNotificationRequest::new(self.config.clone());
        request.execute(body).await
    }
}

/// 执行创建推送通知
pub async fn create_notification(
    config: &Config,
    body: CreateNotificationBody,
) -> SDKResult<CreateNotificationResponse> {
    create_notification_with_options(config, body, RequestOption::default()).await
}

/// 执行创建推送通知（支持自定义选项）
pub async fn create_notification_with_options(
    config: &Config,
    body: CreateNotificationBody,
    option: RequestOption,
) -> SDKResult<CreateNotificationResponse> {
    body.validate()?;

    let req: ApiRequest<CreateNotificationResponse> =
        ApiRequest::post(HelpdeskApiV1::NotificationCreate.to_url())
            .body(serialize_params(&body, "创建推送通知")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "创建推送通知")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid() {
        let body = CreateNotificationBody {
            title: "系统维护通知".to_string(),
            content: "系统将于今晚进行维护...".to_string(),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_title() {
        let body = CreateNotificationBody {
            title: "".to_string(),
            content: "内容".to_string(),
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
        let builder = CreateNotificationRequestBuilder::new(Arc::new(config));

        assert!(builder.title.is_none());
    }
}
