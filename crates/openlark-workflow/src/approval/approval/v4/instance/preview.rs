//! 预览审批实例（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/preview>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 预览审批实例请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct PreviewInstanceBodyV4 {
    /// 审批定义 Code
    pub approval_code: String,
    /// 表单数据
    pub form: Vec<FormValue>,
}

/// 表单值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormValue {
    /// 字段 ID
    pub id: String,
    /// 字段值
    pub value: serde_json::Value,
}

/// 预览审批实例响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct PreviewInstanceResponseV4 {
    /// 审批流程预览
    pub flow: Vec<FlowNode>,
}

/// 流程节点
#[derive(Debug, Clone, Deserialize)]
pub struct FlowNode {
    /// 节点 ID
    pub node_id: String,
    /// 节点名称
    pub node_name: String,
    /// 节点类型
    #[serde(rename = "type")]
    pub node_type: String,
    /// 处理人用户 ID
    #[serde(default)]
    pub user_id: String,
}

/// 预览审批实例请求（v4）
#[derive(Debug, Clone)]
pub struct PreviewInstanceRequestV4 {
    config: Arc<Config>,
    body: PreviewInstanceBodyV4,
}

impl PreviewInstanceRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: PreviewInstanceBodyV4::default(),
        }
    }

    /// 设置审批定义 Code
    pub fn approval_code(mut self, approval_code: impl Into<String>) -> Self {
        self.body.approval_code = approval_code.into();
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
        validate_required!(self.body.approval_code.trim(), "审批定义 Code 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstancePreview;
        let mut request = ApiRequest::<PreviewInstanceResponseV4>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for PreviewInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_instance_preview_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstancePreview;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances/preview"
        );
    }
}
