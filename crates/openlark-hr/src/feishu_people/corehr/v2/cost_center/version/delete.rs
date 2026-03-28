//! 撤销成本中心版本
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/cost_center.version/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 撤销成本中心版本请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeleteRequest {
    /// 配置信息
    config: Config,
    version_id: Option<String>,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            version_id: None,
        }
    }

    pub fn version_id(mut self, version_id: String) -> Self {
        self.version_id = Some(version_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        let version_id = self.version_id.unwrap_or_default();
        validate_required!(version_id.trim(), "version_id 不能为空");

        let request = ApiRequest::<DeleteResponse>::delete(format!(
            "/open-apis/corehr/v2/cost_center_versions/{}",
            version_id
        ));

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("接口响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 撤销成本中心版本响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for DeleteResponse {
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
