//! 获取数据源
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取数据源请求。
#[derive(Debug, Clone)]
pub struct GetDataSourceRequest {
    config: Arc<Config>,
    data_source_id: String,
}

/// 获取数据源响应。
///
/// 官方响应 `data` 内容形如 `{ "data_source": { ... } }`，
/// 本结构即 `data` 的内容类型（信封由 Transport 处理）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataSourceResponse {
    /// 数据源实例。
    #[serde(default)]
    pub data_source: Option<DataSource>,
}

impl ApiResponseTrait for GetDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 多语言配置（i18n_meta）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nMeta {
    /// 国际化字段：中文。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 国际化字段：英文。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    /// 国际化字段：日文。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

/// 数据源详情（data_source）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataSource {
    /// 数据源的唯一标识。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 数据源的展示名称。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据源状态：0-已上线，1-未上线。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    /// 对于数据源的描述。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间，Unix 时间戳，单位为秒。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间，Unix 时间戳，单位为秒。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 是否超限。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_exceed_quota: Option<bool>,
    /// 数据源在 search tab 上的展示图标路径。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// 数据源采用的展示模版名称。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// 【已废弃，如有定制需要请使用"数据范式"接口】描述哪些字段可以被搜索。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub searchable_fields: Option<Vec<String>>,
    /// 数据源的国际化展示名称。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<I18nMeta>,
    /// 数据源的国际化描述。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<I18nMeta>,
    /// 数据源关联的 schema 标识。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    /// datasource 对应的开放平台应用 id。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl GetDataSourceRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, data_source_id: impl Into<String>) -> Self {
        Self {
            config,
            data_source_id: data_source_id.into(),
        }
    }

    /// 执行获取数据源请求。
    pub async fn execute(self) -> SDKResult<GetDataSourceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行获取数据源请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetDataSourceResponse> {
        let path = format!("/open-apis/search/v2/data_sources/{}", self.data_source_id);
        let req: ApiRequest<GetDataSourceResponse> = ApiRequest::get(&path);

        Transport::request_typed(req, &self.config, Some(option), "获取数据源").await
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

    /// 端到端：GET /open-apis/search/v2/data_sources/{data_source_id} → 官方形状响应解析。
    #[tokio::test]
    async fn test_get_data_source_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/search/v2/data_sources/ds_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "data_source": {
                        "id": "ds_001",
                        "name": "客服工单",
                        "state": 0,
                        "description": "搜索客服工单数据",
                        "create_time": "1618829192",
                        "update_time": "1618829192",
                        "is_exceed_quota": true,
                        "icon_url": "https://www.xxx.com/open.jpg",
                        "template": "search_common_card",
                        "i18n_name": { "zh_cn": "任务", "en_us": "TODO", "ja_jp": "タスク" },
                        "schema_id": "custom_schema",
                        "app_id": "cli_a1306bed4738d01b"
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

        let resp = GetDataSourceRequest::new(config, "ds_001")
            .execute()
            .await
            .expect("获取数据源应成功");
        let ds = resp.data_source.expect("data_source 应存在");
        assert_eq!(ds.id.as_deref(), Some("ds_001"));
        assert_eq!(ds.name.as_deref(), Some("客服工单"));
        assert_eq!(ds.state, Some(0));
        assert_eq!(ds.is_exceed_quota, Some(true));
        assert_eq!(
            ds.i18n_name.as_ref().and_then(|m| m.zh_cn.as_deref()),
            Some("任务")
        );
        assert_eq!(ds.schema_id.as_deref(), Some("custom_schema"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/search/v2/data_sources/ds_001"
        );
    }
}
