//! 重启职位
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/open>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 重启职位请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OpenRequest {
    job_id: String,
    /// 配置信息
    config: Config,
}

impl OpenRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            job_id: String::new(),
            config,
        }
    }

    pub fn job_id(mut self, job_id: String) -> Self {
        self.job_id = job_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OpenResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OpenResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.job_id.trim(), "职位 ID 不能为空");

        let api_endpoint = HireApiV1::JobOpen(self.job_id);
        let request = ApiRequest::<OpenResponse>::post(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "重启职位响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 重启职位响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for OpenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_open_request_builder_new() {
        let request = OpenRequest::new(TestConfigBuilder::new().build()).job_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_open_request_validation_fails_on_default_request() {
        let request = OpenRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
