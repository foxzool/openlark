//! 编辑试用期
//!
//! docPath: <https://open.feishu.cn/document/corehr-v1/probation/edit>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 编辑试用期请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditProbationBody {
    /// 员工 ID。
    pub employee_id: Option<String>,
}
/// 编辑试用的响应。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditProbationResponse {
    /// 试用期。
    pub probation: Option<serde_json::Value>,
}
impl ApiResponseTrait for EditProbationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
/// 编辑试用的请求。
#[derive(Debug, Clone)]
pub struct EditProbationRequest {
    config: Arc<Config>,
    body: EditProbationBody,
}
/// 编辑试用期请求构建器实现。
impl EditProbationRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: EditProbationBody::default(),
        }
    }
    /// 设置请求体。
    pub fn body(mut self, body: EditProbationBody) -> Self {
        self.body = body;
        self
    }
    /// 执行编辑试用期请求。
    pub async fn execute(self) -> SDKResult<EditProbationResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }
    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EditProbationResponse> {
        let request =
            ApiRequest::<EditProbationResponse>::post("/open-apis/corehr/v2/probation/edit")
                .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(request, &self.config, Some(option), "编辑试用期").await
    }
}
