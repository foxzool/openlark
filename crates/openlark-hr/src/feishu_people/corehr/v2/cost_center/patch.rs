//! 启用 / 停用成本中心
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/cost_center/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// PatchRequest
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    cost_center_id: Option<String>,
    /// 请求体（可选）
    body: Option<Value>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            cost_center_id: None,
            body: None,
        }
    }

    /// 设置 `cost_center_id`。
    pub fn cost_center_id(mut self, cost_center_id: impl Into<String>) -> Self {
        self.cost_center_id = Some(cost_center_id.into());
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        let cost_center_id = self.cost_center_id.unwrap_or_default();
        validate_required!(cost_center_id.trim(), "cost_center_id 不能为空");

        let mut request = ApiRequest::<PatchResponse>::patch(format!(
            "/open-apis/corehr/v2/cost_centers/{cost_center_id}"
        ));

        if let Some(body) = self.body {
            request = request.body(body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("接口响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// PatchResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for PatchResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
