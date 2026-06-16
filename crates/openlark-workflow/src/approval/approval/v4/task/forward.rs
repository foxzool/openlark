//! 转交审批任务（用户级，v4）
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/forward

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::{missing_response_data_error, request_serialization_error};

/// 转交审批任务请求体（用户级，v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct ForwardTaskBodyV4 {
    /// 审批实例 Code
    pub instance_code: String,
    /// 审批任务 ID
    pub task_id: String,
    /// 转交目标用户 ID
    pub transfer_user_id: String,
    /// 转交原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 转交审批任务响应（用户级，v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ForwardTaskResponseV4 {}

/// 转交审批任务请求（用户级，v4）
#[derive(Debug, Clone)]
pub struct ForwardTaskRequestV4 {
    config: Arc<Config>,
    body: ForwardTaskBodyV4,
    user_id_type: Option<String>,
}

impl ForwardTaskRequestV4 {
    /// 创建请求实例
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: ForwardTaskBodyV4::default(),
            user_id_type: None,
        }
    }

    /// 设置审批实例 Code
    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.body.instance_code = instance_code.into();
        self
    }

    /// 设置审批任务 ID
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.body.task_id = task_id.into();
        self
    }

    /// 设置转交目标用户 ID
    pub fn transfer_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.transfer_user_id = user_id.into();
        self
    }

    /// 设置转交原因
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.body.comment = Some(comment.into());
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ForwardTaskResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ForwardTaskResponseV4> {
        validate_required!(self.body.instance_code.trim(), "审批实例 Code 不能为空");
        validate_required!(self.body.task_id.trim(), "审批任务 ID 不能为空");
        validate_required!(
            self.body.transfer_user_id.trim(),
            "转交目标用户 ID 不能为空"
        );

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskForward;
        let mut request = ApiRequest::<ForwardTaskResponseV4>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("转交审批任务（用户级）", e))?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            missing_response_data_error(
                "转交审批任务（用户级）",
                response.raw_response.request_id.clone(),
            )
        })
    }
}

impl ApiResponseTrait for ForwardTaskResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_task_forward_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskForward;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/tasks/forward");
    }

    #[test]
    fn test_forward_task_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = ForwardTaskRequestV4::new(config)
            .instance_code("instance_code")
            .task_id("task_123")
            .transfer_user_id("ou_yyy")
            .comment("转交原因")
            .user_id_type("open_id");

        assert_eq!(request.body.instance_code, "instance_code");
        assert_eq!(request.body.task_id, "task_123");
        assert_eq!(request.body.transfer_user_id, "ou_yyy");
        assert_eq!(request.body.comment.as_deref(), Some("转交原因"));
    }
}
