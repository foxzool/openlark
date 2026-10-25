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
///
/// 官方响应 `data` 内容形如 `{ "schema": { ... } }`，
/// 本结构即 `data` 的内容类型（信封由 Transport 处理）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSchemaResponse {
    /// 数据范式。
    #[serde(default)]
    pub schema: Option<Schema>,
}

impl ApiResponseTrait for GetSchemaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据范式详情（schema）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Schema {
    /// 用户自定义数据范式的唯一标识。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    /// 数据范式的属性定义。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<SchemaProperty>>,
    /// 数据展示相关配置。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<SchemaDisplay>,
}

/// 数据范式的属性定义（schema_property）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaProperty {
    /// 属性名。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 属性类型（text / int / tag / timestamp / double / tinytext）。
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 该属性是否可用作搜索，默认为 false。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_searchable: Option<bool>,
    /// 该属性是否可用作搜索结果排序，默认为 false。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_sortable: Option<bool>,
    /// 该属性是否可用作返回字段，默认为 false。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_returnable: Option<bool>,
    /// 属性排序的可选配置（is_sortable 为 true 时必填）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_options: Option<SchemaSortOptions>,
    /// 相关类型数据的定义和约束。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_definitions: Option<SchemaTypeDefinitions>,
    /// 属性搜索的可选配置（is_searchable 为 true 时必填）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_options: Option<SchemaSearchOptions>,
}

/// 属性排序配置（schema_sort_options）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaSortOptions {
    /// 排序的优先级，0~4，0 为最高优先级。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 排序的顺序：asc-升序，desc-降序（默认 desc）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// 相关类型数据的定义和约束（schema_type_definitions）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaTypeDefinitions {
    /// 标签类型的定义。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<SchemaTagOptions>>,
}

/// 标签类型的定义（schema_tag_options）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaTagOptions {
    /// tag 对应的枚举值名称。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 标签对应的颜色（red / green / blue / grey / yellow）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// 标签中展示的文本。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// 属性搜索的可选配置（schema_search_options）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaSearchOptions {
    /// 是否支持语义切词召回（默认不支持）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_semantic_match: Option<bool>,
    /// 是否支持精确匹配（默认不支持）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_exact_match: Option<bool>,
    /// 是否支持前缀匹配（前缀长度 1-12）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_prefix_match: Option<bool>,
    /// 是否支持数字后缀匹配（后缀长度 3-12）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_number_suffix_match: Option<bool>,
    /// 是否支持驼峰英文匹配。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_camel_match: Option<bool>,
}

/// 数据展示相关配置（schema_display）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaDisplay {
    /// 搜索数据的展示卡片（当前可选 search_common_card）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_key: Option<String>,
    /// 数据字段名称和展示字段名称的映射关系。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields_mapping: Option<Vec<SchemaDisplayFieldMapping>>,
}

/// 数据字段与展示字段的映射（schema_display_field_mapping）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaDisplayFieldMapping {
    /// 展示字段名称，与 card_key 有关，不能重复。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_field: Option<String>,
    /// 数据字段的名称，需使用 ${xxx} 规则描述。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_field: Option<String>,
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

    /// 端到端：GET /open-apis/search/v2/schemas/{schema_id} → 官方形状响应解析。
    #[tokio::test]
    async fn test_get_schema_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/search/v2/schemas/sch_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "schema": {
                        "schema_id": "sch_001",
                        "display": {
                            "card_key": "search_common_card",
                            "fields_mapping": [
                                { "data_field": "${description}", "display_field": "summary" }
                            ]
                        },
                        "properties": [
                            {
                                "name": "description",
                                "type": "text",
                                "is_searchable": true,
                                "is_returnable": true,
                                "search_options": { "enable_semantic_match": true }
                            },
                            {
                                "name": "priority",
                                "type": "tag",
                                "is_returnable": true,
                                "type_definitions": {
                                    "tag": [
                                        { "name": "HIGH", "color": "red", "text": "紧急" }
                                    ]
                                }
                            }
                        ]
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
        let schema = resp.schema.expect("schema 应存在");
        assert_eq!(schema.schema_id.as_deref(), Some("sch_001"));
        let display = schema.display.expect("display 应存在");
        assert_eq!(display.card_key.as_deref(), Some("search_common_card"));
        assert_eq!(
            display.fields_mapping.as_ref().unwrap()[0]
                .display_field
                .as_deref(),
            Some("summary")
        );
        let properties = schema.properties.expect("properties 应存在");
        assert_eq!(properties.len(), 2);
        assert_eq!(properties[0].name.as_deref(), Some("description"));
        assert_eq!(properties[0].type_.as_deref(), Some("text"));
        assert_eq!(properties[0].is_searchable, Some(true));
        assert_eq!(
            properties[0]
                .search_options
                .as_ref()
                .and_then(|o| o.enable_semantic_match),
            Some(true)
        );
        assert_eq!(
            properties[1]
                .type_definitions
                .as_ref()
                .and_then(|d| d.tag.as_ref())
                .map(|t| t.len()),
            Some(1)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/search/v2/schemas/sch_001"
        );
    }
}
