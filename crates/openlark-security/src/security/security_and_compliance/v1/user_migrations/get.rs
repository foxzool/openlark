//! 获取单个用户迁移状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption,
};

/// 获取单个用户迁移状态请求。
#[derive(Debug)]
pub struct GetUserMigrationRequest {
    /// 配置信息。
    config: Config,
    /// 用户迁移任务 ID。
    migration_id: String,
}

impl GetUserMigrationRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, migration_id: impl Into<String>) -> Self {
        Self {
            config,
            migration_id: migration_id.into(),
        }
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "/open-apis/security_and_compliance/v1/user_migrations/{}",
            self.migration_id
        ))
        .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("获取单个用户迁移状态", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../user_migrations/{id} + 响应解析。
    #[tokio::test]
    async fn test_get_user_migration_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/security_and_compliance/v1/user_migrations/mig_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "migration_id": "mig_001", "status": "processing" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = GetUserMigrationRequest::new(config, "mig_001")
            .execute()
            .await
            .expect("获取用户迁移状态应成功");
        assert_eq!(data["migration_id"], "mig_001");
        assert_eq!(data["status"], "processing");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/security_and_compliance/v1/user_migrations/mig_001"
        );
    }
}
