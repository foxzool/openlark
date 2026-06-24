//! 编辑试用期
//!
//! docPath: https://open.feishu.cn/document/corehr-v1/probation/edit

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 待补充文档。
/// 待补充文档。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditProbationBody {
    /// 待补充文档。
    pub employee_id: Option<String>,
}
/// 待补充文档。
/// 待补充文档。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditProbationResponse {
    /// 待补充文档。
    pub probation: Option<serde_json::Value>,
}
impl ApiResponseTrait for EditProbationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
/// 待补充文档。
#[derive(Debug, Clone)]
pub struct EditProbationRequest {
    config: Arc<Config>,
    body: EditProbationBody,
}
/// 待补充文档。
/// 待补充文档。
impl EditProbationRequest {
    /// 待补充文档。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: EditProbationBody::default(),
        }
    }
    /// 待补充文档。
    pub fn body(mut self, body: EditProbationBody) -> Self {
        self.body = body;
        self
    }
    /// 待补充文档。
    pub async fn execute(self) -> SDKResult<EditProbationResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }
    /// 待补充文档。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EditProbationResponse> {
        let request =
            ApiRequest::<EditProbationResponse>::post("/open-apis/corehr/v2/probation/edit")
                .body(serde_json::to_value(&self.body)?);
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("编辑试用期", "响应数据为空"))
    }
}
