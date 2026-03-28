//! 撤回审批实例（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/cancel>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 撤回审批实例请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelInstanceBodyV4 {
    /// 审批实例 Code
    pub instance_code: String,
}

/// 撤回审批实例响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct CancelInstanceResponseV4 {
    /// 是否成功
    pub success: bool,
}

/// 撤回审批实例请求（v4）
#[derive(Debug, Clone)]
pub struct CancelInstanceRequestV4 {
    config: Arc<Config>,
    body: CancelInstanceBodyV4,
}

impl CancelInstanceRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CancelInstanceBodyV4::default(),
        }
    }

    /// 设置审批实例 Code
    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.body.instance_code = instance_code.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CancelInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CancelInstanceResponseV4> {
        validate_required!(self.body.instance_code.trim(), "审批实例 Code 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCancel;
        let mut request = ApiRequest::<CancelInstanceResponseV4>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for CancelInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_instance_cancel_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCancel;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/instances/cancel");
    }
}
