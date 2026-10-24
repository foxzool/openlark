//! 创建三方审批定义（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/external_approval/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::approval::approval::v4::models::{ApprovalCreateViewerV4, I18nResourceV4};
use crate::common::api_utils::request_serialization_error;

/// 三方审批相关信息（approval_create_external）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApprovalCreateExternalV4 {
    /// 列表中用于提示审批来自哪个三方系统的国际化文案 Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_name: Option<String>,
    /// 审批定义业务类别，自定义设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
    /// PC 端发起三方审批的链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_link_pc: Option<String>,
    /// 移动端发起三方审批的链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_link_mobile: Option<String>,
    /// 是否在 PC 端的发起审批页面展示（与 support_mobile 不可同为 false）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_pc: Option<bool>,
    /// 是否在移动端的发起审批页面展示（与 support_pc 不可同为 false）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_mobile: Option<bool>,
    /// 是否支持批量已读
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_batch_read: Option<bool>,
    /// 是否支持标注可读（该字段暂无效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_mark_readed: Option<bool>,
    /// 是否支持快速操作（该字段暂无效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_quick_operate: Option<bool>,
    /// 三方系统的操作回调 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_callback_url: Option<String>,
    /// 回调时带的 token，用于业务系统验证请求来自审批中心
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_callback_token: Option<String>,
    /// 请求参数加密密钥
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_callback_key: Option<String>,
    /// 是否支持批量审批
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_batch_operate: Option<bool>,
    /// 审批流程数据是否不纳入效率统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_efficiency_statistics: Option<bool>,
}

/// 创建三方审批定义请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateExternalApprovalBodyV4 {
    /// 三方审批定义名称（国际化文案 Key，以 `@i18n@` 开头）
    pub approval_name: String,
    /// 应用自定义 Code，最大 128 字符，用于唯一关联三方审批定义
    pub approval_code: String,
    /// 审批定义所属审批分组，用户自定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_code: Option<String>,
    /// 审批分组名称（国际化文案 Key，以 `@i18n@` 开头）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// 审批定义的说明（国际化文案 Key，以 `@i18n@` 开头）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 三方审批相关信息
    pub external: ApprovalCreateExternalV4,
    /// 审批可见人列表（最大 200）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewers: Option<Vec<ApprovalCreateViewerV4>>,
    /// 国际化文案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_resources: Option<Vec<I18nResourceV4>>,
    /// 审批流程管理员的用户 ID 列表（最多 200 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managers: Option<Vec<String>>,
}

/// 创建三方审批定义响应（v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateExternalApprovalResponseV4 {
    /// 审批定义 Code
    pub approval_code: String,
}

/// 创建三方审批定义请求（v4）
#[derive(Debug, Clone)]
pub struct CreateExternalApprovalRequestV4 {
    config: Arc<Config>,
    body: CreateExternalApprovalBodyV4,
}

impl CreateExternalApprovalRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateExternalApprovalBodyV4::default(),
        }
    }

    /// 设置三方审批定义名称（国际化文案 Key）
    pub fn approval_name(mut self, approval_name: impl Into<String>) -> Self {
        self.body.approval_name = approval_name.into();
        self
    }

    /// 设置应用自定义 Code
    pub fn approval_code(mut self, approval_code: impl Into<String>) -> Self {
        self.body.approval_code = approval_code.into();
        self
    }

    /// 设置审批定义所属审批分组
    pub fn group_code(mut self, group_code: impl Into<String>) -> Self {
        self.body.group_code = Some(group_code.into());
        self
    }

    /// 设置审批分组名称（国际化文案 Key）
    pub fn group_name(mut self, group_name: impl Into<String>) -> Self {
        self.body.group_name = Some(group_name.into());
        self
    }

    /// 设置审批定义的说明（国际化文案 Key）
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 设置三方审批相关信息
    pub fn external(mut self, external: ApprovalCreateExternalV4) -> Self {
        self.body.external = external;
        self
    }

    /// 设置审批可见人列表
    pub fn viewers(mut self, viewers: Vec<ApprovalCreateViewerV4>) -> Self {
        self.body.viewers = Some(viewers);
        self
    }

    /// 设置国际化文案
    pub fn i18n_resources(mut self, i18n_resources: Vec<I18nResourceV4>) -> Self {
        self.body.i18n_resources = Some(i18n_resources);
        self
    }

    /// 设置审批流程管理员的用户 ID 列表
    pub fn managers(mut self, managers: Vec<String>) -> Self {
        self.body.managers = Some(managers);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateExternalApprovalResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateExternalApprovalResponseV4> {
        validate_required!(self.body.approval_name, "三方审批定义名称不能为空");
        validate_required!(self.body.approval_code, "应用自定义 Code 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalApprovalCreate;
        let mut request =
            ApiRequest::<CreateExternalApprovalResponseV4>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("创建三方审批定义", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建三方审批定义",
        )
        .await
    }
}

impl ApiResponseTrait for CreateExternalApprovalResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_external_approval_create_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalApprovalCreate;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/external_approvals"
        );
    }

    #[test]
    fn test_create_external_approval_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = CreateExternalApprovalRequestV4::new(config)
            .approval_name("@i18n@demoname")
            .approval_code("F46EB460-9476-4789-9524-ECD564291234")
            .group_code("work_group")
            .external(ApprovalCreateExternalV4 {
                support_pc: Some(true),
                support_mobile: Some(true),
                ..Default::default()
            })
            .managers(vec!["96449fb3".to_string()]);

        assert_eq!(request.body.approval_name, "@i18n@demoname");
        assert_eq!(
            request.body.approval_code,
            "F46EB460-9476-4789-9524-ECD564291234"
        );
        assert_eq!(request.body.group_code.as_deref(), Some("work_group"));
        assert_eq!(request.body.external.support_pc, Some(true));
        assert_eq!(request.body.managers, Some(vec!["96449fb3".to_string()]));
    }
}
