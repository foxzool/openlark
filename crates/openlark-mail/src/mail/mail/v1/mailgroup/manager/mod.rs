//! 邮件组管理员模块

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ===== 数据模型 =====

/// 批量创建邮件组管理员请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchCreateMailGroupManagerBody {
    /// 管理员邮箱列表
    pub managers: Vec<String>,
}

/// 批量创建邮件组管理员响应
#[derive(Debug, Clone, Deserialize)]
pub struct BatchCreateMailGroupManagerResponse {
    /// 是否创建成功
    pub success: bool,
}

/// 邮件组管理员列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct MailGroupManagerListResponse {
    /// 管理员列表
    pub items: Vec<MailGroupManagerItem>,
    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 邮件组管理员项目
#[derive(Debug, Clone, Deserialize)]
pub struct MailGroupManagerItem {
    /// 管理员邮箱
    pub email: String,
    /// 添加时间
    pub create_time: String,
}

/// 批量删除邮件组管理员请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchDeleteMailGroupManagerBody {
    /// 管理员邮箱列表
    pub managers: Vec<String>,
}

/// 批量删除邮件组管理员响应
#[derive(Debug, Clone, Deserialize)]
pub struct BatchDeleteMailGroupManagerResponse {
    /// 是否删除成功
    pub success: bool,
}

/// Manager：邮件组管理员资源
#[derive(Clone)]
pub struct Manager {
    config: Arc<Config>,
    group_id: String,
}

impl Manager {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, group_id: String) -> Self {
        Self { config, group_id }
    }

    /// 批量创建管理员
    pub fn batch_create(&self) -> BatchCreateMailGroupManagerRequest {
        BatchCreateMailGroupManagerRequest::new(self.config.clone(), self.group_id.clone())
    }

    /// 批量删除管理员
    pub fn batch_delete(&self) -> BatchDeleteMailGroupManagerRequest {
        BatchDeleteMailGroupManagerRequest::new(self.config.clone(), self.group_id.clone())
    }

    /// 获取管理员列表
    pub fn list(&self) -> MailGroupManagerListRequest {
        MailGroupManagerListRequest::new(self.config.clone(), self.group_id.clone())
    }
}

/// 批量创建邮件组管理员请求
#[derive(Debug, Clone)]
pub struct BatchCreateMailGroupManagerRequest {
    config: Arc<Config>,
    group_id: String,
    body: BatchCreateMailGroupManagerBody,
}

impl BatchCreateMailGroupManagerRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, group_id: impl Into<String>) -> Self {
        Self {
            config,
            group_id: group_id.into(),
            body: BatchCreateMailGroupManagerBody::default(),
        }
    }

    /// 设置管理员邮箱列表
    pub fn managers(mut self, managers: Vec<String>) -> Self {
        self.body.managers = managers;
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<BatchCreateMailGroupManagerResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchCreateMailGroupManagerResponse> {
        let api_endpoint = MailApiV1::MailGroupManagerBatchCreate(self.group_id.clone());
        let mut request =
            ApiRequest::<BatchCreateMailGroupManagerResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "批量创建邮件组管理员")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "批量创建邮件组管理员")
    }
}

impl ApiResponseTrait for BatchCreateMailGroupManagerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除邮件组管理员请求
#[derive(Debug, Clone)]
pub struct BatchDeleteMailGroupManagerRequest {
    config: Arc<Config>,
    group_id: String,
    body: BatchDeleteMailGroupManagerBody,
}

impl BatchDeleteMailGroupManagerRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, group_id: impl Into<String>) -> Self {
        Self {
            config,
            group_id: group_id.into(),
            body: BatchDeleteMailGroupManagerBody::default(),
        }
    }

    /// 设置管理员邮箱列表
    pub fn managers(mut self, managers: Vec<String>) -> Self {
        self.body.managers = managers;
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<BatchDeleteMailGroupManagerResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchDeleteMailGroupManagerResponse> {
        let api_endpoint = MailApiV1::MailGroupManagerBatchDelete(self.group_id.clone());
        let mut request =
            ApiRequest::<BatchDeleteMailGroupManagerResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "批量删除邮件组管理员")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "批量删除邮件组管理员")
    }
}

impl ApiResponseTrait for BatchDeleteMailGroupManagerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 邮件组管理员列表请求
#[derive(Debug, Clone)]
pub struct MailGroupManagerListRequest {
    config: Arc<Config>,
    group_id: String,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl MailGroupManagerListRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, group_id: impl Into<String>) -> Self {
        Self {
            config,
            group_id: group_id.into(),
            page_token: None,
            page_size: None,
        }
    }

    /// page_token。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// page_size。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<MailGroupManagerListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MailGroupManagerListResponse> {
        let api_endpoint = MailApiV1::MailGroupManagerList(self.group_id.clone());
        let mut request = ApiRequest::<MailGroupManagerListResponse>::get(api_endpoint.to_url());

        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = &self.page_size {
            request = request.query("page_size", page_size.to_string());
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取邮件组管理员列表")
    }
}

impl ApiResponseTrait for MailGroupManagerListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_create_mail_group_manager_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = BatchCreateMailGroupManagerRequest::new(config, "group_123".to_string())
            .managers(vec![
                "admin1@example.com".to_string(),
                "admin2@example.com".to_string(),
            ]);

        assert_eq!(request.group_id, "group_123");
        assert_eq!(request.body.managers.len(), 2);
    }
}

pub mod batch_create;
pub mod batch_delete;
pub mod list;
