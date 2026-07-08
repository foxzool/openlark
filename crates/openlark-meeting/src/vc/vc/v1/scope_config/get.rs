//! 查询会议室配置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::VC_V1_SCOPE_CONFIG;

/// 查询会议室配置请求
pub struct GetScopeConfigRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl GetScopeConfigRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/get>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/vc/v1/scope_config
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(VC_V1_SCOPE_CONFIG);
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询会议室配置")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../scope_config → data 信封解析。
    #[tokio::test]
    async fn test_get_scope_config_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/scope_config"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "scope_id": "scope_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetScopeConfigRequest::new(config)
            .execute()
            .await
            .expect("查询会议室配置应成功");
        assert_eq!(resp["scope_id"], "scope_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/scope_config");
    }
}
