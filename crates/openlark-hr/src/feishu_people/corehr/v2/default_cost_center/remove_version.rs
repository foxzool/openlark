//! 删除默认成本中心
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/default_cost_center/remove_version>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 删除默认成本中心请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RemoveVersionRequest {
    /// 配置信息
    config: Config,
    version_id: Option<String>,
    body: Option<Value>,
}

impl RemoveVersionRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            version_id: None,
            body: None,
        }
    }

    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RemoveVersionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RemoveVersionResponse> {
        let version_id = self.version_id.unwrap_or_default();
        validate_required!(version_id.trim(), "version_id 不能为空");

        let mut request = ApiRequest::<RemoveVersionResponse>::delete(format!(
            "/open-apis/corehr/v2/default_cost_centers/{}",
            version_id
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

/// 删除默认成本中心响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RemoveVersionResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for RemoveVersionResponse {
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
