//! 取消用户迁移
//!
//! docPath: https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/cancel

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption,
};

/// 取消用户迁移请求。
#[derive(Debug)]
pub struct CancelUserMigrationRequest {
    /// 配置信息。
    config: Config,
    /// 用户迁移任务 ID。
    migration_id: String,
}

impl CancelUserMigrationRequest {
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
        let body = serde_json::json!({ "migration_id": self.migration_id });
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/security_and_compliance/v1/user_migrations/cancel")
                .body(body)
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("取消用户迁移", "响应数据为空"))
    }
}
