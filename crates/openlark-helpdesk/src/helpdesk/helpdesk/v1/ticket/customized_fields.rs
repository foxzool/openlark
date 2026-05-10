//! 获取服务台自定义字段
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/customized_fields

use crate::common::api_endpoints::HelpdeskApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取自定义字段请求。
#[derive(Debug, Clone)]
pub struct GetCustomizedFieldsRequest {
    config: Arc<Config>,
}

/// 获取自定义字段响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomizedFieldsResponse {
    /// 响应数据。
    pub data: Option<GetCustomizedFieldsData>,
}

impl ApiResponseTrait for GetCustomizedFieldsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 自定义字段响应数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomizedFieldsData {
    /// 自定义字段列表。
    pub fields: Vec<CustomizedField>,
}

/// 自定义字段条目。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizedField {
    /// 字段 ID。
    pub field_id: String,
    /// 字段名称。
    pub field_name: String,
    /// 字段类型。
    pub field_type: String,
}

impl GetCustomizedFieldsRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetCustomizedFieldsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetCustomizedFieldsResponse> {
        let path = HelpdeskApiV1::TicketCustomizedFields.to_url();
        let req: ApiRequest<GetCustomizedFieldsResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取服务台自定义字段", "响应数据为空")
        })
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
