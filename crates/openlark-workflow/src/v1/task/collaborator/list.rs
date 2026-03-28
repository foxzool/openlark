//! 获取任务协作者列表（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskcollaborator/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::Deserialize;
use std::sync::Arc;

/// 任务协作者列表项（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct TaskCollaboratorItemV1 {
    /// 协作者用户 ID
    pub collaborator_id: String,
    /// 协作者名称
    pub name: Option<String>,
}

/// 获取任务协作者列表响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct ListTaskCollaboratorResponseV1 {
    /// 协作者列表
    pub items: Vec<TaskCollaboratorItemV1>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
    /// 翻页标记
    pub page_token: Option<String>,
}

/// 获取任务协作者列表请求（v1）
#[derive(Debug, Clone)]
pub struct ListTaskCollaboratorRequestV1 {
    config: Arc<Config>,
    task_id: String,
    /// 每页数量
    page_size: Option<i32>,
    /// 翻页标记
    page_token: Option<String>,
}

impl ListTaskCollaboratorRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置翻页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListTaskCollaboratorResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTaskCollaboratorResponseV1> {
        let api_endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskCollaboratorList(self.task_id.clone());
        let mut request = ApiRequest::<ListTaskCollaboratorResponseV1>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for ListTaskCollaboratorResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_list_task_collaborator_v1_url() {
        let endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskCollaboratorList("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/collaborators"
        );
    }
}
