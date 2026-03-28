//! 创建审批定义（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/approval/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建审批定义请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateApprovalBodyV4 {
    /// 审批定义名称
    pub name: String,
    /// 审批定义描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 创建审批定义响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct CreateApprovalResponseV4 {
    /// 审批定义 Code
    pub approval_code: String,
}

/// 创建审批定义请求（v4）
#[derive(Debug, Clone)]
pub struct CreateApprovalRequestV4 {
    config: Arc<Config>,
    body: CreateApprovalBodyV4,
}

impl CreateApprovalRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateApprovalBodyV4::default(),
        }
    }

    /// 设置审批定义名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置审批定义描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
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
        validate_required!(self.body.name.trim(), "审批定义名称不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::ApprovalCreate;
        let mut request = ApiRequest::<CreateApprovalResponseV4>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for CreateApprovalResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_approval_create_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::ApprovalCreate;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/approvals");
    }
}
