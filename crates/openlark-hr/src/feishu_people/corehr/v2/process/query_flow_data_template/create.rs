//! 查询流程数据参数模板
//! docPath: 

use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct QueryFlowDataTemplateBody { pub process_code: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct QueryFlowDataTemplateResponse { pub template: Option<serde_json::Value> }
impl ApiResponseTrait for QueryFlowDataTemplateResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
#[derive(Debug, Clone)] pub struct QueryFlowDataTemplateRequest { config: Arc<Config>, body: QueryFlowDataTemplateBody }
impl QueryFlowDataTemplateRequest { pub fn new(config: Arc<Config>) -> Self { Self { config, body: QueryFlowDataTemplateBody::default() } } pub fn body(mut self, body: QueryFlowDataTemplateBody) -> Self { self.body = body; self } pub async fn execute(self) -> SDKResult<QueryFlowDataTemplateResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<QueryFlowDataTemplateResponse> { let request = ApiRequest::<QueryFlowDataTemplateResponse>::post("/open-apis/corehr/v2/query_flow_data_template").body(serde_json::to_value(&self.body)?); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("查询流程数据参数模板", "响应数据为空")) } }
