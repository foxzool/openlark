//! 查询抄送列表（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/approval-search/search_cc>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 审批抄送列表项（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct CcItemV4 {
    /// 抄送 ID
    pub cc_id: String,
    /// 审批实例 Code
    pub instance_code: String,
    /// 抄送状态
    pub status: String,
}

/// 查询抄送列表响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct SearchCcResponseV4 {
    /// 抄送列表
    pub items: Vec<CcItemV4>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

/// 查询抄送列表请求（v4）
#[derive(Debug, Clone)]
pub struct SearchCcRequestV4 {
    config: Arc<Config>,
}

impl SearchCcRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchCcResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchCcResponseV4> {
        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceSearchCc;
        let request = ApiRequest::<SearchCcResponseV4>::post(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for SearchCcResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_instance_search_cc_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceSearchCc;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/instances/search_cc");
    }
}
