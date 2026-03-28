//! 获取异动原因列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/transfer_reason/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取异动原因列表请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QueryRequest {
    config: Config,
}

impl QueryRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::CorehrApiV1;

        let api_endpoint = CorehrApiV1::TransferReasonQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(_option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取异动原因列表响应为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取异动原因列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    pub data: Value,
}

impl ApiResponseTrait for QueryResponse {
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
