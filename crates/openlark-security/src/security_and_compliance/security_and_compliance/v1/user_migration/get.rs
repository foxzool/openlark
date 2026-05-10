//! 获取单个用户迁移状态
//! docPath: https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/get

use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct UserMigrationGetResponse { pub status: Option<String> }
impl ApiResponseTrait for UserMigrationGetResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
#[derive(Debug, Clone)] pub struct UserMigrationGetRequest { config: Arc<Config>, user_id: String }
impl UserMigrationGetRequest { pub fn new(config: Arc<Config>, user_id: impl Into<String>) -> Self { Self { config, user_id: user_id.into() } } pub async fn execute(self) -> SDKResult<UserMigrationGetResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<UserMigrationGetResponse> { let path = format!("/open-apis/security_and_compliance/v1/user_migrations/{}", self.user_id); let request = ApiRequest::<UserMigrationGetResponse>::get(&path); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("获取单个用户迁移状态", "响应数据为空")) } }
