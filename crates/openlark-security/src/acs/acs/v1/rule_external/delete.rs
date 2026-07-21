//! 删除权限组
//!
//! docPath: <https://open.feishu.cn/document/acs-v1/rule_external/delete>
//!
//! 文档核对：`DELETE /open-apis/acs/v1/rule_external?rule_id={rule_id}`，**无 body**。
//! `rule_id` 是查询参数。旧实现错误地发了 JSON body。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption, validate_required,
};

/// 删除权限组请求
#[derive(Debug)]
pub struct DeleteRuleExternalRequest {
    /// 配置信息。
    config: Config,
    /// 权限组 ID（查询参数，必填）。
    rule_id: String,
}

impl DeleteRuleExternalRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, rule_id: impl Into<String>) -> Self {
        Self {
            config,
            rule_id: rule_id.into(),
        }
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.rule_id, "rule_id 不能为空");

        let req: ApiRequest<serde_json::Value> =
            ApiRequest::delete("/open-apis/acs/v1/rule_external")
                .query("rule_id", &self.rule_id)
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        Transport::request_typed(req, &self.config, Some(option), "删除权限组").await
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
    async fn test_delete_rule_external_rejects_empty_id() {
        let req = DeleteRuleExternalRequest::new(test_config(), "  ");
        let result = req.execute_with_options(RequestOption::default()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("rule_id"));
    }

    /// 端到端：DELETE .../rule_external?rule_id= + query 拼装（无 body）。
    #[tokio::test]
    async fn test_delete_rule_external_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/acs/v1/rule_external"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "rule_id": "rule_123" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = DeleteRuleExternalRequest::new(config, "rule_123")
            .execute()
            .await
            .expect("删除权限组应成功");
        assert_eq!(data["rule_id"], "rule_123");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/acs/v1/rule_external");
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("rule_id=rule_123")
        );
    }
}
