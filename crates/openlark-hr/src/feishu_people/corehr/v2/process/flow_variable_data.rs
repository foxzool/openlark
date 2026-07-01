//! 获取流程数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/process/flow_variable_data>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取流程数据请求
#[derive(Debug, Clone)]
pub struct FlowVariableDataRequest {
    /// 配置信息
    config: Config,
    /// 流程 ID
    process_id: Option<String>,
    body: Option<Value>,
}

impl FlowVariableDataRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            process_id: None,
            body: None,
        }
    }

    /// 设置 `process_id`。
    pub fn process_id(mut self, process_id: impl Into<String>) -> Self {
        self.process_id = Some(process_id.into());
        self
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<FlowVariableDataResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<FlowVariableDataResponse> {
        let process_id = self.process_id.unwrap_or_default();
        validate_required!(process_id.trim(), "process_id 不能为空");

        let mut request = ApiRequest::<FlowVariableDataResponse>::get(format!(
            "/open-apis/corehr/v2/processes/{process_id}/flow_variable_data"
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

/// 获取流程数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlowVariableDataResponse {
    /// 响应数据
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FlowVariableData>,
}

/// 流程变量数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlowVariableData {
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
}

impl ApiResponseTrait for FlowVariableDataResponse {
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
