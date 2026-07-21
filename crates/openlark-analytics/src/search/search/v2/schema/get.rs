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

        Transport::request_typed(req, &self.config, Some(option), "获取数据范式").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/search/v2/schemas/{schema_id} → 响应解析。
    #[tokio::test]
    async fn test_get_schema_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/search/v2/schemas/sch_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "data": {
                        "schema_id": "sch_001",
                        "name": "n",
                        "fields": []
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = GetSchemaRequest::new(config, "sch_001")
            .execute()
            .await
            .expect("获取数据范式应成功");
        assert_eq!(resp.data.unwrap().schema_id, "sch_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/search/v2/schemas/sch_001"
        );
    }
}
