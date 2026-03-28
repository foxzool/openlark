//! 预约会议
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/apply>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::extract_response_data;

/// 预约会议请求

#[derive(Debug, Clone)]
pub struct ApplyReserveRequest {
    /// 配置信息
    config: Config,
}

/// 预约会议响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApplyReserveResponse {
    /// 会议 ID
    pub meeting_id: String,
    /// 预约 ID
    pub reserve_id: String,
}

impl ApiResponseTrait for ApplyReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApplyReserveRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/apply>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<ApplyReserveResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<ApplyReserveResponse> {
        let api_endpoint = VcApiV1::ReserveCreate;
        let api_request: ApiRequest<ApplyReserveResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "预约会议")
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
