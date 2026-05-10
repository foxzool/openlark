//! 批量获取用户迁移状态
//! docPath: https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/search

use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct UserMigrationSearchBody { pub user_ids: Option<Vec<String>> }
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct UserMigrationSearchResponse { pub items: Vec<serde_json::Value> }
impl ApiResponseTrait for UserMigrationSearchResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
#[derive(Debug, Clone)] pub struct UserMigrationSearchRequest { config: Arc<Config>, body: UserMigrationSearchBody }
impl UserMigrationSearchRequest { pub fn new(config: Arc<Config>) -> Self { Self { config, body: UserMigrationSearchBody::default() } } pub fn body(mut self, body: UserMigrationSearchBody) -> Self { self.body = body; self } pub async fn execute(self) -> SDKResult<UserMigrationSearchResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<UserMigrationSearchResponse> { let request = ApiRequest::<UserMigrationSearchResponse>::post("/open-apis/security_and_compliance/v1/user_migrations/search").body(serde_json::to_value(&self.body)?); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("批量获取用户迁移状态", "响应数据为空")) } }
