//! 取消用户迁移
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/cancel>
//!
//! 文档核对：`POST /open-apis/security_and_compliance/v1/user_migrations/cancel?user_id_type={...}`，
//! body 为 `{"user_ids": [...]}`（必填，1~100 个）；查询参数 `user_id_type` 必填（默认 `open_id`）。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 取消用户迁移请求 body。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelUserMigrationBody {
    /// 取消迁移用户 ID 列表（1~100 个，ID 类型须与 `user_id_type` 一致）。
    pub user_ids: Vec<String>,
}

/// 取消用户迁移请求。
#[derive(Debug)]
pub struct CancelUserMigrationRequest {
    /// 配置信息。
    config: Config,
    /// 请求 body。
    body: CancelUserMigrationBody,
    /// 用户 ID 类型（查询参数，必填；可选 `open_id`/`union_id`/`user_id`，默认 `open_id`）。
    user_id_type: String,
}

impl CancelUserMigrationRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: CancelUserMigrationBody {
                user_ids: Vec::new(),
            },
            user_id_type: "open_id".to_string(),
        }
    }

    /// 设置取消迁移用户 ID 列表。
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.body.user_ids = user_ids;
        self
    }

    /// 设置用户 ID 类型（默认 `open_id`）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required_list!(
            self.body.user_ids,
            100,
            "user_ids 不能为空且不能超过 100 个"
        );

        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/security_and_compliance/v1/user_migrations/cancel")
                .query("user_id_type", &self.user_id_type)
                .body(
                    serde_json::to_value(&self.body).map_err(|e| {
                        validation_error("取消用户迁移", format!("序列化失败: {e}"))
                    })?,
                )
                .with_supported_access_token_types(vec![
                    AccessTokenType::Tenant,
                    AccessTokenType::User,
                ]);

        Transport::request_typed(req, &self.config, Some(option), "取消用户迁移").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    fn test_config() -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build()
    }

    #[tokio::test]
    async fn test_cancel_user_migration_rejects_empty_user_ids() {
        let req = CancelUserMigrationRequest::new(test_config());
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("user_ids"));
    }

    /// 端到端：POST .../user_migrations/cancel + body {user_ids} + 查询参数 user_id_type + 响应解析。
    #[tokio::test]
    async fn test_cancel_user_migration_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/security_and_compliance/v1/user_migrations/cancel",
            ))
            .and(query_param("user_id_type", "user_id"))
            .and(body_json(json!({ "user_ids": ["7064408494200487xxx"] })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = CancelUserMigrationRequest::new(config)
            .user_ids(vec!["7064408494200487xxx".to_string()])
            .user_id_type("user_id")
            .execute()
            .await
            .expect("取消用户迁移应成功");
        assert_eq!(data, json!({}));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("user_id_type=user_id"));
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["user_ids"].as_array().unwrap().len(), 1);
    }

    /// 默认 user_id_type=open_id：未显式设置时查询参数应带默认值。
    #[tokio::test]
    async fn test_cancel_user_migration_defaults_user_id_type() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/security_and_compliance/v1/user_migrations/cancel",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        CancelUserMigrationRequest::new(config)
            .user_ids(vec!["ou_123".to_string()])
            .execute()
            .await
            .expect("取消用户迁移应成功");

        let received = server.received_requests().await.unwrap_or_default();
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("user_id_type=open_id"));
    }
}
