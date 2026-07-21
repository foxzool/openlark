//! 移除规则看板
//! docPath: <https://open.feishu.cn/document/server-docs/report-v1/rule-view/remove>

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

/// 移除规则看板请求。
#[derive(Debug, Clone)]
pub struct RemoveReportRuleViewRequest {
    config: Arc<Config>,
    rule_id: String,
}

/// 移除规则看板响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveReportRuleViewResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for RemoveReportRuleViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl RemoveReportRuleViewRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, rule_id: impl Into<String>) -> Self {
        Self {
            config,
            rule_id: rule_id.into(),
        }
    }

    /// 执行移除规则看板请求。
    pub async fn execute(self) -> SDKResult<RemoveReportRuleViewResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行移除规则看板请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RemoveReportRuleViewResponse> {
        validate_required!(self.rule_id, "rule_id 不能为空");
        let path = format!("/open-apis/report/v1/rules/{}/views/remove", self.rule_id);
        let req: ApiRequest<RemoveReportRuleViewResponse> = ApiRequest::post(&path);

        Transport::request_typed(req, &self.config, Some(option), "移除规则看板").await
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

    /// 端到端：POST .../rules/{rule_id}/views/remove（修复后路径插值）→ 响应解析。
    #[tokio::test]
    async fn test_remove_report_rule_view_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/report/v1/rules/rule_001/views/remove"))
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

        let resp = RemoveReportRuleViewRequest::new(config, "rule_001")
            .execute()
            .await
            .expect("移除规则看板应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/report/v1/rules/rule_001/views/remove"
        );
    }
}
