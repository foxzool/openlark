//! 新建职位
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/combined_create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 新建职位请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CombinedCreateRequest {
    request_body: CombinedCreateRequestBody,
    /// 配置信息
    config: Config,
}

impl CombinedCreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            request_body: CombinedCreateRequestBody::default(),
            config,
        }
    }

    pub fn request_body(mut self, request_body: CombinedCreateRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CombinedCreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CombinedCreateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        self.request_body.validate()?;

        let api_endpoint = HireApiV1::JobCombinedCreate;
        let request = ApiRequest::<CombinedCreateResponse>::post(api_endpoint.to_url());
        let request = request.body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "新建职位响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CombinedCreateRequestBody {
    #[serde(flatten)]
    pub fields: Value,
}

impl CombinedCreateRequestBody {
    pub fn new(fields: Value) -> Self {
        Self { fields }
    }

    fn validate(&self) -> SDKResult<()> {
        if self.fields.is_null() {
            return Err(openlark_core::error::validation_error(
                "新建职位请求体不能为空",
                "请传入有效的请求参数",
            ));
        }

        Ok(())
    }
}

/// 新建职位响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CombinedCreateResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for CombinedCreateResponse {
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
    fn test_combined_create_request_builder_new() {
        let request = CombinedCreateRequest::new(TestConfigBuilder::new().build());
        let _ = request;
    }
}
