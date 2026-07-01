//! 查询单个人员自定义组织变更记录
//!
//! docPath: <https://open.feishu.cn/document/corehr-v1/employee/employee-custom_org/querybyid>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

/// 查询单个人员自定义组织变更记录请求。
#[derive(Debug, Clone)]
pub struct EmployeeCustomOrgQuerybyidRequest {
    config: Arc<Config>,
}

impl EmployeeCustomOrgQuerybyidRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let path = "/open-apis/corehr/v2/custom_org/querybyid".to_string();
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询单个人员自定义组织变更记录", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = EmployeeCustomOrgQuerybyidRequest::new(config);
    }
}
