//! 获取单个审批实例详情（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 审批实例详情（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstanceResponseV4 {
    /// 审批实例 Code
    pub instance_code: String,
    /// 审批定义 Code
    pub approval_code: String,
    /// 审批实例状态
    pub status: String,
}

/// 获取单个审批实例详情请求（v4）
#[derive(Debug, Clone)]
pub struct GetInstanceRequestV4 {
    config: Arc<Config>,
    instance_id: String,
}

impl GetInstanceRequestV4 {
    pub fn new(config: Arc<Config>, instance_id: impl Into<String>) -> Self {
        Self {
            config,
            instance_id: instance_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetInstanceResponseV4> {
        let api_endpoint =
            crate::common::api_endpoints::ApprovalApiV4::InstanceGet(self.instance_id.clone());
        let request = ApiRequest::<GetInstanceResponseV4>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for GetInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_instance_get_v4_url() {
        let endpoint =
            crate::common::api_endpoints::ApprovalApiV4::InstanceGet("instance_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances/instance_123"
        );
    }
}
