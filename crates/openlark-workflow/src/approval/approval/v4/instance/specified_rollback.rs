//! 退回审批任务（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/task/specified_rollback>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::request_serialization_error;

/// 退回审批任务请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct SpecifiedRollbackBodyV4 {
    /// 当前审批任务的审批人的用户 ID
    pub user_id: String,
    /// 当前需要回退的审批任务 ID
    pub task_id: String,
    /// 退回原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 扩展字段（灰度参数，暂未开放使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 需要退回到的任务 node_key 列表（1~100 个）
    pub task_def_key_list: Vec<String>,
}

/// 退回审批任务响应（v4）
///
/// 官方响应仅包含 `code` / `msg`，`data` 为空对象。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SpecifiedRollbackResponseV4 {}

/// 退回审批任务请求（v4）
#[derive(Debug, Clone)]
pub struct SpecifiedRollbackRequestV4 {
    config: Arc<Config>,
    body: SpecifiedRollbackBodyV4,
    user_id_type: Option<String>,
}

impl SpecifiedRollbackRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: SpecifiedRollbackBodyV4::default(),
            user_id_type: None,
        }
    }

    /// 设置当前审批任务的审批人的用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置当前需要回退的审批任务 ID
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.body.task_id = task_id.into();
        self
    }

    /// 设置退回原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.body.reason = Some(reason.into());
        self
    }

    /// 设置扩展字段（灰度参数，暂未开放使用）
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.body.extra = Some(extra.into());
        self
    }

    /// 设置需要退回到的任务 node_key 列表
    pub fn task_def_key_list(mut self, task_def_key_list: Vec<String>) -> Self {
        self.body.task_def_key_list = task_def_key_list;
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SpecifiedRollbackResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SpecifiedRollbackResponseV4> {
        validate_required!(self.body.user_id, "审批人的用户 ID 不能为空");
        validate_required!(self.body.task_id, "审批任务 ID 不能为空");
        validate_required_list!(
            self.body.task_def_key_list,
            100,
            "退回目标 node_key 列表不能为空且不能超过 100 个"
        );

        // 端点无路径参数；枚举变体占位参数不影响 URL
        let api_endpoint =
            crate::common::api_endpoints::ApprovalApiV4::InstanceSpecifiedRollback(String::new());
        let mut request = ApiRequest::<SpecifiedRollbackResponseV4>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("退回审批任务", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "退回审批任务",
        )
        .await
    }
}

impl ApiResponseTrait for SpecifiedRollbackResponseV4 {
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
    fn test_instance_specified_rollback_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceSpecifiedRollback(
            "test_instance_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances/specified_rollback"
        );
    }

    #[test]
    fn test_specified_rollback_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = SpecifiedRollbackRequestV4::new(config)
            .user_id("893g4c45")
            .task_id("7026591166355210260")
            .reason("申请事项填写不具体，请重新填写")
            .extra("demo")
            .task_def_key_list(vec![
                "START".to_string(),
                "APPROVAL_27997_285502".to_string(),
            ]);

        assert_eq!(request.body.user_id, "893g4c45");
        assert_eq!(request.body.task_id, "7026591166355210260");
        assert_eq!(
            request.body.reason.as_deref(),
            Some("申请事项填写不具体，请重新填写")
        );
        assert_eq!(request.body.extra.as_deref(), Some("demo"));
        assert_eq!(
            request.body.task_def_key_list,
            vec!["START", "APPROVAL_27997_285502"]
        );
    }
}
