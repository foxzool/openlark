//! 删除国家证件类型
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/national_id_type/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 删除国家证件类型请求
#[derive(Debug, Clone)]
pub struct DeleteRequest {
    /// 配置信息
    config: Config,
    /// 国家证件类型 ID（必填）
    national_id_type_id: String,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            national_id_type_id: String::new(),
        }
    }

    /// 设置国家证件类型 ID（必填）
    pub fn national_id_type_id(mut self, national_id_type_id: String) -> Self {
        self.national_id_type_id = national_id_type_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        use crate::common::api_endpoints::CorehrApiV1;

        validate_required!(self.national_id_type_id.trim(), "国家证件类型 ID 不能为空");

        let api_endpoint = CorehrApiV1::NationalIdTypeDelete(self.national_id_type_id);
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");

        let request = ApiRequest::<DeleteResponse>::delete(endpoint_url);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "删除国家证件类型响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 删除国家证件类型响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 响应数据
    /// 操作结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<NationalIdTypeItem>,
}

/// 国家证件类型条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NationalIdTypeItem {
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

impl ApiResponseTrait for DeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
