//! 获取推送通知列表
//!
//! 获取推送通知列表。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/notification/list

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 获取推送通知列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListNotificationResponse {
    /// 推送通知列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<NotificationItem>>,
}

impl ApiResponseTrait for ListNotificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 推送通知项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationItem {
    /// 推送ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// 获取推送通知列表请求
#[derive(Debug, Clone)]
pub struct ListNotificationRequest {
    config: Arc<Config>,
}

impl ListNotificationRequest {
    /// 创建新的获取推送通知列表请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取推送通知列表请求
    pub async fn execute(self) -> SDKResult<ListNotificationResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListNotificationResponse> {
        let api_endpoint = HelpdeskApiV1::NotificationList;
        let request = ApiRequest::<ListNotificationResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取推送通知列表")
    }
}

/// 获取推送通知列表请求构建器
#[derive(Debug, Clone)]
pub struct ListNotificationRequestBuilder {
    config: Arc<Config>,
}

impl ListNotificationRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<ListNotificationResponse> {
        let api_endpoint = HelpdeskApiV1::NotificationList;
        let request = ApiRequest::<ListNotificationResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取推送通知列表")
    }
}

/// 执行获取推送通知列表
pub async fn list_notifications(config: &Config) -> SDKResult<ListNotificationResponse> {
    let api_endpoint = HelpdeskApiV1::NotificationList;
    let request = ApiRequest::<ListNotificationResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取推送通知列表")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let _builder = ListNotificationRequestBuilder::new(Arc::new(config));
    }
}
