//! 查询环境变量详情
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-environment_variable/get>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-environment_variable/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询环境变量详情 Builder
#[derive(Debug, Clone)]
pub struct EnvironmentVariableGetRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 环境变量 API 名称
    env_var_api_name: String,
}

impl EnvironmentVariableGetRequestBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        env_var_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            env_var_api_name: env_var_api_name.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EnvironmentVariableGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EnvironmentVariableGetResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/environment_variables/{}",
            self.namespace, self.env_var_api_name
        );

        let req: ApiRequest<EnvironmentVariableGetResponse> = ApiRequest::get(&url);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 环境变量详情
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentVariableDetail {
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

/// 查询环境变量详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentVariableGetResponse {
    /// 环境变量详情
    #[serde(rename = "environment_variable")]
    pub environment_variable: EnvironmentVariableDetail,
}

impl ApiResponseTrait for EnvironmentVariableGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to EnvironmentVariableGetRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type EnvironmentVariableGetBuilder = EnvironmentVariableGetRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../environment_variables/{api_name} → EnvironmentVariableGetResponse。
    #[tokio::test]
    async fn test_get_environment_variable_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/environment_variables/env_var_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "environment_variable": {
                        "api_name": "env_var_001",
                        "name": "ENV_NAME",
                        "value": "value_001",
                        "description": "测试变量"
                    }
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

        let resp = EnvironmentVariableGetRequestBuilder::new(config, "ns_test", "env_var_001")
            .execute()
            .await
            .expect("查询环境变量详情应成功");
        assert_eq!(resp.environment_variable.api_name, "env_var_001");
        assert_eq!(resp.environment_variable.name, "ENV_NAME");
        assert_eq!(resp.environment_variable.value, "value_001");
        assert_eq!(
            resp.environment_variable.description.as_deref(),
            Some("测试变量")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/environment_variables/env_var_001"
        );
    }
}
