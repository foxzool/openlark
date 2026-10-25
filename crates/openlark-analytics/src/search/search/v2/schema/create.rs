//! 创建数据范式
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/schema/create>

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

use super::get::Schema;

/// 数据范式的属性定义（创建用；name/type 必填）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaProperty {
    /// 属性名（必填，0-20 字符）。
    pub name: String,
    /// 属性类型（必填：text / int / tag / timestamp / double / tinytext）。
    #[serde(rename = "type")]
    pub type_: String,
    /// 该属性是否可用作搜索，默认为 false。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_searchable: Option<bool>,
    /// 该属性是否可用作搜索结果排序，默认为 false。为 true 时需配置 sort_options。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sortable: Option<bool>,
    /// 该属性是否可用作返回字段，默认为 false。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_returnable: Option<bool>,
    /// 属性排序的可选配置（is_sortable 为 true 时必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_options: Option<SchemaSortOptions>,
    /// 相关类型数据的定义和约束。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_definitions: Option<SchemaTypeDefinitions>,
    /// 属性搜索的可选配置（is_searchable 为 true 时必填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_options: Option<SchemaSearchOptions>,
}

/// 属性排序配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaSortOptions {
    /// 排序的优先级，0~4，0 为最高优先级，默认 0。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 排序的顺序：asc-升序，desc-降序（默认 desc）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// 相关类型数据的定义和约束。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaTypeDefinitions {
    /// 标签类型的定义（最多 100 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<SchemaTagOptions>>,
}

/// 标签类型的定义（name/color/text 必填）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaTagOptions {
    /// tag 对应的枚举值名称（必填，≤20 字符）。
    pub name: String,
    /// 标签对应的颜色（必填：red / green / blue / grey / yellow）。
    pub color: String,
    /// 标签中展示的文本（必填，≤8 字符）。
    pub text: String,
}

/// 属性搜索的可选配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaSearchOptions {
    /// 是否支持语义切词召回。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_semantic_match: Option<bool>,
    /// 是否支持精确匹配。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_exact_match: Option<bool>,
    /// 是否支持前缀匹配（前缀长度 1-12）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_prefix_match: Option<bool>,
    /// 是否支持数字后缀匹配（后缀长度 3-12）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_number_suffix_match: Option<bool>,
    /// 是否支持驼峰英文匹配。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_camel_match: Option<bool>,
}

/// 数据展示相关配置（card_key 必填）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaDisplay {
    /// 搜索数据的展示卡片（必填，当前可选 search_common_card）。
    pub card_key: String,
    /// 数据字段名称和展示字段名称的映射关系；未设置时只展示与展示字段同名的数据字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_mapping: Option<Vec<SchemaDisplayFieldMapping>>,
}

/// 数据字段与展示字段的映射（display_field/data_field 必填）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaDisplayFieldMapping {
    /// 展示字段名称（必填），与 card_key 有关，不能重复。
    pub display_field: String,
    /// 数据字段的名称（必填），需使用 ${xxx} 规则描述。
    pub data_field: String,
}

/// 创建数据范式请求体（schema_id/properties/display 均必填）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateSchemaBody {
    /// 用户自定义数据范式的唯一标识（必填，≤40 字符，`^[a-zA-Z][a-zA-Z0-9-_].*$`）。
    pub schema_id: String,
    /// 数据范式的属性定义（必填）。
    pub properties: Vec<SchemaProperty>,
    /// 数据展示相关配置（必填）。
    pub display: SchemaDisplay,
}

/// 创建数据范式请求。
#[derive(Debug, Clone)]
pub struct CreateSchemaRequest {
    config: Arc<Config>,
    body: CreateSchemaBody,
    /// 是否只用来校验合法性（查询参数，默认 false）。
    validate_only: Option<bool>,
}

/// 创建数据范式响应。
///
/// 官方响应 `data` 内容形如 `{ "schema": { ... } }`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSchemaResponse {
    /// 数据范式实例。
    #[serde(default)]
    pub schema: Option<Schema>,
}

impl ApiResponseTrait for CreateSchemaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateSchemaRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateSchemaBody::default(),
            validate_only: None,
        }
    }

    /// 设置用户自定义数据范式的唯一标识（必填，≤40 字符）。
    pub fn schema_id(mut self, schema_id: impl Into<String>) -> Self {
        self.body.schema_id = schema_id.into();
        self
    }

    /// 设置数据范式的属性定义（必填）。
    pub fn properties(mut self, properties: Vec<SchemaProperty>) -> Self {
        self.body.properties = properties;
        self
    }

    /// 追加一条属性定义。
    pub fn add_property(mut self, property: SchemaProperty) -> Self {
        self.body.properties.push(property);
        self
    }

    /// 设置数据展示相关配置（必填，card_key 必填）。
    pub fn display(mut self, display: SchemaDisplay) -> Self {
        self.body.display = display;
        self
    }

    /// 设置是否只用来校验合法性（查询参数，默认 false）。
    pub fn validate_only(mut self, validate_only: bool) -> Self {
        self.validate_only = Some(validate_only);
        self
    }

    /// 执行创建数据范式请求。
    pub async fn execute(self) -> SDKResult<CreateSchemaResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行创建数据范式请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateSchemaResponse> {
        validate_required!(self.body.schema_id, "schema_id 不能为空");
        validate_required!(self.body.properties, "properties 不能为空");
        validate_required!(self.body.display.card_key, "display.card_key 不能为空");
        let path = "/open-apis/search/v2/schemas".to_string();
        let req: ApiRequest<CreateSchemaResponse> = ApiRequest::post(&path)
            .body(serde_json::to_value(&self.body)?)
            .query_opt("validate_only", self.validate_only.map(|v| v.to_string()));

        Transport::request_typed(req, &self.config, Some(option), "创建数据范式").await
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

    /// 端到端：POST /open-apis/search/v2/schemas + 官方形状请求体与 validate_only 查询参数。
    #[tokio::test]
    async fn test_create_schema_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/search/v2/schemas"))
            .and(wiremock::matchers::query_param("validate_only", "true"))
            .and(wiremock::matchers::body_partial_json(json!({
                "schema_id": "example_schema",
                "properties": [
                    {
                        "name": "description",
                        "type": "text",
                        "is_searchable": true,
                        "is_returnable": true
                    }
                ],
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
                        "schema_id": "example_schema",
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

        let resp = CreateSchemaRequest::new(config)
            .schema_id("example_schema")
            .properties(vec![SchemaProperty {
                name: "description".to_string(),
                type_: "text".to_string(),
                is_searchable: Some(true),
                is_sortable: None,
                is_returnable: Some(true),
                sort_options: None,
                type_definitions: None,
                search_options: None,
            }])
            .display(SchemaDisplay {
                card_key: "search_common_card".to_string(),
                fields_mapping: Some(vec![SchemaDisplayFieldMapping {
                    display_field: "summary".to_string(),
                    data_field: "${description}".to_string(),
                }]),
            })
            .validate_only(true)
            .execute()
            .await
            .expect("创建数据范式应成功");
        assert_eq!(
            resp.schema.and_then(|s| s.schema_id),
            Some("example_schema".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/search/v2/schemas");
        // 请求体顶层字段与官方文档对齐（type 为 serde rename）
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["schema_id"], "example_schema");
        assert_eq!(sent["properties"][0]["type"], "text");
        assert_eq!(sent["properties"][0]["is_searchable"], true);
        assert_eq!(sent["display"]["card_key"], "search_common_card");
        assert_eq!(
            sent["display"]["fields_mapping"][0]["data_field"],
            "${description}"
        );
    }

    /// 缺必填 schema_id 时快速失败，不发请求。
    #[tokio::test]
    async fn test_create_schema_rejects_empty_schema_id() {
        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .build(),
        );
        let err = CreateSchemaRequest::new(config)
            .execute()
            .await
            .expect_err("缺 schema_id 应报错");
        assert!(err.to_string().contains("schema_id"));
    }
}
