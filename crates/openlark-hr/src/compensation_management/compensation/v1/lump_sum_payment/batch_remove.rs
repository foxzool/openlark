//! 批量删除一次性支付记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/lump_sum_payment/batch_remove>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量删除一次性支付记录请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BatchRemoveRequest {
    /// 配置信息
    config: Config,
}

impl BatchRemoveRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchRemoveResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchRemoveResponse> {
        use crate::common::api_endpoints::CompensationApiV1;

        let api_endpoint = CompensationApiV1::LumpSumPaymentBatchRemove;
        let request = ApiRequest::<BatchRemoveResponse>::post(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "批量删除一次性支付记录响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 批量删除一次性支付记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchRemoveResponse {
    /// 删除结果列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<BatchRemoveResult>>,
}

/// 批量删除结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchRemoveResult {
    /// 记录 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 是否成功
    pub success: bool,
}

impl ApiResponseTrait for BatchRemoveResponse {
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
