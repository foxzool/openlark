//! 查询会议室预定限制
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/reserve_scope>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

/// 查询会议室预定限制请求
pub struct GetReserveScopeRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl GetReserveScopeRequest {
    /// 创建请求实例。
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/reserve_scope>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/vc/v1/reserve_configs/:reserve_config_id/reserve_scope
        // 注意：这个端点需要 reserve_config_id 参数
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/vc/v1/reserve_configs/reserve_scope");
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        Transport::request_typed(req, &self.config, Some(option), "查询会议室预定限制").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../vc/v1/reserve_configs/reserve_scope → serde_json::Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_get_reserve_scope_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/reserve_configs/reserve_scope"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "scope_config_id": "rc_001", "scope": "all" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetReserveScopeRequest::new(config)
            .execute()
            .await
            .expect("查询会议室预定限制应成功");
        assert_eq!(resp["scope_config_id"], "rc_001");
        assert_eq!(resp["scope"], "all");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/reserve_configs/reserve_scope"
        );
    }
}
