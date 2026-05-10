//! 取消用户迁移
//! docPath: https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/cancel

use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct UserMigrationCancelBody { pub user_id: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct UserMigrationCancelResponse { pub success: Option<bool> }
impl ApiResponseTrait for UserMigrationCancelResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
#[derive(Debug, Clone)] pub struct UserMigrationCancelRequest { config: Arc<Config>, body: UserMigrationCancelBody }
impl UserMigrationCancelRequest { pub fn new(config: Arc<Config>) -> Self { Self { config, body: UserMigrationCancelBody::default() } } pub fn body(mut self, body: UserMigrationCancelBody) -> Self { self.body = body; self } pub async fn execute(self) -> SDKResult<UserMigrationCancelResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<UserMigrationCancelResponse> { let request = ApiRequest::<UserMigrationCancelResponse>::post("/open-apis/security_and_compliance/v1/user_migrations/cancel").body(serde_json::to_value(&self.body)?); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("取消用户迁移", "响应数据为空")) } }
