//! 更新职位
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/combined_update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新职位请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CombinedUpdateRequest {
    request_body: CombinedUpdateRequestBody,
    /// 配置信息
    config: Config,
}

impl CombinedUpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            request_body: CombinedUpdateRequestBody::default(),
            config,
        }
    }

    pub fn request_body(mut self, request_body: CombinedUpdateRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CombinedUpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CombinedUpdateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        self.request_body.validate()?;

        let api_endpoint = HireApiV1::JobCombinedUpdate;
        let request = ApiRequest::<CombinedUpdateResponse>::post(api_endpoint.to_url());
        let request = request.body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新职位响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CombinedUpdateRequestBody {
    #[serde(flatten)]
    pub fields: Value,
}

impl CombinedUpdateRequestBody {
    pub fn new(fields: Value) -> Self {
        Self { fields }
    }

    fn validate(&self) -> SDKResult<()> {
        if self.fields.is_null() {
            return Err(openlark_core::error::validation_error(
                "更新职位请求体不能为空",
                "请传入有效的请求参数",
            ));
        }

        Ok(())
    }
}

/// 更新职位响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CombinedUpdateResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for CombinedUpdateResponse {
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
    fn test_combined_update_request_builder_new() {
        let request = CombinedUpdateRequest::new(TestConfigBuilder::new().build());
        let _ = request;
    }
}
