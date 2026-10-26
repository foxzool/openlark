//! 获取单个用户迁移状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/get>
//!
//! 文档核对：`GET /open-apis/security_and_compliance/v1/user_migrations/:user_id?user_id_type={...}`，
//! 路径参数为 `user_id`（用户 ID，非迁移任务 ID）；查询参数 `user_id_type` 必填（默认 `open_id`）。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption, validate_required,
};

/// 获取单个用户迁移状态请求。
#[derive(Debug)]
pub struct GetUserMigrationRequest {
    /// 配置信息。
    config: Config,
    /// 用户 ID（路径参数，ID 类型须与 `user_id_type` 一致）。
    user_id: String,
    /// 用户 ID 类型（查询参数，必填；可选 `open_id`/`union_id`/`user_id`，默认 `open_id`）。
    user_id_type: String,
}

impl GetUserMigrationRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, user_id: impl Into<String>) -> Self {
        Self {
            config,
            user_id: user_id.into(),
            user_id_type: "open_id".to_string(),
        }
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
        validate_required!(self.user_id, "user_id 不能为空");

        let req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "/open-apis/security_and_compliance/v1/user_migrations/{}",
            self.user_id
        ))
        .query("user_id_type", &self.user_id_type)
        .with_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        Transport::request_typed(req, &self.config, Some(option), "获取单个用户迁移状态").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    fn test_config() -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build()
    }

    #[tokio::test]
    async fn test_get_user_migration_rejects_empty_user_id() {
        let req = GetUserMigrationRequest::new(test_config(), "");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("user_id"));
    }

    /// 端到端：GET .../user_migrations/{user_id} + 查询参数 user_id_type + 响应解析。
    #[tokio::test]
    async fn test_get_user_migration_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/security_and_compliance/v1/user_migrations/ou_1234567890abcdef1234567890abcdef",
            ))
            .and(query_param("user_id_type", "open_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "user_migration": {
                        "user_id": "ou_1234567890abcdef1234567890abcdef",
                        "dest_geo": "us",
                        "task_id": "task-id-360043495213",
                        "status": "1",
                        "progress": 100
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

        let data = GetUserMigrationRequest::new(config, "ou_1234567890abcdef1234567890abcdef")
            .execute()
            .await
            .expect("获取用户迁移状态应成功");
        assert_eq!(data["user_migration"]["status"], "1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/security_and_compliance/v1/user_migrations/ou_1234567890abcdef1234567890abcdef"
        );
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("user_id_type=open_id"));
    }
}
