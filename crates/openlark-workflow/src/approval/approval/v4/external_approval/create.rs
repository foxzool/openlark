//! 创建外部审批定义（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/external_approval/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建外部审批定义请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateExternalApprovalBodyV4 {
    /// 审批名称
    pub name: String,
    /// 审批描述
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// 表单字段列表
    pub form: Vec<FormField>,
}

/// 表单字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    /// 字段 ID
    pub id: String,
    /// 字段类型
    #[serde(rename = "type")]
    pub field_type: String,
    /// 字段名称
    pub name: String,
    /// 是否必填
    #[serde(default)]
    pub required: bool,
}

/// 创建外部审批定义响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct CreateExternalApprovalResponseV4 {
    /// 审批定义 Code
    pub approval_code: String,
}

/// 创建外部审批定义请求（v4）
#[derive(Debug, Clone)]
pub struct CreateExternalApprovalRequestV4 {
    config: Arc<Config>,
    body: CreateExternalApprovalBodyV4,
}

impl CreateExternalApprovalRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateExternalApprovalBodyV4::default(),
        }
    }

    /// 设置审批名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置审批描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = description.into();
        self
    }

    /// 添加表单字段
    pub fn add_form_field(
        mut self,
        id: impl Into<String>,
        field_type: impl Into<String>,
        name: impl Into<String>,
        required: bool,
    ) -> Self {
        self.body.form.push(FormField {
            id: id.into(),
            field_type: field_type.into(),
            name: name.into(),
            required,
        });
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
        validate_required!(self.body.name.trim(), "审批名称不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalApprovalCreate;
        let mut request = ApiRequest::<CreateExternalApprovalResponseV4>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("序列化请求体失败", e.to_string().as_str())
        })?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
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
    

    #[test]
    fn test_external_approval_create_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalApprovalCreate;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/external_approvals"
        );
    }
}
