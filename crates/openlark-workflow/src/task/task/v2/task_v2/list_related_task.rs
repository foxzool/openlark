//! 列取与我相关的任务。
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task_v2/list_related_task>

use crate::common::{TaskV2Endpoint, api_utils::*};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};

/// 与当前调用身份相关的任务。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelatedTaskItem {
    /// 任务 GUID。
    pub guid: String,
    /// 任务标题。
    #[serde(default)]
    pub summary: Option<String>,
    /// 任务描述。
    #[serde(default)]
    pub description: Option<String>,
    /// 任务状态。
    #[serde(default)]
    pub status: Option<String>,
    /// 父任务 GUID。
    #[serde(default)]
    pub parent_task_guid: Option<String>,
    /// 任务分享链接。
    #[serde(default)]
    pub url: Option<String>,
    /// 其他暂未建模的官方字段。
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_json::Value>,
}

/// 列取与我相关的任务响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRelatedTaskResponse {
    /// 返回的任务列表。
    #[serde(default)]
    pub items: Vec<RelatedTaskItem>,
    /// 分页标记。
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否还有更多项。
    #[serde(default)]
    pub has_more: bool,
}

/// 列取与我相关的任务请求。
#[derive(Debug, Clone)]
pub struct ListRelatedTaskRequest {
    config: Arc<Config>,
    page_size: Option<i32>,
    page_token: Option<String>,
    completed: Option<bool>,
    user_id_type: Option<String>,
}

impl ListRelatedTaskRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            completed: None,
            user_id_type: None,
        }
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置是否只返回已完成或未完成任务。
    pub fn completed(mut self, completed: bool) -> Self {
        self.completed = Some(completed);
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListRelatedTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListRelatedTaskResponse> {
        let api_endpoint = TaskV2Endpoint::ListRelatedTask;
        let mut request = ApiRequest::<ListRelatedTaskResponse>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(completed) = self.completed {
            request = request.query("completed", completed.to_string());
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "列取与我相关的任务")
    }
}

impl ApiResponseTrait for ListRelatedTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_194_list_related_task_builder() {
        let request = ListRelatedTaskRequest::new(Arc::new(Config::default()))
            .page_size(10)
            .page_token("token_123")
            .completed(false)
            .user_id_type("open_id");

        assert_eq!(request.page_size, Some(10));
        assert_eq!(request.page_token.as_deref(), Some("token_123"));
        assert_eq!(request.completed, Some(false));
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }
}
