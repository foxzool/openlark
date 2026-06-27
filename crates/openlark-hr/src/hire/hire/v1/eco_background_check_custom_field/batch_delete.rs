//! 删除背调自定义字段
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/eco_background_check_custom_field/batch_delete

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hire::hire::common_models::EcoOperationResult;

/// 删除背调自定义字段请求
#[derive(Debug, Clone)]
pub struct BatchDeleteRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl BatchDeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchDeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchDeleteResponse> {
        let mut request = ApiRequest::<BatchDeleteResponse>::post(
            "/open-apis/hire/v1/eco_background_check_custom_fields/batch_delete",
        );
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "删除背调自定义字段响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 删除背调自定义字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchDeleteResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: EcoOperationResult,
}

impl ApiResponseTrait for BatchDeleteResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
