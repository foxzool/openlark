//! 创建外部审批实例（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/external_instance/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建外部审批实例请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateExternalInstanceBodyV4 {
    /// 审批定义 Code
    pub approval_code: String,
    /// 审批实例标题
    pub title: String,
    /// 发起人用户 ID
    pub user_id: String,
    /// 表单数据
    pub form: Vec<FormValue>,
    /// 是否需要审批
    #[serde(default)]
    pub need_approval: bool,
}

/// 表单值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormValue {
    /// 字段 ID
    pub id: String,
    /// 字段值
    pub value: serde_json::Value,
}

/// 创建外部审批实例响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct CreateExternalInstanceResponseV4 {
    /// 审批实例 ID
    pub instance_id: String,
    /// 审批实例 Code
    pub instance_code: String,
}

/// 创建外部审批实例请求（v4）
#[derive(Debug, Clone)]
pub struct CreateExternalInstanceRequestV4 {
    config: Arc<Config>,
    body: CreateExternalInstanceBodyV4,
}

impl CreateExternalInstanceRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateExternalInstanceBodyV4::default(),
        }
    }

    /// 设置审批定义 Code
    pub fn approval_code(mut self, approval_code: impl Into<String>) -> Self {
        self.body.approval_code = approval_code.into();
        self
    }

    /// 设置审批实例标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.body.title = title.into();
        self
    }

    /// 设置发起人用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 添加表单字段值
    pub fn add_form_value(mut self, id: impl Into<String>, value: serde_json::Value) -> Self {
        self.body.form.push(FormValue {
            id: id.into(),
            value,
        });
        self
    }

    /// 设置是否需要审批
    pub fn need_approval(mut self, need_approval: bool) -> Self {
        self.body.need_approval = need_approval;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateExternalInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateExternalInstanceResponseV4> {
        validate_required!(self.body.approval_code.trim(), "审批定义 Code 不能为空");
        validate_required!(self.body.title.trim(), "审批实例标题不能为空");
        validate_required!(self.body.user_id.trim(), "发起人用户 ID 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalInstanceCreate;
        let mut request = ApiRequest::<CreateExternalInstanceResponseV4>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for CreateExternalInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_external_instance_create_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalInstanceCreate;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/external_instances"
        );
    }
}
