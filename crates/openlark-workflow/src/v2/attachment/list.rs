//! 列取附件
//!
//! docPath: <https://open.feishu.cn/document/task-v2/attachment/list>

use crate::common::api_endpoints::TaskApiV2;
use crate::v2::attachment::models::AttachmentInfo;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 附件列表项。
pub type AttachmentListItem = AttachmentInfo;

/// 列取附件响应
#[derive(Debug, Clone, Deserialize)]
pub struct ListAttachmentsResponse {
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,

    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,

    /// 附件列表
    #[serde(default)]
    pub items: Vec<AttachmentListItem>,
}

/// 列取附件请求
#[derive(Debug, Clone)]
pub struct ListAttachmentsRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 分页大小
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 资源类型
    resource_type: Option<String>,
    /// 资源 ID
    resource_id: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl ListAttachmentsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            resource_type: None,
            resource_id: None,
            user_id_type: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置任务 GUID（等价于设置资源 ID）。
    pub fn task_guid(mut self, task_guid: impl Into<String>) -> Self {
        self.resource_id = Some(task_guid.into());
        self
    }

    /// 设置资源类型。
    pub fn resource_type(mut self, resource_type: impl Into<String>) -> Self {
        self.resource_type = Some(resource_type.into());
        self
    }

    /// 设置资源 ID。
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListAttachmentsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListAttachmentsResponse> {
        let resource_id = self.resource_id.as_deref().unwrap_or_default();
        validate_required!(resource_id.trim(), "资源ID不能为空");

        let api_endpoint = TaskApiV2::AttachmentList;
        let mut request = ApiRequest::<ListAttachmentsResponse>::get(api_endpoint.to_url());

        // 构建查询参数
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        request = request.query("resource_id", resource_id);
        request = request.query(
            "resource_type",
            self.resource_type.as_deref().unwrap_or("task"),
        );
        if let Some(user_id_type) = &self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "列取附件",
        )
        .await
    }
}

impl ApiResponseTrait for ListAttachmentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_list_attachments_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = ListAttachmentsRequest::new(Arc::new(config))
            .page_size(20)
            .page_token("next_page_token")
            .task_guid("task_123");

        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
        assert_eq!(request.resource_id, Some("task_123".to_string()));
    }

    #[test]
    fn test_attachment_list_api_v2_url() {
        let endpoint = TaskApiV2::AttachmentList;
        assert_eq!(endpoint.to_url(), "/open-apis/task/v2/attachments");
    }
}
