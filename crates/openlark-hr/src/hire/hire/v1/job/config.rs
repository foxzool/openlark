//! 获取职位设置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/config>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取职位设置请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ConfigRequest {
    job_id: String,
    /// 配置信息
    config: Config,
}

impl ConfigRequest {
    /// 创建请求
    pub fn new(config: Config, job_id: String) -> Self {
        Self { job_id, config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ConfigResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ConfigResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.job_id.trim(), "职位 ID 不能为空");

        let api_endpoint = HireApiV1::JobConfig(self.job_id);
        let request = ApiRequest::<ConfigResponse>::get(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取职位设置响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取职位设置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for ConfigResponse {
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
    fn test_config_request_builder_new() {
        let request = ConfigRequest::new(TestConfigBuilder::new().build(), "test".to_string());
        let _ = request;
    }
}
