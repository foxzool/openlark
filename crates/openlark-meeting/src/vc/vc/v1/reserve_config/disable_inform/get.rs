//! 查询禁用状态变更通知
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/get-4>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::endpoints::VC_V1_RESERVE_CONFIGS;

/// 查询禁用状态变更通知请求
pub struct GetDisableInformRequest {
    config: Config,
    reserve_config_id: String,
    /// 查询参数
    query: GetDisableInformQuery,
}

/// 查询禁用状态变更通知查询参数
///
/// scope_type 必填；user_id_type 可选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDisableInformQuery {
    /// 1 表示层级，2 表示会议室
    pub scope_type: i32,
    /// 用户 ID 类型：open_id / union_id / user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetDisableInformRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_config_id: String::new(),
            query: GetDisableInformQuery {
                scope_type: 0,
                user_id_type: None,
            },
        }
    }

    /// 预定配置 ID（路径参数）
    pub fn reserve_config_id(mut self, reserve_config_id: impl Into<String>) -> Self {
        self.reserve_config_id = reserve_config_id.into();
        self
    }

    /// 设置范围类型：1 表示层级，2 表示会议室
    pub fn scope_type(mut self, scope_type: i32) -> Self {
        self.query.scope_type = scope_type;
        self
    }

    /// 设置用户 ID 类型：open_id / union_id / user_id
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.query.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/scope_config/get-4>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.reserve_config_id, "reserve_config_id 不能为空");

        // url: GET:/open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform
        let q = self.query;
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "{}/{}/disable_inform",
            VC_V1_RESERVE_CONFIGS, self.reserve_config_id
        ))
        .query("scope_type", q.scope_type.to_string())
        .query_opt("user_id_type", q.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "查询禁用状态变更通知").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../vc/v1/reserve_configs/{reserve_config_id}/disable_inform → serde_json::Value 解析（单层 data 信封）+ 官方必填查询参数。
    #[tokio::test]
    async fn test_get_disable_inform_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/vc/v1/reserve_configs/omm_3c5dd7e09bac0c1758fcf9511bd1a771/disable_inform",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "disable_inform": {
                        "if_inform": true,
                        "informed_users": [ { "user_id": "ou_a27b07a9071d90577c0177bcec98f856" } ],
                        "informed_depts": [ { "department_id": "od-47d8b570b0a011e9679a755efcc5f61a" } ]
                    }
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

        let resp = GetDisableInformRequest::new(config)
            .reserve_config_id("omm_3c5dd7e09bac0c1758fcf9511bd1a771")
            .scope_type(2)
            .execute()
            .await
            .expect("查询禁用状态变更通知应成功");
        assert_eq!(resp["disable_inform"]["if_inform"], true);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/reserve_configs/omm_3c5dd7e09bac0c1758fcf9511bd1a771/disable_inform"
        );
        // 官方必填查询参数：scope_type
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("scope_type=2"));
    }
}
