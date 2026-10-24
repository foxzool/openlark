//! 转交审批任务（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/task/transfer>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::request_serialization_error;

/// 转交审批任务请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct TransferTaskBodyV4 {
    /// 审批定义 Code
    pub approval_code: String,
    /// 审批实例 Code
    pub instance_code: String,
    /// 当前审批人的用户 ID
    pub user_id: String,
    /// 审批意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 被转交人的用户 ID
    pub transfer_user_id: String,
    /// 审批任务 ID
    pub task_id: String,
}

/// 转交审批任务响应（v4）
///
/// 官方响应仅包含 `code` / `msg`，`data` 为空对象。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct TransferTaskResponseV4 {}

/// 转交审批任务请求（v4）
#[derive(Debug, Clone)]
pub struct TransferTaskRequestV4 {
    config: Arc<Config>,
    body: TransferTaskBodyV4,
    user_id_type: Option<String>,
}

impl TransferTaskRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: TransferTaskBodyV4::default(),
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

    /// 设置当前审批人的用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置审批意见
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.body.comment = Some(comment.into());
        self
    }

    /// 设置被转交人的用户 ID
    pub fn transfer_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.transfer_user_id = user_id.into();
        self
    }

    /// 设置审批任务 ID
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.body.task_id = task_id.into();
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TransferTaskResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<TransferTaskResponseV4> {
        validate_required!(self.body.approval_code, "审批定义 Code 不能为空");
        validate_required!(self.body.instance_code, "审批实例 Code 不能为空");
        validate_required!(self.body.user_id, "当前审批人的用户 ID 不能为空");
        validate_required!(self.body.transfer_user_id, "被转交人的用户 ID 不能为空");
        validate_required!(self.body.task_id, "审批任务 ID 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskTransfer;
        let mut request = ApiRequest::<TransferTaskResponseV4>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("转交审批任务", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "转交审批任务",
        )
        .await
    }
}

impl ApiResponseTrait for TransferTaskResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 官方成功响应 `data` 为空对象（或缺省），显式给出空成功值
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_task_transfer_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskTransfer;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/tasks/transfer");
    }

    #[test]
    fn test_transfer_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = TransferTaskRequestV4::new(config)
            .approval_code("7C468A54-8745-2245-9675-08B7C63E7A85")
            .instance_code("81D31358-93AF-92D6-7425-01A5D67C4E71")
            .user_id("f7cb567e")
            .comment("OK")
            .transfer_user_id("f4ip317q")
            .task_id("12345");

        assert_eq!(
            request.body.approval_code,
            "7C468A54-8745-2245-9675-08B7C63E7A85"
        );
        assert_eq!(
            request.body.instance_code,
            "81D31358-93AF-92D6-7425-01A5D67C4E71"
        );
        assert_eq!(request.body.user_id, "f7cb567e");
        assert_eq!(request.body.comment.as_deref(), Some("OK"));
        assert_eq!(request.body.transfer_user_id, "f4ip317q");
        assert_eq!(request.body.task_id, "12345");
    }
}
