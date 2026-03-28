//! 获取职位列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取职位列表请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ListRequest {
    page_size: Option<i32>,
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            config,
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::HireApiV1;

        let api_endpoint = HireApiV1::JobList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取职位列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取职位列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for ListResponse {
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
    fn test_list_request_builder_new() {
        let request = ListRequest::new(TestConfigBuilder::new().build()).page_size(1);
        let _ = request;
    }
}
