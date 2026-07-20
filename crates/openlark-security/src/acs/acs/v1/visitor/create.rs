//! 添加访客
//!
//! docPath: <https://open.feishu.cn/document/acs-v1/visitor/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption,
};

/// 添加访客请求
///
/// 通过 body 传入访客信息。本计划范围只做 Transport 迁移，body 用 `serde_json::Value`
/// 透传（字段细化见 spec §9，不在本次范围）。
#[derive(Debug)]
pub struct CreateVisitorRequest {
    /// 配置信息。
    config: Config,
    /// 请求体（必填，调用方自行构造 JSON）。
    body: Option<serde_json::Value>,
}

impl CreateVisitorRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
    pub fn body(mut self, body: serde_json::Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let body = self
            .body
            .ok_or_else(|| validation_error("添加访客", "body 不能为空"))?;

        let req: ApiRequest<serde_json::Value> = ApiRequest::post("/open-apis/acs/v1/visitors")
            .body(body)
            .with_supported_access_token_types(vec![AccessTokenType::App]);

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
    async fn test_create_visitor_rejects_empty_body() {
        let req = CreateVisitorRequest::new(test_config());
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("body"));
    }

    /// 端到端：POST .../visitors + body 透传 + 响应解析。
    #[tokio::test]
    async fn test_create_visitor_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/acs/v1/visitors"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "visitor_id": "visitor_new_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = CreateVisitorRequest::new(config)
            .body(json!({ "name": "访客甲", "phone": "13800000000" }))
            .execute()
            .await
            .expect("添加访客应成功");
        assert_eq!(data["visitor_id"], "visitor_new_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["name"], "访客甲");
    }
}
