//! 创建数据源
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source/create>

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

use super::get::DataSource;

/// 创建数据源请求体。
///
/// `name` 必填；其余字段可选（官方文档 Required=No）；
/// `i18n_name`/`i18n_description` 为多语言配置（i18n_meta），以 JSON 值透传。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDataSourceBody {
    /// 数据源展示名称（必填）。
    pub name: String,
    /// 数据源状态：0-已上线，1-未上线。未填默认未上线。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    /// 对于数据源的描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据源在 search tab 上的展示图标路径，建议 png 或 jpeg 格式。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// 数据源采用的展示模版名称，默认 search_common_card。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// 【已废弃，如有定制需要请使用"数据范式"接口】描述哪些字段可以被搜索。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_fields: Option<Vec<String>>,
    /// 数据源的国际化展示名称（i18n_meta，json 格式）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<serde_json::Value>,
    /// 数据源的国际化描述（i18n_meta，json 格式）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<serde_json::Value>,
    /// 数据源关联的 schema 标识。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
}

/// 创建数据源请求。
#[derive(Debug, Clone)]
pub struct CreateDataSourceRequest {
    config: Arc<Config>,
    body: CreateDataSourceBody,
}

/// 创建数据源响应。
///
/// 官方响应 `data` 内容形如 `{ "data_source": { ... } }`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDataSourceResponse {
    /// 数据源实例。
    #[serde(default)]
    pub data_source: Option<DataSource>,
}

impl ApiResponseTrait for CreateDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateDataSourceRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateDataSourceBody::default(),
        }
    }

    /// 设置数据源展示名称（必填）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置数据源状态（0-已上线，1-未上线）。
    pub fn state(mut self, state: i32) -> Self {
        self.body.state = Some(state);
        self
    }

    /// 设置数据源描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 设置数据源展示图标路径。
    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.body.icon_url = Some(icon_url.into());
        self
    }

    /// 设置数据源展示模版名称。
    pub fn template(mut self, template: impl Into<String>) -> Self {
        self.body.template = Some(template.into());
        self
    }

    /// 设置可搜索字段（官方已废弃，建议改用数据范式接口）。
    pub fn searchable_fields(mut self, searchable_fields: Vec<String>) -> Self {
        self.body.searchable_fields = Some(searchable_fields);
        self
    }

    /// 设置数据源名称多语言配置（json 格式）。
    pub fn i18n_name(mut self, i18n_name: serde_json::Value) -> Self {
        self.body.i18n_name = Some(i18n_name);
        self
    }

    /// 设置数据源描述多语言配置（json 格式）。
    pub fn i18n_description(mut self, i18n_description: serde_json::Value) -> Self {
        self.body.i18n_description = Some(i18n_description);
        self
    }

    /// 设置数据源关联的 schema 标识。
    pub fn schema_id(mut self, schema_id: impl Into<String>) -> Self {
        self.body.schema_id = Some(schema_id.into());
        self
    }

    /// 执行创建数据源请求。
    pub async fn execute(self) -> SDKResult<CreateDataSourceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行创建数据源请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateDataSourceResponse> {
        validate_required!(self.body.name, "name 不能为空");
        let path = "/open-apis/search/v2/data_sources".to_string();
        let req: ApiRequest<CreateDataSourceResponse> =
            ApiRequest::post(&path).body(serde_json::to_value(&self.body)?);

        Transport::request_typed(req, &self.config, Some(option), "创建数据源").await
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

    /// 端到端：POST /open-apis/search/v2/data_sources + 官方形状请求体 → 响应解析。
    #[tokio::test]
    async fn test_create_data_source_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/search/v2/data_sources"))
            .and(wiremock::matchers::body_partial_json(json!({
                "name": "客服工单",
                "state": 0,
                "schema_id": "example_schema"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "data_source": {
                        "id": "6953903108179099667",
                        "name": "客服工单",
                        "state": 0,
                        "is_exceed_quota": false
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

        let resp = CreateDataSourceRequest::new(config)
            .name("客服工单")
            .state(0)
            .description("搜索客服工单数据")
            .icon_url("https://www.xxx.com/open.jpg")
            .template("search_common_card")
            .schema_id("example_schema")
            .execute()
            .await
            .expect("创建数据源应成功");
        assert_eq!(
            resp.data_source.and_then(|ds| ds.id),
            Some("6953903108179099667".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/search/v2/data_sources");
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["name"], "客服工单");
        assert_eq!(sent["state"], 0);
        assert_eq!(sent["description"], "搜索客服工单数据");
        assert_eq!(sent["icon_url"], "https://www.xxx.com/open.jpg");
        assert_eq!(sent["template"], "search_common_card");
        assert_eq!(sent["schema_id"], "example_schema");
        // 未设置的可选字段不序列化
        assert!(sent.get("searchable_fields").is_none());
        assert!(sent.get("i18n_name").is_none());
    }

    /// 未设置任何可选字段时 body 仅含必填 name。
    #[test]
    fn test_create_data_source_body_requires_name_only() {
        let body = CreateDataSourceBody {
            name: "客服工单".to_string(),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({ "name": "客服工单" })
        );
    }
}
