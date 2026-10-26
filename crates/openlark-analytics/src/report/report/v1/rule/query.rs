//! 查询规则
//! docPath: <https://open.feishu.cn/document/server-docs/report-v1/rule/query>

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

/// 查询规则请求。
///
/// GET 接口，官方请求字段全部为查询参数：`rule_name` 必填，
/// `include_deleted`/`user_id_type` 可选。
#[derive(Debug, Clone)]
pub struct QueryReportRuleRequest {
    config: Arc<Config>,
    /// 规则名称（查询参数，必填）。
    rule_name: String,
    /// 是否包括已删除：0-不包括（默认），1-包括（查询参数，可选）。
    include_deleted: Option<i32>,
    /// 用户 ID 类型（查询参数，默认 open_id）。
    user_id_type: Option<String>,
}

/// 查询规则响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryReportRuleResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for QueryReportRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl QueryReportRuleRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, rule_name: impl Into<String>) -> Self {
        Self {
            config,
            rule_name: rule_name.into(),
            include_deleted: None,
            user_id_type: None,
        }
    }

    /// 设置是否包括已删除：0-不包括（默认），1-包括。
    pub fn include_deleted(mut self, include_deleted: i32) -> Self {
        self.include_deleted = Some(include_deleted);
        self
    }

    /// 设置用户 ID 类型（默认 open_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行查询规则请求。
    pub async fn execute(self) -> SDKResult<QueryReportRuleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行查询规则请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<QueryReportRuleResponse> {
        validate_required!(self.rule_name, "rule_name 不能为空");
        let path = "/open-apis/report/v1/rules/query".to_string();
        let req: ApiRequest<QueryReportRuleResponse> = ApiRequest::get(&path)
            .query("rule_name", &self.rule_name)
            .query_opt(
                "include_deleted",
                self.include_deleted.map(|v| v.to_string()),
            )
            .query_opt("user_id_type", self.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "查询规则").await
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

    /// 端到端：GET /open-apis/report/v1/rules/query?rule_name&include_deleted&user_id_type → 响应解析。
    #[tokio::test]
    async fn test_query_report_rule_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/report/v1/rules/query"))
            .and(wiremock::matchers::query_param("rule_name", "工作月报"))
            .and(wiremock::matchers::query_param("include_deleted", "1"))
            .and(wiremock::matchers::query_param("user_id_type", "open_id"))
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

        let resp = QueryReportRuleRequest::new(config, "工作月报")
            .include_deleted(1)
            .user_id_type("open_id")
            .execute()
            .await
            .expect("查询规则应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/report/v1/rules/query");
        let pairs: Vec<(String, String)> = received[0]
            .url
            .query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        assert!(pairs.contains(&("rule_name".to_string(), "工作月报".to_string())));
        assert!(pairs.contains(&("include_deleted".to_string(), "1".to_string())));
        assert!(pairs.contains(&("user_id_type".to_string(), "open_id".to_string())));
    }

    /// rule_name 为空时应被校验拦截，不发请求。
    #[tokio::test]
    async fn test_query_report_rule_rejects_blank_rule_name() {
        let server = MockServer::start().await;
        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let result = QueryReportRuleRequest::new(config, "  ").execute().await;
        assert!(result.is_err());
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }
}
