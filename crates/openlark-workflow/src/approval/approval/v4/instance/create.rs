//! 创建审批实例（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::approval::approval::v4::models::I18nResourceV4;
use crate::common::api_utils::request_serialization_error;

/// 发起人自选审批人节点（node_approver）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeApproverV4 {
    /// 节点的 node_id 或 custom_node_id
    pub key: String,
    /// 审批人列表（user_id 或 open_id）
    pub value: Vec<String>,
}

/// 发起人自选抄送人节点（node_cc）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeCcV4 {
    /// 节点的 node_id
    pub key: String,
    /// 抄送人列表（user_id 或 open_id）
    pub value: Vec<String>,
}

/// 自动通过节点（node_auto_approval）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeAutoApprovalV4 {
    /// 节点 ID 类型（CUSTOM/NON_CUSTOM）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id_type: Option<String>,
    /// 节点 ID 值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

/// 创建审批实例请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateInstanceBodyV4 {
    /// 审批定义 Code
    pub approval_code: String,
    /// 审批发起人的 user_id（与 open_id 必须传入其中一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 审批发起人的 open_id（与 user_id 必须传入其中一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 审批发起人所属部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 填写的审批表单控件值（JSON 数组压缩转义后的字符串）
    pub form: String,
    /// 发起人自选审批人（通过 user_id 指定）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_approver_user_id_list: Option<Vec<NodeApproverV4>>,
    /// 发起人自选审批人（通过 open_id 指定）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_approver_open_id_list: Option<Vec<NodeApproverV4>>,
    /// 发起人自选抄送人（通过 user_id 指定）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_cc_user_id_list: Option<Vec<NodeCcV4>>,
    /// 发起人自选抄送人（通过 open_id 指定）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_cc_open_id_list: Option<Vec<NodeCcV4>>,
    /// 审批实例 uuid，用于幂等操作
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// 是否配置「提交」按钮
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_resubmit: Option<bool>,
    /// 是否配置「再次提交」按钮
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_submit_again: Option<bool>,
    /// 取消指定的 Bot 推送通知（位运算加和值）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_bot_notification: Option<String>,
    /// 是否禁止撤销审批实例
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbid_revoke: Option<bool>,
    /// 国际化文案（目前只支持表单的单行、多行文本控件）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_resources: Option<Vec<I18nResourceV4>>,
    /// 审批实例的展示名称（国际化文案 Key）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 审批详情页 title 展示模式（0 全部展示 / 1 只展示实例 title）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_display_method: Option<i32>,
    /// 自动通过的节点列表（最大 10）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_auto_approval_list: Option<Vec<NodeAutoApprovalV4>>,
}

/// 创建审批实例响应（v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateInstanceResponseV4 {
    /// 审批实例 Code
    pub instance_code: String,
}

/// 创建审批实例请求（v4）
#[derive(Debug, Clone)]
pub struct CreateInstanceRequestV4 {
    config: Arc<Config>,
    body: CreateInstanceBodyV4,
}

impl CreateInstanceRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateInstanceBodyV4::default(),
        }
    }

    /// 设置审批定义 Code
    pub fn approval_code(mut self, approval_code: impl Into<String>) -> Self {
        self.body.approval_code = approval_code.into();
        self
    }

    /// 设置审批发起人的 user_id
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = Some(user_id.into());
        self
    }

    /// 设置审批发起人的 open_id
    pub fn open_id(mut self, open_id: impl Into<String>) -> Self {
        self.body.open_id = Some(open_id.into());
        self
    }

    /// 设置审批发起人所属部门 ID
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.body.department_id = Some(department_id.into());
        self
    }

    /// 设置审批表单控件值（JSON 数组压缩转义后的字符串）
    pub fn form(mut self, form: impl Into<String>) -> Self {
        self.body.form = form.into();
        self
    }

    /// 设置发起人自选审批人列表（通过 user_id 指定）
    pub fn node_approver_user_id_list(mut self, list: Vec<NodeApproverV4>) -> Self {
        self.body.node_approver_user_id_list = Some(list);
        self
    }

    /// 设置发起人自选审批人列表（通过 open_id 指定）
    pub fn node_approver_open_id_list(mut self, list: Vec<NodeApproverV4>) -> Self {
        self.body.node_approver_open_id_list = Some(list);
        self
    }

    /// 设置发起人自选抄送人列表（通过 user_id 指定）
    pub fn node_cc_user_id_list(mut self, list: Vec<NodeCcV4>) -> Self {
        self.body.node_cc_user_id_list = Some(list);
        self
    }

    /// 设置发起人自选抄送人列表（通过 open_id 指定）
    pub fn node_cc_open_id_list(mut self, list: Vec<NodeCcV4>) -> Self {
        self.body.node_cc_open_id_list = Some(list);
        self
    }

    /// 设置审批实例 uuid（幂等操作）
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.body.uuid = Some(uuid.into());
        self
    }

    /// 设置是否配置「提交」按钮
    pub fn allow_resubmit(mut self, allow_resubmit: bool) -> Self {
        self.body.allow_resubmit = Some(allow_resubmit);
        self
    }

    /// 设置是否配置「再次提交」按钮
    pub fn allow_submit_again(mut self, allow_submit_again: bool) -> Self {
        self.body.allow_submit_again = Some(allow_submit_again);
        self
    }

    /// 设置取消指定的 Bot 推送通知（位运算加和值）
    pub fn cancel_bot_notification(mut self, cancel: impl Into<String>) -> Self {
        self.body.cancel_bot_notification = Some(cancel.into());
        self
    }

    /// 设置是否禁止撤销审批实例
    pub fn forbid_revoke(mut self, forbid_revoke: bool) -> Self {
        self.body.forbid_revoke = Some(forbid_revoke);
        self
    }

    /// 设置国际化文案
    pub fn i18n_resources(mut self, i18n_resources: Vec<I18nResourceV4>) -> Self {
        self.body.i18n_resources = Some(i18n_resources);
        self
    }

    /// 设置审批实例的展示名称（国际化文案 Key）
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.body.title = Some(title.into());
        self
    }

    /// 设置审批详情页 title 展示模式
    pub fn title_display_method(mut self, title_display_method: i32) -> Self {
        self.body.title_display_method = Some(title_display_method);
        self
    }

    /// 设置自动通过的节点列表
    pub fn node_auto_approval_list(mut self, list: Vec<NodeAutoApprovalV4>) -> Self {
        self.body.node_auto_approval_list = Some(list);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateInstanceResponseV4> {
        validate_required!(self.body.approval_code, "审批定义 Code 不能为空");
        validate_required!(self.body.form, "审批表单数据不能为空");
        if self.body.user_id.as_deref().is_none_or(str::is_empty)
            && self.body.open_id.as_deref().is_none_or(str::is_empty)
        {
            return Err(openlark_core::error::CoreError::validation_msg(
                "审批发起人的 user_id 与 open_id 必须传入其中一个",
            ));
        }

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCreate;
        let mut request = ApiRequest::<CreateInstanceResponseV4>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("创建审批实例", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建审批实例",
        )
        .await
    }
}

impl ApiResponseTrait for CreateInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_create_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCreate;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/instances");
    }

    #[test]
    fn test_create_instance_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = CreateInstanceRequestV4::new(config)
            .approval_code("4202AD96-9EC1-4284-9C48-B923CDC4F30B")
            .user_id("59a92c4a")
            .form("[{\"id\":\"111\",\"type\":\"input\",\"value\":\"11111\"}]")
            .uuid("7C468A54-8745-2245-9675-08B7C63E7A87")
            .allow_resubmit(true)
            .node_approver_user_id_list(vec![NodeApproverV4 {
                key: "46e6d96cfa756980907209209ec03b64".to_string(),
                value: vec!["59a92c4a".to_string()],
            }]);

        assert_eq!(
            request.body.approval_code,
            "4202AD96-9EC1-4284-9C48-B923CDC4F30B"
        );
        assert_eq!(request.body.user_id.as_deref(), Some("59a92c4a"));
        assert_eq!(
            request.body.form,
            "[{\"id\":\"111\",\"type\":\"input\",\"value\":\"11111\"}]"
        );
        assert_eq!(request.body.allow_resubmit, Some(true));
        assert_eq!(
            request
                .body
                .node_approver_user_id_list
                .as_ref()
                .unwrap()
                .len(),
            1
        );
    }
}
