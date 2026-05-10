//! 迁移用户
//! docPath: https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/create

use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct UserMigrationCreateBody { pub user_id: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct UserMigrationCreateResponse { pub migration_id: Option<String> }
impl ApiResponseTrait for UserMigrationCreateResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
#[derive(Debug, Clone)] pub struct UserMigrationCreateRequest { config: Arc<Config>, body: UserMigrationCreateBody }
impl UserMigrationCreateRequest { pub fn new(config: Arc<Config>) -> Self { Self { config, body: UserMigrationCreateBody::default() } } pub fn body(mut self, body: UserMigrationCreateBody) -> Self { self.body = body; self } pub async fn execute(self) -> SDKResult<UserMigrationCreateResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<UserMigrationCreateResponse> { let request = ApiRequest::<UserMigrationCreateResponse>::post("/open-apis/security_and_compliance/v1/user_migrations").body(serde_json::to_value(&self.body)?); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("迁移用户", "响应数据为空")) } }
