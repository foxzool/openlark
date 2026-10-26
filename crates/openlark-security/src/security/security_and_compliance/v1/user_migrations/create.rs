//! 创建用户迁移
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/create>
//!
//! 文档核对：`POST /open-apis/security_and_compliance/v1/user_migrations?user_id_type={...}`，
//! body 为 `{"user_ids": [...], "dest_geo": "..."}`（均必填）；查询参数 `user_id_type`
//! 必填（默认 `open_id`）。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 创建用户迁移请求 body。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserMigrationBody {
    /// 迁移用户 ID 列表（1~100 个，ID 类型须与 `user_id_type` 一致）。
    pub user_ids: Vec<String>,
    /// 迁移目标地理位置区域（2~10 字符，如 `sg`）。
    pub dest_geo: String,
}

/// 创建用户迁移请求。
#[derive(Debug)]
pub struct CreateUserMigrationRequest {
    /// 配置信息。
    config: Config,
    /// 请求 body。
    body: CreateUserMigrationBody,
    /// 用户 ID 类型（查询参数，必填；可选 `open_id`/`union_id`/`user_id`，默认 `open_id`）。
    user_id_type: String,
}

impl CreateUserMigrationRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: CreateUserMigrationBody {
                user_ids: Vec::new(),
                dest_geo: String::new(),
            },
            user_id_type: "open_id".to_string(),
        }
    }

    /// 设置迁移用户 ID 列表。
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.body.user_ids = user_ids;
        self
    }

    /// 设置迁移目标地理位置区域（如 `sg`）。
    pub fn dest_geo(mut self, dest_geo: impl Into<String>) -> Self {
        self.body.dest_geo = dest_geo.into();
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
        validate_required!(self.body.dest_geo, "dest_geo 不能为空");

        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/security_and_compliance/v1/user_migrations")
                .query("user_id_type", &self.user_id_type)
                .body(
                    serde_json::to_value(&self.body).map_err(|e| {
                        validation_error("创建用户迁移", format!("序列化失败: {e}"))
                    })?,
                )
                .with_supported_access_token_types(vec![
                    AccessTokenType::Tenant,
                    AccessTokenType::User,
                ]);

        Transport::request_typed(req, &self.config, Some(option), "创建用户迁移").await
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
    async fn test_create_user_migration_rejects_empty_user_ids() {
        let req = CreateUserMigrationRequest::new(test_config()).dest_geo("sg");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("user_ids"));
    }

    #[tokio::test]
    async fn test_create_user_migration_rejects_empty_dest_geo() {
        let req = CreateUserMigrationRequest::new(test_config())
            .user_ids(vec!["7064408494200487938".to_string()]);
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("dest_geo"));
    }

    /// 端到端：POST .../user_migrations + body {user_ids, dest_geo} + 查询参数 user_id_type + 响应解析。
    #[tokio::test]
    async fn test_create_user_migration_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/security_and_compliance/v1/user_migrations"))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "user_ids": ["7064408494200487938"],
                "dest_geo": "sg"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "user_migrations": [
                        { "user_id": "123165707875", "dest_geo": "sg", "task_id": "task_123", "status": "0", "progress": 20 }
                    ]
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

        let data = CreateUserMigrationRequest::new(config)
            .user_ids(vec!["7064408494200487938".to_string()])
            .dest_geo("sg")
            .execute()
            .await
            .expect("创建用户迁移应成功");
        assert_eq!(data["user_migrations"].as_array().unwrap().len(), 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("user_id_type=open_id"));
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["dest_geo"], "sg");
        assert_eq!(sent["user_ids"].as_array().unwrap().len(), 1);
    }
}
