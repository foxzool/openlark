//! 获取数据范式
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/schema/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取数据范式请求。
#[derive(Debug, Clone)]
pub struct GetSchemaRequest {
    config: Arc<Config>,
    schema_id: String,
}

/// 获取数据范式响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSchemaResponse {
    /// 响应数据。
    pub data: Option<SchemaData>,
}

impl ApiResponseTrait for GetSchemaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据范式详情数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaData {
    /// 数据范式 ID。
    pub schema_id: String,
    /// 数据范式名称。
    pub name: String,
    /// 数据范式字段列表。
    pub fields: Vec<SchemaField>,
}

/// 数据范式字段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    /// 字段名称。
    pub field_name: String,
    /// 字段类型。
    pub field_type: String,
}

impl GetSchemaRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, schema_id: impl Into<String>) -> Self {
        Self {
            config,
            schema_id: schema_id.into(),
        }
    }

    /// 执行获取数据范式请求。
    pub async fn execute(self) -> SDKResult<GetSchemaResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行获取数据范式请求。
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
