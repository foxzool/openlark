//! 撤回审批实例（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/cancel>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::request_serialization_error;

/// 撤回审批实例请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelInstanceBodyV4 {
    /// 审批定义 Code
    pub approval_code: String,
    /// 审批实例 Code
    pub instance_code: String,
    /// 审批提交人的用户 ID
    pub user_id: String,
}

/// 撤回审批实例响应（v4）
///
/// 官方响应仅包含 `code` / `msg`，`data` 为空对象。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CancelInstanceResponseV4 {}

/// 撤回审批实例请求（v4）
#[derive(Debug, Clone)]
pub struct CancelInstanceRequestV4 {
    config: Arc<Config>,
    body: CancelInstanceBodyV4,
    user_id_type: Option<String>,
}

impl CancelInstanceRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CancelInstanceBodyV4::default(),
            user_id_type: None,
        }
    }

    /// 设置审批定义 Code
    pub fn approval_code(mut self, approval_code: impl Into<String>) -> Self {
        self.body.approval_code = approval_code.into();
        self
    }

    /// 设置审批实例 Code
    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.body.instance_code = instance_code.into();
        self
    }

    /// 设置审批提交人的用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
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
        validate_required!(self.body.approval_code, "审批定义 Code 不能为空");
        validate_required!(self.body.instance_code, "审批实例 Code 不能为空");
        validate_required!(self.body.user_id, "审批提交人的用户 ID 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCancel;
        let mut request = ApiRequest::<CancelInstanceResponseV4>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("撤回审批实例", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "撤回审批实例",
        )
        .await
    }
}

impl ApiResponseTrait for CancelInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 官方成功响应 `data` 为空对象（或缺省），显式给出空成功值
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_cancel_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCancel;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/instances/cancel");
    }

    #[test]
    fn test_cancel_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = CancelInstanceRequestV4::new(config)
            .approval_code("7C468A54-8745-2245-9675-08B7C63E7A85")
            .instance_code("81D31358-93AF-92D6-7425-01A5D67C4E71")
            .user_id("f7cb567e");

        assert_eq!(
            request.body.approval_code,
            "7C468A54-8745-2245-9675-08B7C63E7A85"
        );
        assert_eq!(
            request.body.instance_code,
            "81D31358-93AF-92D6-7425-01A5D67C4E71"
        );
        assert_eq!(request.body.user_id, "f7cb567e");
    }
}
