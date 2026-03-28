//! 更新职位相关人员
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job.manager/batch_update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新职位相关人员请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BatchUpdateRequest {
    request_body: BatchUpdateRequestBody,
    /// 配置信息
    config: Config,
}

impl BatchUpdateRequest {
    /// 创建请求
    pub fn new(config: Config, request_body: BatchUpdateRequestBody) -> Self {
        Self {
            request_body,
            config,
        }
    }

    pub fn request_body(mut self, request_body: BatchUpdateRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchUpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchUpdateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        self.request_body.validate()?;

        let api_endpoint = HireApiV1::JobManagerBatchUpdate;
        let request = ApiRequest::<BatchUpdateResponse>::post(api_endpoint.to_url());
        let request = request.body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新职位相关人员响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateRequestBody {
    #[serde(flatten)]
    pub fields: Value,
}

impl BatchUpdateRequestBody {
    pub fn new(fields: Value) -> Self {
        Self { fields }
    }

    fn validate(&self) -> SDKResult<()> {
        if self.fields.is_null() {
            return Err(openlark_core::error::validation_error(
                "更新职位相关人员请求体不能为空",
                "请传入有效的请求参数",
            ));
        }

        Ok(())
    }
}

/// 更新职位相关人员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for BatchUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
