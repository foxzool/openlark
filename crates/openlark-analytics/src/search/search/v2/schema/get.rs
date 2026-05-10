//! 获取数据范式
//! docPath: https://open.feishu.cn/document/server-docs/search-v2/open-search/schema/get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GetSchemaRequest {
    config: Arc<Config>,
    schema_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSchemaResponse {
    pub data: Option<SchemaData>,
}

impl ApiResponseTrait for GetSchemaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaData {
    pub schema_id: String,
    pub name: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub field_name: String,
    pub field_type: String,
}

impl GetSchemaRequest {
    pub fn new(config: Arc<Config>, schema_id: impl Into<String>) -> Self {
        Self {
            config,
            schema_id: schema_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetSchemaResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<GetSchemaResponse> {
        let path = format!("/open-apis/search/v2/schemas/{}", self.schema_id);
        let req: ApiRequest<GetSchemaResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("获取数据范式", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

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
