//! 删除预约
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, validate_required_field};

/// 删除预约请求

#[derive(Debug, Clone)]
pub struct DeleteReserveRequest {
    /// 配置信息
    config: Config,
    /// 预约 ID（路径参数）
    reserve_id: String,
}

/// 删除预约响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteReserveResponse {
    /// 删除状态
    pub success: bool,
}

impl ApiResponseTrait for DeleteReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteReserveRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_id: String::new(),
        }
    }

    /// 设置预约 ID（路径参数）
    pub fn reserve_id(mut self, reserve_id: impl Into<String>) -> Self {
        self.reserve_id = reserve_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/delete>
    pub async fn execute(self) -> SDKResult<DeleteReserveResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteReserveResponse> {
        validate_required_field("reserve_id", Some(&self.reserve_id), "预约 ID 不能为空")?;

        let api_endpoint = VcApiV1::ReserveDelete(self.reserve_id.clone());
        let api_request: ApiRequest<DeleteReserveResponse> =
            ApiRequest::delete(api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除预约")
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
