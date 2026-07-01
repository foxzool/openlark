//! 同意审批任务（用户级，v4）
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/pass>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::{missing_response_data_error, request_serialization_error};

/// 同意审批任务请求体（用户级，v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct PassTaskBodyV4 {
    /// 审批实例 Code
    pub instance_code: String,
    /// 审批任务 ID
    pub task_id: String,
    /// 条件分支表单数据（仅需要填写的节点传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    /// 审批意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 同意审批任务响应（用户级，v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PassTaskResponseV4 {}

/// 同意审批任务请求（用户级，v4）
#[derive(Debug, Clone)]
pub struct PassTaskRequestV4 {
    config: Arc<Config>,
    body: PassTaskBodyV4,
    user_id_type: Option<String>,
}

impl PassTaskRequestV4 {
    /// 创建请求实例
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: PassTaskBodyV4::default(),
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

    /// 设置条件分支表单数据
    pub fn form(mut self, form: impl Into<String>) -> Self {
        self.body.form = Some(form.into());
        self
    }

    /// 设置审批意见
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
    pub async fn execute(self) -> SDKResult<PassTaskResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PassTaskResponseV4> {
        validate_required!(self.body.instance_code.trim(), "审批实例 Code 不能为空");
        validate_required!(self.body.task_id.trim(), "审批任务 ID 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskPass;
        let mut request = ApiRequest::<PassTaskResponseV4>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("同意审批任务（用户级）", e))?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            missing_response_data_error(
                "同意审批任务（用户级）",
                response.raw_response.request_id.clone(),
            )
        })
    }
}

impl ApiResponseTrait for PassTaskResponseV4 {
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
    fn test_task_pass_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskPass;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/tasks/pass");
    }

    #[test]
    fn test_pass_task_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = PassTaskRequestV4::new(config)
            .instance_code("instance_code")
            .task_id("task_123")
            .form(r#"[{"id":"x"}]"#)
            .comment("同意")
            .user_id_type("open_id");

        assert_eq!(request.body.instance_code, "instance_code");
        assert_eq!(request.body.task_id, "task_123");
        assert_eq!(request.body.comment.as_deref(), Some("同意"));
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }

    #[test]
    fn test_pass_task_body_contract() {
        let body = PassTaskBodyV4 {
            instance_code: "ins".to_string(),
            task_id: "t1".to_string(),
            form: Some(r#"[{}]"#.to_string()),
            comment: Some("同意".to_string()),
        };
        let value = serde_json::to_value(&body).expect("pass body should serialize");
        assert_eq!(
            value,
            json!({"instance_code": "ins", "task_id": "t1", "form": "[{}]", "comment": "同意"})
        );
    }
}
