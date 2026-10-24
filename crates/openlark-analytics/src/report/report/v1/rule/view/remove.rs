//! 移除规则看板
//! docPath: <https://open.feishu.cn/document/server-docs/report-v1/rule-view/remove>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 移除规则看板请求体。
///
/// `user_ids` 可选：列表为空（或不传）时删除规则下全部用户看板；
/// 非空时删除指定用户看板，最多 200 个。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveReportRuleViewBody {
    /// 用户 ID 列表（长度 0～200）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

/// 移除规则看板请求。
#[derive(Debug, Clone)]
pub struct RemoveReportRuleViewRequest {
    config: Arc<Config>,
    rule_id: String,
    body: RemoveReportRuleViewBody,
    /// 用户 ID 类型（默认 open_id）。
    user_id_type: Option<String>,
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
            body: RemoveReportRuleViewBody::default(),
            user_id_type: None,
        }
    }

    /// 设置要移除看板的用户 ID 列表（最多 200 个）。
    ///
    /// 不设置（或传空列表）时删除规则下全部用户看板。
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.body.user_ids = Some(user_ids);
        self
    }

    /// 设置用户 ID 类型（默认 open_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
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
        if let Some(user_ids) = &self.body.user_ids
            && user_ids.len() > 200
        {
            return Err(validation_error(
                "user_ids",
                "user_ids 不能为空且不能超过 200 个",
            ));
        }
        let path = format!("/open-apis/report/v1/rules/{}/views/remove", self.rule_id);
        let mut req: ApiRequest<RemoveReportRuleViewResponse> =
            ApiRequest::post(&path).body(serde_json::to_value(&self.body)?);
        if let Some(ref user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type);
        }

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

    /// 端到端：POST .../rules/{rule_id}/views/remove + body {user_ids} + query user_id_type → 响应解析。
    #[tokio::test]
    async fn test_remove_report_rule_view_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/report/v1/rules/rule_001/views/remove"))
            .and(wiremock::matchers::body_json(json!({
                "user_ids": ["ou_d6a5b5a55c77ca0b5b6c6ca0dd628c85"]
            })))
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

        let resp = RemoveReportRuleViewRequest::new(config, "rule_001")
            .user_ids(vec!["ou_d6a5b5a55c77ca0b5b6c6ca0dd628c85".to_string()])
            .user_id_type("open_id")
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
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(
            sent["user_ids"],
            json!(["ou_d6a5b5a55c77ca0b5b6c6ca0dd628c85"])
        );
    }

    /// user_ids 超过 200 个时应被校验拦截，不发请求。
    #[tokio::test]
    async fn test_remove_report_rule_view_rejects_oversized_user_ids() {
        let server = MockServer::start().await;
        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let result = RemoveReportRuleViewRequest::new(config, "rule_001")
            .user_ids(vec!["ou_x".to_string(); 201])
            .execute()
            .await;
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
