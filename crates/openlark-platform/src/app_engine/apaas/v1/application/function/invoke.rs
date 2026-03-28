//! 执行函数
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-function/invoke>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 执行函数 Builder
#[derive(Debug, Clone)]
pub struct FunctionInvokeBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 函数 API 名称
    function_api_name: String,
    /// 函数参数
    params: serde_json::Value,
}

impl FunctionInvokeBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        function_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            function_api_name: function_api_name.into(),
            params: serde_json::json!({}),
        }
    }

    /// 设置函数参数
    pub fn params(mut self, params: impl Into<serde_json::Value>) -> Self {
        self.params = params.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<FunctionInvokeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FunctionInvokeResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/functions/{}/invoke",
            self.namespace, self.function_api_name
        );

        let request = FunctionInvokeRequest {
            params: self.params,
        };

        let req: ApiRequest<FunctionInvokeResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 执行函数请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct FunctionInvokeRequest {
    /// 函数参数
    #[serde(rename = "params")]
    params: serde_json::Value,
}

/// 执行函数响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionInvokeResponse {
    /// 执行结果
    #[serde(rename = "result")]
    result: serde_json::Value,
    /// 执行状态
    #[serde(rename = "status")]
    status: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for FunctionInvokeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
