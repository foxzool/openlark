//! 修改数据范式
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/schema/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::create::SchemaDisplay;
use super::get::Schema;

/// 修改数据范式请求体。
///
/// 官方文档请求体唯一字段为 `display`（选填）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchSchemaBody {
    /// 数据展示相关配置（card_key 必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<SchemaDisplay>,
}

/// 修改数据范式请求。
#[derive(Debug, Clone)]
pub struct PatchSchemaRequest {
    config: Arc<Config>,
    schema_id: String,
    body: PatchSchemaBody,
}

/// 修改数据范式响应。
///
/// 官方响应 `data` 内容形如 `{ "schema": { ... } }`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchSchemaResponse {
    /// 数据范式实例。
    #[serde(default)]
    pub schema: Option<Schema>,
}

impl ApiResponseTrait for PatchSchemaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchSchemaRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, schema_id: impl Into<String>) -> Self {
        Self {
            config,
            schema_id: schema_id.into(),
            body: PatchSchemaBody::default(),
        }
    }

    /// 设置数据展示相关配置（card_key 必填）。
    pub fn display(mut self, display: SchemaDisplay) -> Self {
        self.body.display = Some(display);
        self
    }

    /// 执行修改数据范式请求。
    pub async fn execute(self) -> SDKResult<PatchSchemaResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行修改数据范式请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchSchemaResponse> {
        validate_required!(self.schema_id, "schema_id 不能为空");
        if let Some(display) = &self.body.display {
            validate_required!(display.card_key, "display.card_key 不能为空");
        }
        let path = format!("/open-apis/search/v2/schemas/{}", self.schema_id);
        let req: ApiRequest<PatchSchemaResponse> =
            ApiRequest::patch(&path).body(serde_json::to_value(&self.body)?);

        Transport::request_typed(req, &self.config, Some(option), "修改数据范式").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::search::search::v2::schema::create::SchemaDisplayFieldMapping;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH .../schemas/{schema_id} + display 请求体 → 响应解析。
    #[tokio::test]
    async fn test_patch_schema_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/search/v2/schemas/sch_001"))
            .and(wiremock::matchers::body_partial_json(json!({
                "display": {
                    "card_key": "search_common_card",
                    "fields_mapping": [
                        { "data_field": "${description}", "display_field": "summary" }
                    ]
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "schema": {
                        "schema_id": "sch_001",
                        "display": { "card_key": "search_common_card" },
                        "properties": []
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

        let resp = PatchSchemaRequest::new(config, "sch_001")
            .display(SchemaDisplay {
                card_key: "search_common_card".to_string(),
                fields_mapping: Some(vec![SchemaDisplayFieldMapping {
                    display_field: "summary".to_string(),
                    data_field: "${description}".to_string(),
                }]),
            })
            .execute()
            .await
            .expect("修改数据范式应成功");
        assert_eq!(
            resp.schema.and_then(|s| s.schema_id),
            Some("sch_001".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/search/v2/schemas/sch_001"
        );
        // 请求体携带官方文档的 display 配置
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["display"]["card_key"], "search_common_card");
        assert_eq!(
            sent["display"]["fields_mapping"][0]["display_field"],
            "summary"
        );
        assert_eq!(
            sent["display"]["fields_mapping"][0]["data_field"],
            "${description}"
        );
    }

    /// 未设置 display 时 body 序列化为空对象（不发虚构字段）。
    #[test]
    fn test_patch_schema_body_omits_absent_fields() {
        let body = PatchSchemaBody::default();
        assert_eq!(serde_json::to_value(&body).unwrap(), json!({}));
    }
}
