//! 列取附件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/attachment/list>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 附件列表项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttachmentListItem {
    /// 附件 GUID
    pub attachment_guid: String,
    /// 任务 GUID
    pub task_guid: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小（字节）
    pub file_size: i64,
    /// 文件类型
    pub file_type: String,
    /// 上传时间
    pub created_at: String,
    /// 上传者 ID
    #[serde(default)]
    pub creator_id: Option<String>,
}

/// 列取附件响应
#[derive(Debug, Clone, Deserialize)]
pub struct ListAttachmentsResponse {
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,

    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,

    /// 总数
    #[serde(default)]
    pub total: Option<i32>,

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
    /// 任务 GUID（可选，用于筛选特定任务的附件）
    task_guid: Option<String>,
}

impl ListAttachmentsRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            task_guid: None,
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

    /// 设置任务 GUID（用于筛选特定任务的附件）
    pub fn task_guid(mut self, task_guid: impl Into<String>) -> Self {
        self.task_guid = Some(task_guid.into());
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
        let api_endpoint = TaskApiV2::AttachmentList;
        let mut request = ApiRequest::<ListAttachmentsResponse>::get(api_endpoint.to_url());

        // 构建查询参数
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(task_guid) = &self.task_guid {
            request = request.query("task_guid", task_guid);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "列取附件")
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
        assert_eq!(request.task_guid, Some("task_123".to_string()));
    }

    #[test]
    fn test_attachment_list_api_v2_url() {
        let endpoint = TaskApiV2::AttachmentList;
        assert_eq!(endpoint.to_url(), "/open-apis/task/v2/attachments");
    }
}
