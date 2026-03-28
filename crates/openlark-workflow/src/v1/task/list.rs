//! 查询所有任务（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/task/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::Deserialize;
use std::sync::Arc;

/// 任务信息（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct TaskListItemV1 {
    /// 任务 ID
    pub id: String,
    /// 任务标题
    pub summary: String,
    /// 任务描述
    #[serde(default)]
    pub description: Option<String>,
    /// 任务开始时间
    #[serde(default)]
    pub start: Option<String>,
    /// 任务截止时间
    #[serde(default)]
    pub due: Option<String>,
    /// 任务优先级（1-5）
    #[serde(default)]
    pub priority: Option<i32>,
    /// 任务是否完成
    pub is_completed: bool,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 查询任务响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct ListTaskResponseV1 {
    /// 任务列表
    pub tasks: Vec<TaskListItemV1>,
    /// 分页令牌
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(default)]
    pub has_more: bool,
}

/// 查询任务请求（v1）
#[derive(Debug, Clone)]
pub struct ListTaskRequestV1 {
    config: Arc<Config>,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl ListTaskRequestV1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_token: None,
            page_size: None,
        }
    }

    /// 设置分页令牌
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置每页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListTaskResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTaskResponseV1> {
        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskList;
        let mut request = ApiRequest::<ListTaskResponseV1>::get(api_endpoint.to_url());

        // 添加查询参数
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for ListTaskResponseV1 {
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
    fn test_list_task_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = ListTaskRequestV1::new(config)
            .page_token("next_page_token")
            .page_size(20);

        assert_eq!(request.page_token, Some("next_page_token".to_string()));
        assert_eq!(request.page_size, Some(20));
    }

    #[test]
    fn test_task_api_v1_list_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskList;
        assert_eq!(endpoint.to_url(), "/open-apis/task/v1/tasks");
    }
}
