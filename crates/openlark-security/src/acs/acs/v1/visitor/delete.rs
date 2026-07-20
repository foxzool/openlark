//! 删除访客
//!
//! docPath: <https://open.feishu.cn/document/acs-v1/visitor/delete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption, validate_required,
};

/// 删除访客请求
#[derive(Debug)]
pub struct DeleteVisitorRequest {
    /// 配置信息。
    config: Config,
    /// 访客 ID（路径参数，必填）。
    visitor_id: String,
}

impl DeleteVisitorRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, visitor_id: impl Into<String>) -> Self {
        Self {
            config,
            visitor_id: visitor_id.into(),
        }
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.visitor_id, "visitor_id 不能为空");

        let path = format!("/open-apis/acs/v1/visitors/{}", self.visitor_id);
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::delete(&path).with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build()
    }

    #[tokio::test]
    async fn test_delete_visitor_rejects_empty_id() {
        let req = DeleteVisitorRequest::new(test_config(), "");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("visitor_id"));
    }

    /// 端到端：DELETE .../visitors/{id} + 响应解析。
    #[tokio::test]
    async fn test_delete_visitor_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/acs/v1/visitors/visitor_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "visitor_id": "visitor_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = DeleteVisitorRequest::new(config, "visitor_001")
            .execute()
            .await
            .expect("删除访客应成功");
        assert_eq!(data["visitor_id"], "visitor_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/acs/v1/visitors/visitor_001"
        );
    }
}
