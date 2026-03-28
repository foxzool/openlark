//! 查询实例列表（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/approval-search/query-2>

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
    /// 审批定义 Code
    pub approval_code: String,
    /// 审批实例状态
    pub status: String,
}

/// 查询实例列表响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct QueryInstanceResponseV4 {
    /// 审批实例列表
    pub items: Vec<InstanceItemV4>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

/// 查询实例列表请求（v4）
#[derive(Debug, Clone)]
pub struct QueryInstanceRequestV4 {
    config: Arc<Config>,
}

impl QueryInstanceRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryInstanceResponseV4> {
        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceQuery;
        let request = ApiRequest::<QueryInstanceResponseV4>::post(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for QueryInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_instance_query_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceQuery;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/instances/query");
    }
}
