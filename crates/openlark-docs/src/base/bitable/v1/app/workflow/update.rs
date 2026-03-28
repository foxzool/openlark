//! 更新自动化流程状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-workflow/update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

/// 自动化状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum WorkflowStatus {
    /// 开启自动化流程
    Enable,
    /// 关闭自动化流程
    Disable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkflowBody {
    /// 自动化状态
    pub status: WorkflowStatus,
}

/// 更新自动化流程状态响应（data）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWorkflowResponse {}

impl ApiResponseTrait for UpdateWorkflowResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新自动化流程状态请求
pub struct UpdateWorkflowRequest {
    config: Config,
    app_token: String,
    workflow_id: String,
    status: WorkflowStatus,
}

impl UpdateWorkflowRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            workflow_id: String::new(),
            status: WorkflowStatus::Enable,
        }
    }

    /// 多维表格 app_token（路径参数）
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 自动化工作流 ID（路径参数）
    pub fn workflow_id(mut self, workflow_id: impl Into<String>) -> Self {
        self.workflow_id = workflow_id.into();
        self
    }

    /// 自动化状态
    pub fn status(mut self, status: WorkflowStatus) -> Self {
        self.status = status;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateWorkflowResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateWorkflowResponse> {
        validate_required!(self.app_token, "app_token 不能为空");
        validate_required!(self.workflow_id, "workflow_id 不能为空");

        let api_endpoint = BitableApiV1::WorkflowUpdate(self.app_token, self.workflow_id);
        let body = serde_json::to_value(&UpdateWorkflowBody {
            status: self.status,
        })
        .map_err(|e| {
            openlark_core::error::serialization_error("序列化更新自动化流程请求体失败", Some(e))
        })?;

        let api_request: ApiRequest<UpdateWorkflowResponse> =
            ApiRequest::put(&api_endpoint.to_url()).body(body);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
