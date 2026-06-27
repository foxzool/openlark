//! 获取职位上的招聘人员信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/job/recruiter

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::JobRecruiterRecord;

/// 获取职位上的招聘人员信息请求
#[derive(Debug, Clone)]
pub struct RecruiterRequest {
    job_id: String,
    /// 配置信息
    config: Config,
}

impl RecruiterRequest {
    /// 创建请求
    pub fn new(config: Config, job_id: String) -> Self {
        Self { job_id, config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecruiterResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RecruiterResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.job_id.trim(), "职位 ID 不能为空");

        let api_endpoint = HireApiV1::JobRecruiter(self.job_id);
        let request = ApiRequest::<RecruiterResponse>::get(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取职位上的招聘人员信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取职位上的招聘人员信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RecruiterResponse {
    #[serde(default, alias = "items", alias = "recruiters")]
    /// `recruiters` 字段。
    pub recruiters: Vec<JobRecruiterRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for RecruiterResponse {
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
    fn test_recruiter_request_builder_new() {
        let request = RecruiterRequest::new(TestConfigBuilder::new().build(), "test".to_string());
        let _ = request;
    }
}
