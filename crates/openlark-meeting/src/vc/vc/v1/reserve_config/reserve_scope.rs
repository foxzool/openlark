//! 查询会议室预定限制
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/reserve_scope>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询会议室预定限制请求
pub struct GetReserveScopeRequest {
    config: Config,
    /// 查询参数
    query: GetReserveScopeQuery,
}

/// 查询会议室预定限制查询参数
///
/// scope_id/scope_type 必填；user_id_type 可选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReserveScopeQuery {
    /// 会议室或层级 ID
    pub scope_id: String,
    /// 1 代表层级，2 代表会议室
    pub scope_type: String,
    /// 用户 ID 类型：open_id / union_id / user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetReserveScopeRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: GetReserveScopeQuery {
                scope_id: String::new(),
                scope_type: String::new(),
                user_id_type: None,
            },
        }
    }

    /// 设置会议室或层级 ID
    pub fn scope_id(mut self, scope_id: impl Into<String>) -> Self {
        self.query.scope_id = scope_id.into();
        self
    }

    /// 设置范围类型：1 代表层级，2 代表会议室
    pub fn scope_type(mut self, scope_type: impl Into<String>) -> Self {
        self.query.scope_type = scope_type.into();
        self
    }

    /// 设置用户 ID 类型：open_id / union_id / user_id
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.query.user_id_type = Some(user_id_type.into());
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
        validate_required!(self.query.scope_id, "scope_id 不能为空");
        validate_required!(self.query.scope_type, "scope_type 不能为空");

        // url: GET:/open-apis/vc/v1/reserve_configs/reserve_scope
        let q = self.query;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/vc/v1/reserve_configs/reserve_scope")
                .query("scope_id", q.scope_id)
                .query("scope_type", q.scope_type)
                .query_opt("user_id_type", q.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "查询会议室预定限制").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../vc/v1/reserve_configs/reserve_scope → serde_json::Value 解析（单层 data 信封）+ 官方必填查询参数。
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
                "data": {
                    "approve_config": {
                        "approval_switch": 1,
                        "approval_condition": 1,
                        "meeting_duration": 3,
                        "approvers": [ { "user_id": "ou_a27b0xxxc98f856" } ]
                    },
                    "time_config": { "time_switch": 1, "days_in_advance": 30, "max_duration": 24 },
                    "reserve_scope_config": { "if_cover_child_scope": true, "allow_all_users": 0 }
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

        let resp = GetReserveScopeRequest::new(config)
            .scope_id("omm_3c5dxxxbd1a771")
            .scope_type("2")
            .execute()
            .await
            .expect("查询会议室预定限制应成功");
        assert_eq!(resp["approve_config"]["approval_switch"], 1);
        assert_eq!(resp["reserve_scope_config"]["allow_all_users"], 0);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/reserve_configs/reserve_scope"
        );
        // 官方必填查询参数：scope_id/scope_type
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("scope_id=omm_3c5dxxxbd1a771"));
        assert!(query.contains("scope_type=2"));
    }
}
