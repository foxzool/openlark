//! 查询会议室配置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::endpoints::VC_V1_SCOPE_CONFIG;

/// 查询会议室配置请求
pub struct GetScopeConfigRequest {
    config: Config,
    /// 查询参数
    query: GetScopeConfigQuery,
}

/// 查询会议室配置查询参数
///
/// scope_type/scope_id 必填；user_id_type 可选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScopeConfigQuery {
    /// 查询节点范围：1 会议室层级；2 会议室
    pub scope_type: i32,
    /// 查询节点 ID（scope_type 为 1 时是层级 ID，为 2 时是会议室 ID）
    pub scope_id: String,
    /// 用户 ID 类型：open_id / union_id / user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetScopeConfigRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: GetScopeConfigQuery {
                scope_type: 0,
                scope_id: String::new(),
                user_id_type: None,
            },
        }
    }

    /// 设置查询节点范围：1 会议室层级；2 会议室
    pub fn scope_type(mut self, scope_type: i32) -> Self {
        self.query.scope_type = scope_type;
        self
    }

    /// 设置查询节点 ID（scope_type 为 1 时是层级 ID，为 2 时是会议室 ID）
    pub fn scope_id(mut self, scope_id: impl Into<String>) -> Self {
        self.query.scope_id = scope_id.into();
        self
    }

    /// 设置用户 ID 类型：open_id / union_id / user_id
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.query.user_id_type = Some(user_id_type.into());
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
        validate_required!(self.query.scope_id, "scope_id 不能为空");

        // url: GET:/open-apis/vc/v1/scope_config
        let q = self.query;
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(VC_V1_SCOPE_CONFIG)
            .query("scope_type", q.scope_type.to_string())
            .query("scope_id", q.scope_id)
            .query_opt("user_id_type", q.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "查询会议室配置").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../scope_config → data 信封解析 + 官方必填查询参数。
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
                "data": {
                    "current_config": {
                        "scope_type": 1,
                        "scope_id": "omm_608d34d82d531b27fa993902d350a307",
                        "scope_config": {
                            "room_status": { "status": true }
                        }
                    },
                    "origin_configs": []
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

        let resp = GetScopeConfigRequest::new(config)
            .scope_type(1)
            .scope_id("omm_608d34d82d531b27fa993902d350a307")
            .execute()
            .await
            .expect("查询会议室配置应成功");
        assert_eq!(
            resp["current_config"]["scope_id"],
            "omm_608d34d82d531b27fa993902d350a307"
        );
        assert_eq!(
            resp["current_config"]["scope_config"]["room_status"]["status"],
            true
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/scope_config");
        // 官方必填查询参数：scope_type/scope_id
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("scope_type=1"));
        assert!(query.contains("scope_id=omm_608d34d82d531b27fa993902d350a307"));
    }
}
