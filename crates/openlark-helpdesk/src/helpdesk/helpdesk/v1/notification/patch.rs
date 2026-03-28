//! 更新指定推送通知
//!
//! 更新指定推送通知的信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/notification/patch>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 更新推送通知请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchNotificationBody {
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl PatchNotificationBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if let Some(title) = &self.title {
            if title.is_empty() {
                return Err("title cannot be empty".to_string());
            }
        }
        if let Some(content) = &self.content {
            if content.is_empty() {
                return Err("content cannot be empty".to_string());
            }
        }
        Ok(())
    }
}

/// 更新推送通知响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PatchNotificationResult>,
}

impl openlark_core::api::ApiResponseTrait for PatchNotificationResponse {}

/// 更新推送通知结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchNotificationResult {
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

/// 更新推送通知请求
#[derive(Debug, Clone)]
pub struct PatchNotificationRequest {
    config: Arc<Config>,
    notification_id: String,
}

impl PatchNotificationRequest {
    /// 创建新的更新推送通知请求
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
        }
    }

    /// 执行更新推送通知请求
    pub async fn execute(
        self,
        body: PatchNotificationBody,
    ) -> SDKResult<PatchNotificationResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行更新推送通知请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: PatchNotificationBody,
        option: RequestOption,
    ) -> SDKResult<PatchNotificationResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<PatchNotificationResponse> = ApiRequest::patch(
            HelpdeskApiV1::NotificationPatch(self.notification_id.clone()).to_url(),
        )
        .body(serialize_params(&body, "更新推送通知")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新推送通知")
    }
}

/// 更新推送通知请求构建器
#[derive(Debug, Clone)]
pub struct PatchNotificationRequestBuilder {
    config: Arc<Config>,
    notification_id: String,
    title: Option<String>,
    content: Option<String>,
}

impl PatchNotificationRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
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
    pub fn body(&self) -> PatchNotificationBody {
        PatchNotificationBody {
            title: self.title.clone(),
            content: self.content.clone(),
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<PatchNotificationResponse> {
        let body = self.body();
        let request =
            PatchNotificationRequest::new(self.config.clone(), self.notification_id.clone());
        request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<PatchNotificationResponse> {
        let body = self.body();
        let request =
            PatchNotificationRequest::new(self.config.clone(), self.notification_id.clone());
        request.execute_with_options(body, option).await
    }
}

/// 执行更新推送通知
pub async fn patch_notification(
    config: &Config,
    notification_id: String,
    body: PatchNotificationBody,
) -> SDKResult<PatchNotificationResponse> {
    patch_notification_with_options(config, notification_id, body, RequestOption::default()).await
}

/// 执行更新推送通知（支持自定义选项）
pub async fn patch_notification_with_options(
    config: &Config,
    notification_id: String,
    body: PatchNotificationBody,
    option: RequestOption,
) -> SDKResult<PatchNotificationResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<PatchNotificationResponse> =
        ApiRequest::patch(HelpdeskApiV1::NotificationPatch(notification_id).to_url())
            .body(serialize_params(&body, "更新推送通知")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "更新推送通知")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_empty() {
        let body = PatchNotificationBody::default();
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = PatchNotificationBody {
            title: Some("新标题".to_string()),
            content: Some("新内容".to_string()),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_title() {
        let body = PatchNotificationBody {
            title: Some("".to_string()),
            content: None,
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
            PatchNotificationRequestBuilder::new(Arc::new(config), "notif_123".to_string());

        assert_eq!(builder.notification_id, "notif_123");
        assert!(builder.title.is_none());
    }
}
