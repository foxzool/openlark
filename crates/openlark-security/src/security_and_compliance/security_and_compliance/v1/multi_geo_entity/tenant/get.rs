//! 获取地理位置列表
//! docPath: https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/get-2

use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct TenantMultiGeoGetResponse { pub items: Vec<serde_json::Value> }
impl ApiResponseTrait for TenantMultiGeoGetResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
#[derive(Debug, Clone)] pub struct TenantMultiGeoGetRequest { config: Arc<Config> }
impl TenantMultiGeoGetRequest { pub fn new(config: Arc<Config>) -> Self { Self { config } } pub async fn execute(self) -> SDKResult<TenantMultiGeoGetResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<TenantMultiGeoGetResponse> { let request = ApiRequest::<TenantMultiGeoGetResponse>::get("/open-apis/security_and_compliance/v1/multi_geo_entity/tenant"); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("获取地理位置列表", "响应数据为空")) } }
