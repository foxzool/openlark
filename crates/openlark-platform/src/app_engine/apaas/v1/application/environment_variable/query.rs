//! 查询环境变量列表
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-environment_variable/query>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-environment_variable/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询环境变量列表 Builder
#[derive(Debug, Clone)]
pub struct EnvironmentVariableQueryRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl EnvironmentVariableQueryRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, namespace: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            page: None,
            page_size: None,
        }
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EnvironmentVariableQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EnvironmentVariableQueryResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/environment_variables/query",
            self.namespace
        );

        let request = EnvironmentVariableQueryRequest {
            page: self.page,
            page_size: self.page_size,
        };

        let req: ApiRequest<EnvironmentVariableQueryResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 查询环境变量列表请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EnvironmentVariableQueryRequest {
    /// 页码
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    /// 每页数量
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

/// 环境变量信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentVariableInfo {
    /// 环境变量 API 名称
    #[serde(rename = "api_name")]
    api_name: String,
    /// 环境变量名称
    #[serde(rename = "name")]
    name: String,
    /// 环境变量值
    #[serde(rename = "value")]
    value: String,
    /// 描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/// 查询环境变量列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentVariableQueryResponse {
    /// 环境变量列表
    #[serde(rename = "items")]
    pub items: Vec<EnvironmentVariableInfo>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    pub page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    pub page_size: u32,
}

impl ApiResponseTrait for EnvironmentVariableQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to EnvironmentVariableQueryRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type EnvironmentVariableQueryBuilder = EnvironmentVariableQueryRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../environment_variables/query → EnvironmentVariableQueryResponse。
    #[tokio::test]
    async fn test_query_environment_variable_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/environment_variables/query",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "api_name": "env_var_001",
                            "name": "ENV_NAME",
                            "value": "value_001",
                            "description": "测试变量"
                        }
                    ],
                    "has_more": false,
                    "page": 1,
                    "page_size": 20
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = EnvironmentVariableQueryRequestBuilder::new(config, "ns_test")
            .page(1)
            .page_size(20)
            .execute()
            .await
            .expect("查询环境变量列表应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].api_name, "env_var_001");
        assert_eq!(resp.items[0].value, "value_001");
        assert!(!resp.has_more);
        assert_eq!(resp.page, 1);
        assert_eq!(resp.page_size, 20);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/environment_variables/query"
        );
    }
}
