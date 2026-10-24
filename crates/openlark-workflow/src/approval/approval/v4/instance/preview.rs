//! 预览审批流程（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/approval-preview>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::request_serialization_error;

/// 预览审批流程请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct PreviewInstanceBodyV4 {
    /// 审批定义 Code（创建实例前预览时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    /// 用户 ID：创建实例前预览传审批发起人 ID；创建后预览传审批任务审批人 ID
    pub user_id: String,
    /// 审批发起人所属的部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 审批表单的控件 JSON 值（JSON 数组压缩转义后的字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    /// 审批实例 Code（创建实例后预览后续流程时填写）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    /// 审批任务 ID（创建实例后预览某任务后续流程时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// 预览节点信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PreviewInstanceNodeV4 {
    /// 审批人 ID 列表
    #[serde(default)]
    pub user_id_list: Vec<String>,
    /// 审批结束抄送人 ID 列表
    #[serde(default)]
    pub end_cc_id_list: Vec<String>,
    /// 审批节点 ID
    #[serde(default)]
    pub node_id: String,
    /// 审批节点名称
    #[serde(default)]
    pub node_name: String,
    /// 审批节点类型（AND/OR/AUTO_PASS/AUTO_REJECT/SEQUENTIAL）
    #[serde(default)]
    pub node_type: String,
    /// 用户自定义节点 ID
    #[serde(default)]
    pub custom_node_id: String,
    /// 节点的说明信息
    #[serde(default)]
    pub comments: Vec<serde_json::Value>,
    /// 审批人是否为空（为空时 user_id_list 为兜底审批人 ID 列表）
    #[serde(default)]
    pub is_empty_logic: bool,
    /// 是否发起人自选节点
    #[serde(default)]
    pub is_approver_type_free: bool,
    /// 节点是否支持抄送人自选
    #[serde(default)]
    pub has_cc_type_free: bool,
}

/// 预览审批流程响应（v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PreviewInstanceResponseV4 {
    /// 预览节点信息列表
    #[serde(default)]
    pub preview_nodes: Vec<PreviewInstanceNodeV4>,
}

/// 预览审批流程请求（v4）
#[derive(Debug, Clone)]
pub struct PreviewInstanceRequestV4 {
    config: Arc<Config>,
    body: PreviewInstanceBodyV4,
    user_id_type: Option<String>,
}

impl PreviewInstanceRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: PreviewInstanceBodyV4::default(),
            user_id_type: None,
        }
    }

    /// 设置审批定义 Code
    pub fn approval_code(mut self, approval_code: impl Into<String>) -> Self {
        self.body.approval_code = Some(approval_code.into());
        self
    }

    /// 设置用户 ID（发起人或任务审批人）
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置审批发起人所属的部门 ID
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.body.department_id = Some(department_id.into());
        self
    }

    /// 设置审批表单的控件 JSON 值（JSON 数组压缩转义后的字符串）
    pub fn form(mut self, form: impl Into<String>) -> Self {
        self.body.form = Some(form.into());
        self
    }

    /// 设置审批实例 Code
    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.body.instance_code = Some(instance_code.into());
        self
    }

    /// 设置审批任务 ID
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.body.task_id = Some(task_id.into());
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PreviewInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PreviewInstanceResponseV4> {
        validate_required!(self.body.user_id, "用户 ID 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstancePreview;
        let mut request = ApiRequest::<PreviewInstanceResponseV4>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("预览审批流程", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "预览审批流程",
        )
        .await
    }
}

impl ApiResponseTrait for PreviewInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_preview_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstancePreview;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances/preview"
        );
    }

    #[test]
    fn test_preview_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = PreviewInstanceRequestV4::new(config)
            .approval_code("C2CAAA90-70D9-3214-906B-B6FFF947F00D")
            .user_id("f7cb567e")
            .department_id("")
            .form("[{\"id\":\"widget16256287451710001\", \"type\": \"number\", \"value\":\"43\"}]")
            .user_id_type("open_id");

        assert_eq!(
            request.body.approval_code.as_deref(),
            Some("C2CAAA90-70D9-3214-906B-B6FFF947F00D")
        );
        assert_eq!(request.body.user_id, "f7cb567e");
        assert_eq!(
            request.body.form.as_deref(),
            Some("[{\"id\":\"widget16256287451710001\", \"type\": \"number\", \"value\":\"43\"}]")
        );
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }
}
