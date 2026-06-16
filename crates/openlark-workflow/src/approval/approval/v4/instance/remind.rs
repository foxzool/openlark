//! 单据催办（用户级，v4）
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/remind

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::{missing_response_data_error, request_serialization_error};

/// 单据催办请求体（用户级，v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct RemindInstanceBodyV4 {
    /// 审批实例 Code
    pub instance_code: String,
    /// 催办任务 ID 列表
    pub task_ids: Vec<String>,
    /// 催办留言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 单据催办响应（用户级，v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RemindInstanceResponseV4 {}

/// 单据催办请求（用户级，v4）
#[derive(Debug, Clone)]
pub struct RemindInstanceRequestV4 {
    config: Arc<Config>,
    body: RemindInstanceBodyV4,
    user_id_type: Option<String>,
}

impl RemindInstanceRequestV4 {
    /// 创建请求实例
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: RemindInstanceBodyV4::default(),
            user_id_type: None,
        }
    }

    /// 设置审批实例 Code
    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.body.instance_code = instance_code.into();
        self
    }

    /// 添加催办任务 ID
    pub fn add_task_id(mut self, task_id: impl Into<String>) -> Self {
        self.body.task_ids.push(task_id.into());
        self
    }

    /// 设置催办任务 ID 列表
    pub fn task_ids(mut self, task_ids: Vec<String>) -> Self {
        self.body.task_ids = task_ids;
        self
    }

    /// 设置催办留言
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
    pub async fn execute(self) -> SDKResult<RemindInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RemindInstanceResponseV4> {
        validate_required!(self.body.instance_code.trim(), "审批实例 Code 不能为空");
        validate_required_list!(self.body.task_ids, 1000, "催办任务 ID 列表不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceRemind;
        let mut request = ApiRequest::<RemindInstanceResponseV4>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("单据催办（用户级）", e))?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            missing_response_data_error(
                "单据催办（用户级）",
                response.raw_response.request_id.clone(),
            )
        })
    }
}

impl ApiResponseTrait for RemindInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_remind_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceRemind;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/instances/remind");
    }

    #[test]
    fn test_remind_instance_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = RemindInstanceRequestV4::new(config)
            .instance_code("instance_code")
            .add_task_id("task_1")
            .add_task_id("task_2")
            .comment("请尽快处理")
            .user_id_type("open_id");

        assert_eq!(request.body.instance_code, "instance_code");
        assert_eq!(request.body.task_ids, vec!["task_1", "task_2"]);
        assert_eq!(request.body.comment.as_deref(), Some("请尽快处理"));
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }
}
