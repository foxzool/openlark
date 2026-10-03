//! 更新自动化流程状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-workflow/update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
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

/// 更新自动化流程状态请求体。
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
///
/// 用于更新多维表格中的自动化流程状态。
pub struct UpdateWorkflowRequest {
    config: Config,
    app_token: String,
    workflow_id: String,
    status: WorkflowStatus,
}

impl UpdateWorkflowRequest {
    /// 创建新的自动化流程更新请求。
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

    /// 使用指定请求选项执行请求。
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

        let api_request: ApiRequest<UpdateWorkflowResponse> = api_endpoint.to_request().body(body);

        Transport::request_typed(
            api_request,
            &self.config,
            Some(option),
            "更新自动化流程状态",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PUT .../workflows/{workflow_id} → UpdateWorkflowResponse。
    #[tokio::test]
    async fn test_update_workflow_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/bitable/v1/apps/app001/workflows/wf001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        UpdateWorkflowRequest::new(config)
            .app_token("app001")
            .workflow_id("wf001")
            .execute()
            .await
            .expect("更新工作流应成功");
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/bitable/v1/apps/app001/workflows/wf001"
        );
    }
}
