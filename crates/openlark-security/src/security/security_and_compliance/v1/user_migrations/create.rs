//! 创建用户迁移
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/create>
//!
//! 请求 body 字段较多（源/目标用户、范围等），调用方按飞书文档自行构造 JSON 透传。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption,
};

/// 创建用户迁移请求。
///
/// 通过 [`Self::body`] 传入完整请求体（按飞书文档构造）。
#[derive(Debug)]
pub struct CreateUserMigrationRequest {
    /// 配置信息。
    config: Config,
    /// 请求 body。
    body: serde_json::Value,
}

impl CreateUserMigrationRequest {
    /// 创建新的请求构建器（空 body）。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: serde_json::json!({}),
        }
    }

    /// 设置请求 body（覆盖已有内容）。
    pub fn body(mut self, body: serde_json::Value) -> Self {
        self.body = body;
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/security_and_compliance/v1/user_migrations")
                .body(self.body)
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../user_migrations + body 透传 + 响应解析。
    #[tokio::test]
    async fn test_create_user_migration_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/security_and_compliance/v1/user_migrations",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "migration_id": "mig_new_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = CreateUserMigrationRequest::new(config)
            .body(json!({ "source_user_id": "u_old", "target_user_id": "u_new" }))
            .execute()
            .await
            .expect("创建用户迁移应成功");
        assert_eq!(data["migration_id"], "mig_new_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["source_user_id"], "u_old");
    }
}
