//! 获取外部审批定义详情（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/external_approval/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取外部审批定义详情响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct GetExternalApprovalResponseV4 {
    /// 审批定义 Code
    pub approval_code: String,
    /// 审批名称
    pub name: String,
    /// 审批描述
    #[serde(default)]
    pub description: String,
    /// 表单字段列表
    pub form: Vec<FormField>,
    /// 创建时间（Unix 时间戳）
    pub create_time: i64,
    /// 更新时间（Unix 时间戳）
    #[serde(default)]
    pub update_time: i64,
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

/// 获取外部审批定义详情请求（v4）
#[derive(Debug, Clone)]
pub struct GetExternalApprovalRequestV4 {
    config: Arc<Config>,
    approval_code: String,
}

impl GetExternalApprovalRequestV4 {
    pub fn new(config: Arc<Config>, approval_code: impl Into<String>) -> Self {
        Self {
            config,
            approval_code: approval_code.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetExternalApprovalResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetExternalApprovalResponseV4> {
        validate_required!(
            self.approval_code.trim(),
            "审批定义 Code 不能为空"
        );

        let api_endpoint =
            crate::common::api_endpoints::ApprovalApiV4::ExternalApprovalGet(self.approval_code);
        let request = ApiRequest::<GetExternalApprovalResponseV4>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for GetExternalApprovalResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_external_approval_get_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalApprovalGet(
            "test_approval_code".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/external_approvals/test_approval_code"
        );
    }
}
