//! 审批任务加签（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/task/approval-task-addsign>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::request_serialization_error;

/// 审批任务加签请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddSignBodyV4 {
    /// 审批定义 Code
    pub approval_code: String,
    /// 审批实例 Code
    pub instance_code: String,
    /// 操作用户的 user_id
    pub user_id: String,
    /// 审批任务 ID
    pub task_id: String,
    /// 审核意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 被加签人的 user_id 列表，可以指定多个
    pub add_sign_user_ids: Vec<String>,
    /// 加签方式：1 前加签、2 后加签、3 并加签
    pub add_sign_type: i32,
    /// 审批方式：1 或签、2 会签（仅前加签、后加签时填写）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_method: Option<i32>,
}

/// 审批任务加签响应（v4）
///
/// 官方响应仅包含 `code` / `msg`，无 `data` 字段。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AddSignResponseV4 {}

/// 审批任务加签请求（v4）
#[derive(Debug, Clone)]
pub struct AddSignRequestV4 {
    config: Arc<Config>,
    body: AddSignBodyV4,
}

impl AddSignRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: AddSignBodyV4::default(),
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

    /// 设置操作用户的 user_id
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置审批任务 ID
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.body.task_id = task_id.into();
        self
    }

    /// 设置审核意见
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.body.comment = Some(comment.into());
        self
    }

    /// 添加被加签人用户 ID
    pub fn add_sign_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.add_sign_user_ids.push(user_id.into());
        self
    }

    /// 设置被加签人用户 ID 列表
    pub fn add_sign_user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.body.add_sign_user_ids = user_ids;
        self
    }

    /// 设置加签方式（1 前加签、2 后加签、3 并加签）
    pub fn add_sign_type(mut self, add_sign_type: i32) -> Self {
        self.body.add_sign_type = add_sign_type;
        self
    }

    /// 设置审批方式（1 或签、2 会签）
    pub fn approval_method(mut self, approval_method: i32) -> Self {
        self.body.approval_method = Some(approval_method);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AddSignResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<AddSignResponseV4> {
        validate_required!(self.body.approval_code, "审批定义 Code 不能为空");
        validate_required!(self.body.instance_code, "审批实例 Code 不能为空");
        validate_required!(self.body.user_id, "操作用户的 user_id 不能为空");
        validate_required!(self.body.task_id, "审批任务 ID 不能为空");
        validate_required!(self.body.add_sign_user_ids, "被加签人用户 ID 列表不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceAddSign;
        let mut request = ApiRequest::<AddSignResponseV4>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("审批任务加签", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "审批任务加签",
        )
        .await
    }
}

impl ApiResponseTrait for AddSignResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 官方成功响应无 `data` 字段，显式给出空成功值
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_add_sign_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceAddSign;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances/add_sign"
        );
    }

    #[test]
    fn test_add_sign_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = AddSignRequestV4::new(config)
            .approval_code("approval_code")
            .instance_code("instance_code")
            .user_id("f7cb567e")
            .task_id("task_123")
            .comment("请协助审批")
            .add_sign_user_ids(vec!["d19b913b".to_string(), "3313g62b".to_string()])
            .add_sign_type(1)
            .approval_method(2);

        assert_eq!(request.body.approval_code, "approval_code");
        assert_eq!(request.body.instance_code, "instance_code");
        assert_eq!(request.body.user_id, "f7cb567e");
        assert_eq!(request.body.task_id, "task_123");
        assert_eq!(request.body.comment.as_deref(), Some("请协助审批"));
        assert_eq!(
            request.body.add_sign_user_ids,
            vec!["d19b913b".to_string(), "3313g62b".to_string()]
        );
        assert_eq!(request.body.add_sign_type, 1);
        assert_eq!(request.body.approval_method, Some(2));
    }
}
