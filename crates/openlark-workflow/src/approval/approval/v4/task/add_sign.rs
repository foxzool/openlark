//! 审批任务加签（用户级，v4）
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/add_sign>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::{missing_response_data_error, request_serialization_error};

/// 审批任务加签请求体（用户级，v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddSignTaskBodyV4 {
    /// 审批实例 Code
    pub instance_code: String,
    /// 审批任务 ID
    pub task_id: String,
    /// 加签意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 加签处理人用户 ID 列表
    pub add_sign_user_ids: Vec<String>,
    /// 加签类型（前加签/后加签）
    pub add_sign_type: i32,
    /// 审批方式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_method: Option<i32>,
}

/// 审批任务加签响应（用户级，v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AddSignTaskResponseV4 {}

/// 审批任务加签请求（用户级，v4）
#[derive(Debug, Clone)]
pub struct AddSignTaskRequestV4 {
    config: Arc<Config>,
    body: AddSignTaskBodyV4,
    user_id_type: Option<String>,
}

impl AddSignTaskRequestV4 {
    /// 创建请求实例
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: AddSignTaskBodyV4::default(),
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

    /// 设置加签意见
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.body.comment = Some(comment.into());
        self
    }

    /// 添加加签处理人用户 ID
    pub fn add_sign_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.add_sign_user_ids.push(user_id.into());
        self
    }

    /// 设置加签处理人用户 ID 列表
    pub fn add_sign_user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.body.add_sign_user_ids = user_ids;
        self
    }

    /// 设置加签类型
    pub fn add_sign_type(mut self, add_sign_type: i32) -> Self {
        self.body.add_sign_type = add_sign_type;
        self
    }

    /// 设置审批方式
    pub fn approval_method(mut self, approval_method: i32) -> Self {
        self.body.approval_method = Some(approval_method);
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AddSignTaskResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<AddSignTaskResponseV4> {
        validate_required!(self.body.instance_code.trim(), "审批实例 Code 不能为空");
        validate_required!(self.body.task_id.trim(), "审批任务 ID 不能为空");
        validate_required_list!(
            self.body.add_sign_user_ids,
            1000,
            "加签处理人用户 ID 列表不能为空"
        );

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskAddSign;
        let mut request = ApiRequest::<AddSignTaskResponseV4>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("审批任务加签（用户级）", e))?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            missing_response_data_error(
                "审批任务加签（用户级）",
                response.raw_response.request_id.clone(),
            )
        })
    }
}

impl ApiResponseTrait for AddSignTaskResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_task_add_sign_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskAddSign;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/tasks/add_sign");
    }

    #[test]
    fn test_add_sign_task_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = AddSignTaskRequestV4::new(config)
            .instance_code("instance_code")
            .task_id("task_123")
            .add_sign_user_id("ou_aaa")
            .add_sign_user_id("ou_bbb")
            .add_sign_type(1)
            .approval_method(2)
            .comment("请协助审批")
            .user_id_type("open_id");

        assert_eq!(request.body.instance_code, "instance_code");
        assert_eq!(request.body.task_id, "task_123");
        assert_eq!(request.body.add_sign_user_ids, vec!["ou_aaa", "ou_bbb"]);
        assert_eq!(request.body.add_sign_type, 1);
        assert_eq!(request.body.approval_method, Some(2));
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }
}
