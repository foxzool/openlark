//! 批量获取审批实例 ID（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 审批实例列表项（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct InstanceItemV4 {
    /// 审批实例 Code
    pub instance_code: String,
}

/// 批量获取审批实例 ID 响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct ListInstanceResponseV4 {
    /// 审批实例列表
    pub items: Vec<InstanceItemV4>,
}

/// 批量获取审批实例 ID 请求（v4）
#[derive(Debug, Clone)]
pub struct ListInstanceRequestV4 {
    config: Arc<Config>,
    approval_code: String,
}

impl ListInstanceRequestV4 {
    pub fn new(config: Arc<Config>, approval_code: impl Into<String>) -> Self {
        Self {
            config,
            approval_code: approval_code.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListInstanceResponseV4> {
        let api_endpoint =
            crate::common::api_endpoints::ApprovalApiV4::InstanceList(self.approval_code.clone());
        let request = ApiRequest::<ListInstanceResponseV4>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for ListInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_instance_list_v4_url() {
        let endpoint =
            crate::common::api_endpoints::ApprovalApiV4::InstanceList("approval_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances?approval_code=approval_123"
        );
    }
}
