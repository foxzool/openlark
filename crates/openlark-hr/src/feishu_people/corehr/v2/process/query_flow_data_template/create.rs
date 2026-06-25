//! 查询流程数据参数模板
//! docPath:

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询流程数据参数模板请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryFlowDataTemplateBody {
    /// 流程编码。
    pub process_code: Option<String>,
}
/// 查询流程数据参数模板响应。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryFlowDataTemplateResponse {
    /// 模板数据。
    pub template: Option<serde_json::Value>,
}
impl ApiResponseTrait for QueryFlowDataTemplateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
/// 查询流程数据参数模板请求。
#[derive(Debug, Clone)]
pub struct QueryFlowDataTemplateRequest {
    /// 配置信息。
    config: Arc<Config>,
    /// 请求体。
    body: QueryFlowDataTemplateBody,
}
impl QueryFlowDataTemplateRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: QueryFlowDataTemplateBody::default(),
        }
    }
    /// 设置请求体。
    pub fn body(mut self, body: QueryFlowDataTemplateBody) -> Self {
        self.body = body;
        self
    }
    /// 执行请求。
    pub async fn execute(self) -> SDKResult<QueryFlowDataTemplateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }
    /// 执行请求（带选项）。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryFlowDataTemplateResponse> {
        let request = ApiRequest::<QueryFlowDataTemplateResponse>::post(
            "/open-apis/corehr/v2/query_flow_data_template",
        )
        .body(serde_json::to_value(&self.body)?);
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询流程数据参数模板", "响应数据为空")
        })
    }
}
