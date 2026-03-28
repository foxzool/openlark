//! 同意审批任务（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/task/approve>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 同意审批任务请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct ApproveTaskBodyV4 {
    /// 审批任务 ID
    pub task_id: String,
    /// 意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 同意审批任务响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct ApproveTaskResponseV4 {
    /// 是否成功
    pub success: bool,
}

/// 同意审批任务请求（v4）
#[derive(Debug, Clone)]
pub struct ApproveTaskRequestV4 {
    config: Arc<Config>,
    body: ApproveTaskBodyV4,
}

impl ApproveTaskRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: ApproveTaskBodyV4::default(),
        }
    }

    /// 设置审批任务 ID
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.body.task_id = task_id.into();
        self
    }

    /// 设置意见
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.body.comment = Some(comment.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ApproveTaskResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ApproveTaskResponseV4> {
        validate_required!(self.body.task_id.trim(), "审批任务 ID 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskApprove;
        let mut request = ApiRequest::<ApproveTaskResponseV4>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("序列化请求体失败", e.to_string().as_str())
        })?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for ApproveTaskResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_task_approve_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskApprove;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/tasks/approve");
    }
}
