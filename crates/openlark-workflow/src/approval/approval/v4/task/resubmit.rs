//! 重新提交审批任务（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/task/resubmit>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::{missing_response_data_error, request_serialization_error};

/// 重新提交审批任务请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct ResubmitTaskBodyV4 {
    /// 审批定义 Code
    pub approval_code: String,
    /// 审批实例 Code
    pub instance_code: String,
    /// 操作人用户 ID
    pub user_id: String,
    /// 审批任务 ID
    pub task_id: String,
    /// 重新提交意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 审批表单控件值
    pub form: String,
}

/// 重新提交审批任务响应（v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ResubmitTaskResponseV4 {}

/// 重新提交审批任务请求（v4）
#[derive(Debug, Clone)]
pub struct ResubmitTaskRequestV4 {
    config: Arc<Config>,
    body: ResubmitTaskBodyV4,
    user_id_type: Option<String>,
}

impl ResubmitTaskRequestV4 {
    /// 待补充文档。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: ResubmitTaskBodyV4::default(),
            user_id_type: None,
        }
    }

    /// 设置审批定义 Code
    pub fn approval_code(mut self, approval_code: impl Into<String>) -> Self {
        self.body.approval_code = approval_code.into();
        self
    }

    /// 设置审批实例 Code
    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.body.instance_code = instance_code.into();
        self
    }

    /// 设置操作人用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置审批任务 ID
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.body.task_id = task_id.into();
        self
    }

    /// 设置重新提交意见
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.body.comment = Some(comment.into());
        self
    }

    /// 设置审批表单值
    pub fn form(mut self, form: impl Into<String>) -> Self {
        self.body.form = form.into();
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ResubmitTaskResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ResubmitTaskResponseV4> {
        validate_required!(self.body.approval_code.trim(), "审批定义 Code 不能为空");
        validate_required!(self.body.instance_code.trim(), "审批实例 Code 不能为空");
        validate_required!(self.body.user_id.trim(), "操作人用户 ID 不能为空");
        validate_required!(self.body.task_id.trim(), "审批任务 ID 不能为空");
        validate_required!(self.body.form.trim(), "审批表单控件值不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskResubmit;
        let mut request = ApiRequest::<ResubmitTaskResponseV4>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("重新提交审批任务", e))?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            missing_response_data_error(
                "重新提交审批任务",
                response.raw_response.request_id.clone(),
            )
        })
    }
}

impl ApiResponseTrait for ResubmitTaskResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_task_resubmit_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskResubmit;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/tasks/resubmit");
    }

    #[test]
    fn test_resubmit_task_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = ResubmitTaskRequestV4::new(config)
            .approval_code("approval_code")
            .instance_code("instance_code")
            .user_id("ou_xxx")
            .task_id("task_123")
            .comment("重新提交")
            .form("[{}]")
            .user_id_type("open_id");

        assert_eq!(request.body.approval_code, "approval_code");
        assert_eq!(request.body.instance_code, "instance_code");
        assert_eq!(request.body.user_id, "ou_xxx");
        assert_eq!(request.body.task_id, "task_123");
        assert_eq!(request.body.form, "[{}]");
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }

    #[test]
    fn test_resubmit_task_response_accepts_empty_data() {
        let response: ResubmitTaskResponseV4 =
            serde_json::from_value(json!({})).expect("empty data should deserialize");
        let _ = response;
    }

    #[test]
    fn test_resubmit_task_body_contract() {
        let body = ResubmitTaskBodyV4 {
            approval_code: "approval_code".to_string(),
            instance_code: "instance_code".to_string(),
            user_id: "ou_xxx".to_string(),
            task_id: "task_123".to_string(),
            comment: Some("重新提交".to_string()),
            form: r#"[{"field":"reason","value":"补充材料"}]"#.to_string(),
        };

        let value = serde_json::to_value(&body).expect("resubmit body should serialize");
        assert_eq!(
            value,
            json!({
                "approval_code": "approval_code",
                "instance_code": "instance_code",
                "user_id": "ou_xxx",
                "task_id": "task_123",
                "comment": "重新提交",
                "form": r#"[{"field":"reason","value":"补充材料"}]"#
            })
        );
    }
}
