//! 批量获取用户迁移状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/search>
//!
//! 过滤条件在 body，调用方按飞书文档自行构造 JSON 透传。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption,
};

/// 批量获取用户迁移状态请求。
///
/// 通过 [`Self::body`] 传入过滤条件（如迁移任务 ID 列表、分页等）。
#[derive(Debug)]
pub struct SearchUserMigrationsRequest {
    /// 配置信息。
    config: Config,
    /// 请求 body（过滤 + 分页条件）。
    body: serde_json::Value,
}

impl SearchUserMigrationsRequest {
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
            ApiRequest::post("/open-apis/security_and_compliance/v1/user_migrations/search")
                .body(self.body)
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("批量获取用户迁移状态", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../user_migrations/search + body 透传 + 响应解析。
    #[tokio::test]
    async fn test_search_user_migrations_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/security_and_compliance/v1/user_migrations/search",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{ "migration_id": "mig_001" }, { "migration_id": "mig_002" }],
                    "has_more": false
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

        let data = SearchUserMigrationsRequest::new(config)
            .body(json!({ "migration_ids": ["mig_001", "mig_002"] }))
            .execute()
            .await
            .expect("批量获取用户迁移状态应成功");
        assert_eq!(data["items"].as_array().unwrap().len(), 2);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["migration_ids"].as_array().unwrap().len(), 2);
    }
}
