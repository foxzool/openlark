//! 修改数据源
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source/patch>

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

/// 修改数据源请求体。
///
/// 全部字段可选（官方文档 Required=No）；`i18n_name`/`i18n_description`
/// 为多语言配置（i18n_meta，json 格式），以 JSON 值透传。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchDataSourceBody {
    /// 数据源展示名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据源状态：0-上线，1-下线。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    /// 数据源描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据源在搜索 Tab 中展示图标的路径。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// 数据源名称多语言配置（json 格式）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<serde_json::Value>,
    /// 数据源描述多语言配置（json 格式）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<serde_json::Value>,
}

/// 修改数据源请求。
#[derive(Debug, Clone)]
pub struct PatchDataSourceRequest {
    config: Arc<Config>,
    data_source_id: String,
    body: PatchDataSourceBody,
}

/// 修改数据源响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDataSourceResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchDataSourceRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, data_source_id: impl Into<String>) -> Self {
        Self {
            config,
            data_source_id: data_source_id.into(),
            body: PatchDataSourceBody::default(),
        }
    }

    /// 设置数据源展示名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    /// 设置数据源状态（0-上线，1-下线）。
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

    /// 执行修改数据源请求。
    pub async fn execute(self) -> SDKResult<PatchDataSourceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行修改数据源请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchDataSourceResponse> {
        validate_required!(self.data_source_id, "data_source_id 不能为空");
        let path = format!("/open-apis/search/v2/data_sources/{}", self.data_source_id);
        let req: ApiRequest<PatchDataSourceResponse> =
            ApiRequest::patch(&path).body(serde_json::to_value(&self.body)?);

        Transport::request_typed(req, &self.config, Some(option), "修改数据源").await
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

    /// 端到端：PATCH .../data_sources/{data_source_id} + body 字段下发 → 响应解析。
    #[tokio::test]
    async fn test_patch_data_source_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/search/v2/data_sources/ds_001"))
            .and(wiremock::matchers::body_partial_json(json!({
                "name": "Customer service",
                "state": 0
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": {} }
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

        let resp = PatchDataSourceRequest::new(config, "ds_001")
            .name("Customer service")
            .state(0)
            .description("Search Customer service")
            .icon_url("https://www.xxx.com/open.jpg")
            .execute()
            .await
            .expect("修改数据源应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/search/v2/data_sources/ds_001"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["name"], "Customer service");
        assert_eq!(sent["state"], 0);
        assert_eq!(sent["description"], "Search Customer service");
        assert_eq!(sent["icon_url"], "https://www.xxx.com/open.jpg");
    }

    /// 未设置任何字段时 body 序列化为空对象（不发虚构字段）。
    #[test]
    fn test_patch_data_source_body_omits_absent_fields() {
        let body = PatchDataSourceBody::default();
        assert_eq!(serde_json::to_value(&body).unwrap(), json!({}));
    }
}
