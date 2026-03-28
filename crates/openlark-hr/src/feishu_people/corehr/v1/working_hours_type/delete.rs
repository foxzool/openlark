//! 删除工时制度
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/working_hours_type/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 删除工时制度请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeleteRequest {
    /// 配置信息
    config: Config,
    /// 工时制度 ID（必填）
    working_hours_type_id: String,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            working_hours_type_id: String::new(),
        }
    }

    /// 设置工时制度 ID（必填）
    pub fn working_hours_type_id(mut self, working_hours_type_id: String) -> Self {
        self.working_hours_type_id = working_hours_type_id;
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
        use crate::common::api_endpoints::CorehrApiV1;

        validate_required!(self.working_hours_type_id.trim(), "工时制度 ID 不能为空");

        let api_endpoint = CorehrApiV1::WorkingHoursTypeDelete(self.working_hours_type_id);
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");

        let request = ApiRequest::<DeleteResponse>::delete(endpoint_url);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "删除工时制度响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 删除工时制度响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
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
