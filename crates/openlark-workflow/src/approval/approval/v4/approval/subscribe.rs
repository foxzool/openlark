//! 订阅审批事件（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/event/event-interface/subscribe>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::Deserialize;
use std::sync::Arc;

/// 订阅审批事件响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct SubscribeApprovalResponseV4 {
    /// 是否成功
    pub success: bool,
}

/// 订阅审批事件请求（v4）
#[derive(Debug, Clone)]
pub struct SubscribeApprovalRequestV4 {
    config: Arc<Config>,
    approval_code: String,
}

impl SubscribeApprovalRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, approval_code: impl Into<String>) -> Self {
        Self {
            config,
            approval_code: approval_code.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SubscribeApprovalResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SubscribeApprovalResponseV4> {
        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::ApprovalSubscribe(
            self.approval_code.clone(),
        );
        let request = ApiRequest::<SubscribeApprovalResponseV4>::post(api_endpoint.to_url());

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for SubscribeApprovalResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_approval_subscribe_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::ApprovalSubscribe(
            "approval_123".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/approvals/approval_123/subscribe"
        );
    }
}
