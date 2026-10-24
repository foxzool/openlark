//! 创建审批定义（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/approval/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::request_serialization_error;

use crate::approval::approval::v4::models::{ApprovalCreateViewerV4, I18nResourceV4};

/// 审批定义表单（approval_form）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApprovalFormV4 {
    /// 表单内容：JSON 数组压缩转义后的字符串
    pub form_content: String,
}

/// 审批人 / 抄送人（approval_approver_ccer）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApprovalApproverCcerV4 {
    /// 审批人/抄送人类型（Supervisor/SupervisorTopDown/DepartmentManager/DepartmentManagerTopDown/Personal/Free）
    #[serde(rename = "type")]
    pub approver_type: String,
    /// type 为 Personal 时的用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 审批级数（主管/部门负责人类审批人需要填写）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

/// 表单控件权限（field_group）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApprovalFieldGroupV4 {
    /// 可写权限的表单控件项的 ID 列表
    pub writable: Vec<String>,
    /// 可读权限的表单控件项的 ID 列表
    pub readable: Vec<String>,
}

/// 发起人自选审批人范围（approver_range）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApprovalApproverRangeV4 {
    /// 审批人类型（ALL/PERSONAL/ROLE）
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_type: Option<String>,
    /// ID 列表（用户 ID 或角色 ID）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_list: Option<Vec<String>>,
}

/// 审批定义节点（approval_node）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApprovalNodeV4 {
    /// 节点 ID（开始节点为 START，结束节点为 END）
    pub id: String,
    /// 节点名称的国际化文案 Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 节点审批方式（AND/OR/SEQUENTIAL）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 审批人列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approver: Option<Vec<ApprovalApproverCcerV4>>,
    /// 抄送人列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccer: Option<Vec<ApprovalApproverCcerV4>>,
    /// 表单内的控件权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privilege_field: Option<ApprovalFieldGroupV4>,
    /// 发起人自选审批人时是否允许多选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approver_chosen_multi: Option<bool>,
    /// 发起人自选审批人时可选择的范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approver_chosen_range: Option<Vec<ApprovalApproverRangeV4>>,
    /// 审批人为提交人本人时的操作（STARTER/AUTO_PASS/SUPERVISOR/DEPARTMENT_MANAGER）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starter_assignee: Option<String>,
}

/// 审批定义其他设置（approval_setting）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApprovalSettingV4 {
    /// 审批实例通过后允许撤回的时间（秒），0 为不可撤回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert_interval: Option<i32>,
    /// 是否支持审批通过第一个节点后撤回（1 支持 / 0 不支持）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revert_option: Option<i32>,
    /// 审批被拒绝后的设置（0 流程终止 / 1 退回至发起人）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_option: Option<i32>,
    /// 快捷审批配置项（0 禁用 / 1 启用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_approval_option: Option<i32>,
}

/// 审批定义配置项（approval_config）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApprovalConfigV4 {
    /// 是否允许用户修改可见范围
    pub can_update_viewer: bool,
    /// 是否允许用户更新表单
    pub can_update_form: bool,
    /// 是否允许用户更新流程定义
    pub can_update_process: bool,
    /// 是否允许用户更新撤回设置
    pub can_update_revert: bool,
    /// 审批定义的帮助文档链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help_url: Option<String>,
}

/// 创建审批定义请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateApprovalBodyV4 {
    /// 审批名称的国际化文案 Key（以 `@i18n@` 开头，不少于 9 个字符）
    pub approval_name: String,
    /// 审批定义 Code：不传为新建；传入则为覆盖式全量更新
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    /// 审批描述的国际化文案 Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 审批可见范围列表（最大 200）
    pub viewers: Vec<ApprovalCreateViewerV4>,
    /// 审批定义表单
    pub form: ApprovalFormV4,
    /// 审批定义节点列表（首元素为开始节点，末元素为结束节点）
    pub node_list: Vec<ApprovalNodeV4>,
    /// 审批定义其他设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<ApprovalSettingV4>,
    /// 审批定义配置项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ApprovalConfigV4>,
    /// 审批图标枚举（0~24，默认 0）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<i32>,
    /// 国际化文案
    pub i18n_resources: Vec<I18nResourceV4>,
    /// 审批流程管理员的用户 ID 列表（最大 200）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_manager_ids: Option<Vec<String>>,
}

/// 创建审批定义响应（v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateApprovalResponseV4 {
    /// 审批定义 Code
    pub approval_code: String,
    /// 审批定义 ID
    #[serde(default)]
    pub approval_id: String,
}

/// 创建审批定义请求（v4）
#[derive(Debug, Clone)]
pub struct CreateApprovalRequestV4 {
    config: Arc<Config>,
    body: CreateApprovalBodyV4,
}

impl CreateApprovalRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateApprovalBodyV4::default(),
        }
    }

    /// 设置审批名称的国际化文案 Key（以 `@i18n@` 开头）
    pub fn approval_name(mut self, approval_name: impl Into<String>) -> Self {
        self.body.approval_name = approval_name.into();
        self
    }

    /// 设置审批定义 Code（传入时为覆盖式全量更新）
    pub fn approval_code(mut self, approval_code: impl Into<String>) -> Self {
        self.body.approval_code = Some(approval_code.into());
        self
    }

    /// 设置审批描述的国际化文案 Key
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 设置审批可见范围列表
    pub fn viewers(mut self, viewers: Vec<ApprovalCreateViewerV4>) -> Self {
        self.body.viewers = viewers;
        self
    }

    /// 设置审批定义表单
    pub fn form(mut self, form: ApprovalFormV4) -> Self {
        self.body.form = form;
        self
    }

    /// 设置审批定义节点列表
    pub fn node_list(mut self, node_list: Vec<ApprovalNodeV4>) -> Self {
        self.body.node_list = node_list;
        self
    }

    /// 设置审批定义其他设置
    pub fn settings(mut self, settings: ApprovalSettingV4) -> Self {
        self.body.settings = Some(settings);
        self
    }

    /// 设置审批定义配置项
    pub fn config(mut self, config: ApprovalConfigV4) -> Self {
        self.body.config = Some(config);
        self
    }

    /// 设置审批图标枚举（0~24）
    pub fn icon(mut self, icon: i32) -> Self {
        self.body.icon = Some(icon);
        self
    }

    /// 设置国际化文案
    pub fn i18n_resources(mut self, i18n_resources: Vec<I18nResourceV4>) -> Self {
        self.body.i18n_resources = i18n_resources;
        self
    }

    /// 设置审批流程管理员的用户 ID 列表
    pub fn process_manager_ids(mut self, process_manager_ids: Vec<String>) -> Self {
        self.body.process_manager_ids = Some(process_manager_ids);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateApprovalResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateApprovalResponseV4> {
        validate_required!(self.body.approval_name, "审批名称不能为空");
        validate_required!(self.body.viewers, "审批可见范围列表不能为空");
        validate_required!(self.body.node_list, "审批定义节点列表不能为空");
        validate_required!(self.body.i18n_resources, "国际化文案不能为空");
        validate_required!(self.body.form.form_content, "审批定义表单内容不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::ApprovalCreate;
        let mut request = ApiRequest::<CreateApprovalResponseV4>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("创建审批定义", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建审批定义",
        )
        .await
    }
}

impl ApiResponseTrait for CreateApprovalResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::approval::approval::v4::models::I18nResourceTextV4;

    #[test]
    fn test_approval_create_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::ApprovalCreate;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/approvals");
    }

    #[test]
    fn test_create_approval_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = CreateApprovalRequestV4::new(config)
            .approval_name("@i18n@approval_name")
            .description("@i18n@description")
            .viewers(vec![ApprovalCreateViewerV4 {
                viewer_type: Some("USER".to_string()),
                viewer_user_id: Some("19a294c2".to_string()),
                viewer_department_id: None,
            }])
            .form(ApprovalFormV4 {
                form_content: "[{\"id\":\"user_name\", \"type\": \"input\", \"required\":true, \"name\":\"@i18n@widget1\"}]".to_string(),
            })
            .node_list(vec![ApprovalNodeV4 {
                id: "START".to_string(),
                ..Default::default()
            }])
            .icon(0)
            .i18n_resources(vec![I18nResourceV4 {
                locale: "zh-CN".to_string(),
                texts: vec![I18nResourceTextV4 {
                    key: "@i18n@1".to_string(),
                    value: "people".to_string(),
                }],
                is_default: true,
            }])
            .process_manager_ids(vec!["1c5ea995".to_string()]);

        assert_eq!(request.body.approval_name, "@i18n@approval_name");
        assert_eq!(request.body.viewers.len(), 1);
        assert_eq!(request.body.node_list.len(), 1);
        assert_eq!(request.body.i18n_resources.len(), 1);
        assert_eq!(request.body.icon, Some(0));
        assert_eq!(
            request.body.process_manager_ids,
            Some(vec!["1c5ea995".to_string()])
        );
    }
}
