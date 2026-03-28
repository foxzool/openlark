//! 查询用户的任务列表（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/approval-search/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 审批任务列表项（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct TaskItemV4 {
    /// 任务 ID
    pub task_id: String,
    /// 审批实例 Code
    pub instance_code: String,
    /// 任务状态
    pub status: String,
}

/// 查询用户的任务列表响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct QueryTaskResponseV4 {
    /// 任务列表
    pub items: Vec<TaskItemV4>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

/// 查询用户的任务列表请求（v4）
#[derive(Debug, Clone)]
pub struct QueryTaskRequestV4 {
    config: Arc<Config>,
}

impl QueryTaskRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryTaskResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryTaskResponseV4> {
        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskQuery;
        let request = ApiRequest::<QueryTaskResponseV4>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for QueryTaskResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_task_query_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskQuery;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/tasks/query");
    }
}
